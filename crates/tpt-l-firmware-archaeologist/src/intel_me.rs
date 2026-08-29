//! Intel Management Engine (ME) flash region parsing: the Flash Partition Table
//! (FPT) and its partition entries.

use crate::bindings::Reader;
use crate::error::{ParseError, Result};

/// FPT signature: `"_FPT"`.
pub const FPT_SIGNATURE: u32 = 0x5F46_5054;

/// Flash Partition Table header.
#[derive(Debug, Clone)]
pub struct FptHeader {
    pub num_entries: u32,
    pub header_version: u16,
    pub entry_version: u16,
    pub header_length: u16,
    pub header_checksum: u8,
    pub raw: [u8; 28],
}

impl FptHeader {
    pub const LEN: usize = 28;

    pub fn parse(buf: &[u8], offset: usize) -> Result<Self> {
        let r = Reader::new(buf);
        if r.remaining(offset) < Self::LEN {
            return Err(ParseError::too_short(offset));
        }
        let sig = r.u32(offset).unwrap();
        if sig != FPT_SIGNATURE {
            return Err(ParseError::bad_signature(
                offset,
                FPT_SIGNATURE.to_le_bytes(),
                sig.to_le_bytes(),
            ));
        }
        let num_entries = r.u32(offset + 4).unwrap();
        let header_version = r.u16(offset + 8).unwrap();
        let entry_version = r.u16(offset + 10).unwrap();
        let header_length = r.u16(offset + 12).unwrap();
        let header_checksum = r.u8(offset + 14).unwrap();
        let mut raw = [0u8; 28];
        raw.copy_from_slice(&buf[offset..offset + 28]);
        Ok(FptHeader {
            num_entries,
            header_version,
            entry_version,
            header_length,
            header_checksum,
            raw,
        })
    }

    /// Parse the partition entries following the header.
    pub fn entries(&self, buf: &[u8], offset: usize) -> Result<Vec<FptEntry>> {
        let mut out = Vec::with_capacity(self.num_entries as usize);
        let mut pos = offset + self.header_length as usize;
        for _ in 0..self.num_entries {
            out.push(FptEntry::parse(buf, pos)?);
            pos += FptEntry::LEN;
        }
        Ok(out)
    }
}

/// A single FPT partition entry (32 bytes).
#[derive(Debug, Clone)]
pub struct FptEntry {
    pub name: String,
    pub offset: u32,
    pub length: u32,
    pub attributes: u32,
}

impl FptEntry {
    pub const LEN: usize = 32;

    pub fn parse(buf: &[u8], offset: usize) -> Result<Self> {
        let r = Reader::new(buf);
        if r.remaining(offset) < Self::LEN {
            return Err(ParseError::too_short(offset));
        }
        let name_raw = &buf[offset..offset + 4];
        let name = core::str::from_utf8(name_raw)
            .map(|s| s.trim_end_matches('\0'))
            .unwrap_or("")
            .to_string();
        let _reserved1 = r.u32(offset + 4).unwrap();
        let offset_val = r.u32(offset + 8).unwrap();
        let length = r.u32(offset + 12).unwrap();
        let _reserved2 = r.u32(offset + 16).unwrap();
        let _reserved3 = r.u32(offset + 20).unwrap();
        let attributes = r.u32(offset + 24).unwrap();
        let name = if name.is_empty() {
            format!("{:08X}", u32::from_le_bytes(*<&[u8; 4]>::try_from(name_raw).unwrap()))
        } else {
            name
        };
        Ok(FptEntry { name, offset: offset_val, length, attributes })
    }
}
