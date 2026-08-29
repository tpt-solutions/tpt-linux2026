//! UEFI firmware volume and PE/COFF image parsing (zero-copy, safe API).

use crate::bindings::Reader;
use crate::error::{ParseError, Result};

/// Firmware Volume signature: `"_FVH"`.
pub const FV_SIGNATURE: u32 = 0x4856_465F;
/// DOS executable magic: `"MZ"`.
pub const DOS_MAGIC: u16 = 0x5A4D;
/// PE signature: `"PE\0\0"`.
pub const PE_SIGNATURE: u32 = 0x0000_4550;

/// EFI Firmware Volume header (EFI_FIRMWARE_VOLUME_HEADER).
#[derive(Debug, Clone)]
pub struct FvHeader {
    pub fv_length: u64,
    pub signature: u32,
    pub attributes: u32,
    pub header_length: u16,
    pub checksum: u16,
    pub revision: u8,
    pub ext_header_offset: u16,
    pub raw: [u8; 52],
}

impl FvHeader {
    pub const LEN: usize = 56;

    pub fn parse(buf: &[u8], offset: usize) -> Result<Self> {
        let r = Reader::new(buf);
        if r.remaining(offset) < Self::LEN {
            return Err(ParseError::too_short(offset));
        }
        let fv_length = r.u64(offset + 32).unwrap();
        let signature = r.u32(offset + 40).unwrap();
        if signature != FV_SIGNATURE {
            return Err(ParseError::bad_signature(
                offset + 40,
                FV_SIGNATURE.to_le_bytes(),
                signature.to_le_bytes(),
            ));
        }
        let attributes = r.u32(offset + 44).unwrap();
        let header_length = r.u16(offset + 48).unwrap();
        let checksum = r.u16(offset + 50).unwrap();
        let ext_header_offset = r.u16(offset + 52).unwrap();
        let revision = r.u8(offset + 55).unwrap();
        let mut raw = [0u8; 52];
        raw.copy_from_slice(&buf[offset..offset + 52]);
        Ok(FvHeader {
            fv_length,
            signature,
            attributes,
            header_length,
            checksum,
            revision,
            ext_header_offset,
            raw,
        })
    }

    /// Iterate the FFS file headers contained in this volume (best-effort).
    pub fn files<'a>(&self, buf: &'a [u8]) -> FileIter<'a> {
        let start = self.header_length as usize;
        FileIter { buf, offset: start, end: (self.fv_length as usize).min(buf.len()) }
    }
}

/// An FFS file header (EFI_FFS_FILE_HEADER), 24 bytes.
#[derive(Debug, Clone)]
pub struct FfsFile {
    pub name: [u8; 16],
    pub file_type: u8,
    pub attributes: u8,
    pub size: u32,
    pub state: u8,
}

impl FfsFile {
    pub const LEN: usize = 24;

    pub fn parse(buf: &[u8], offset: usize) -> Result<Self> {
        let r = Reader::new(buf);
        if r.remaining(offset) < Self::LEN {
            return Err(ParseError::too_short(offset));
        }
        let mut name = [0u8; 16];
        name.copy_from_slice(&buf[offset..offset + 16]);
        // bytes 16..18 are the integrity check (header + file checksum).
        let file_type = r.u8(offset + 18).unwrap();
        let attributes = r.u8(offset + 19).unwrap();
        let b0 = r.u8(offset + 20).unwrap() as u32;
        let b1 = r.u8(offset + 21).unwrap() as u32;
        let b2 = r.u8(offset + 22).unwrap() as u32;
        let size = b0 | (b1 << 8) | (b2 << 16);
        let state = r.u8(offset + 23).unwrap();
        Ok(FfsFile { name, file_type, attributes, size, state })
    }
}

/// Iterator over FFS files in a firmware volume.
pub struct FileIter<'a> {
    buf: &'a [u8],
    offset: usize,
    end: usize,
}

#[allow(clippy::needless_lifetimes)]
impl<'a> Iterator for FileIter<'a> {
    type Item = FfsFile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset + FfsFile::LEN <= self.end {
            let file = FfsFile::parse(self.buf, self.offset).ok()?;
            if file.size == 0 {
                return None;
            }
            self.offset += file.size as usize;
            // Align to 8 bytes per FFS rules.
            self.offset = (self.offset + 7) & !7;
            return Some(file);
        }
        None
    }
}

/// PE/COFF image machine types we care about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Machine {
    X64,
    Aarch64,
    Ia32,
    Unknown(u16),
}

impl Machine {
    fn from_raw(m: u16) -> Self {
        match m {
            0x8664 => Machine::X64,
            0xAA64 => Machine::Aarch64,
            0x014C => Machine::Ia32,
            other => Machine::Unknown(other),
        }
    }
}

/// A parsed PE/COFF image header (DOS + COFF + optional header essentials).
#[derive(Debug, Clone)]
pub struct PeImage {
    pub machine: Machine,
    pub number_of_sections: u16,
    pub entry_point: u32,
    pub is_pe32_plus: bool,
}

impl PeImage {
    pub fn parse(buf: &[u8]) -> Result<Self> {
        let r = Reader::new(buf);
        let dos_magic = r.u16(0).ok_or_else(|| ParseError::too_short(0))?;
        if dos_magic != DOS_MAGIC {
            return Err(ParseError::invalid(0, "not a DOS/PE image (missing 'MZ')"));
        }
        let e_lfanew = r.u32(0x3C).ok_or_else(|| ParseError::too_short(0x3C))? as usize;
        let pe_sig = r.u32(e_lfanew).ok_or_else(|| ParseError::too_short(e_lfanew))?;
        if pe_sig != PE_SIGNATURE {
            return Err(ParseError::bad_signature(
                e_lfanew,
                PE_SIGNATURE.to_le_bytes(),
                pe_sig.to_le_bytes(),
            ));
        }
        let coff = e_lfanew + 4;
        let machine = Machine::from_raw(r.u16(coff).ok_or_else(|| ParseError::too_short(coff))?);
        let number_of_sections = r.u16(coff + 2).ok_or_else(|| ParseError::too_short(coff + 2))?;
        let opt_header_offset = coff + 20;
        let magic =
            r.u16(opt_header_offset).ok_or_else(|| ParseError::too_short(opt_header_offset))?;
        let is_pe32_plus = magic == 0x20B;
        // Entry point RVA is at offset 16 within the optional header.
        let entry_point = r
            .u32(opt_header_offset + 16)
            .ok_or_else(|| ParseError::too_short(opt_header_offset + 16))?;
        Ok(PeImage { machine, number_of_sections, entry_point, is_pe32_plus })
    }
}
