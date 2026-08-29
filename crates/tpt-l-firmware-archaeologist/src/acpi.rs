//! ACPI table parsing (DSDT, SSDT, FADT).
//!
//! Only safe [`Reader`](crate::bindings::Reader) access is used here.

use crate::bindings::Reader;
use crate::error::{ParseError, Result};

/// The 36-byte ACPI system description table header (ACPI 6.x).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcpiTableHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: String,
    pub oem_table_id: String,
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

impl AcpiTableHeader {
    pub const LEN: usize = 36;

    pub fn parse(r: &Reader, offset: usize) -> Result<Self> {
        let sig = r.bytes(offset, 4).ok_or_else(|| ParseError::too_short(offset))?;
        let mut signature = [0u8; 4];
        signature.copy_from_slice(sig);

        let length = r.u32(offset + 4).ok_or_else(|| ParseError::too_short(offset + 4))?;
        let revision = r.u8(offset + 8).ok_or_else(|| ParseError::too_short(offset + 8))?;
        let checksum = r.u8(offset + 9).ok_or_else(|| ParseError::too_short(offset + 9))?;
        let oem_id = r.ascii(offset + 10, 6).unwrap_or("").to_string();
        let oem_table_id = r.ascii(offset + 16, 8).unwrap_or("").to_string();
        let oem_revision = r.u32(offset + 24).ok_or_else(|| ParseError::too_short(offset + 24))?;
        let creator_id = r.u32(offset + 28).ok_or_else(|| ParseError::too_short(offset + 28))?;
        let creator_revision =
            r.u32(offset + 32).ok_or_else(|| ParseError::too_short(offset + 32))?;

        Ok(AcpiTableHeader {
            signature,
            length,
            revision,
            checksum,
            oem_id,
            oem_table_id,
            oem_revision,
            creator_id,
            creator_revision,
        })
    }
}

/// A generic ACPI table: a header plus its (AML or data) body.
#[derive(Debug, Clone)]
pub struct AcpiTable<'a> {
    pub header: AcpiTableHeader,
    pub body: &'a [u8],
}

impl<'a> AcpiTable<'a> {
    pub fn parse(buf: &'a [u8], offset: usize) -> Result<Self> {
        let r = Reader::new(buf);
        let header = AcpiTableHeader::parse(&r, offset)?;
        let start = offset + AcpiTableHeader::LEN;
        let end = offset
            .checked_add(header.length as usize)
            .ok_or_else(|| ParseError::too_short(offset))?;
        if end > buf.len() {
            return Err(ParseError::too_short(end));
        }
        let body = &buf[start..end];
        Ok(AcpiTable { header, body })
    }
}

/// Differentiated System Description Table.
#[derive(Debug, Clone)]
pub struct Dsdt<'a> {
    pub table: AcpiTable<'a>,
}

impl<'a> Dsdt<'a> {
    pub fn parse(buf: &'a [u8], offset: usize) -> Result<Self> {
        let table = AcpiTable::parse(buf, offset)?;
        if &table.header.signature != b"DSDT" {
            return Err(ParseError::bad_signature(offset, *b"DSDT", table.header.signature));
        }
        Ok(Dsdt { table })
    }
}

/// Secondary System Description Table.
#[derive(Debug, Clone)]
pub struct Ssdt<'a> {
    pub table: AcpiTable<'a>,
}

impl<'a> Ssdt<'a> {
    pub fn parse(buf: &'a [u8], offset: usize) -> Result<Self> {
        let table = AcpiTable::parse(buf, offset)?;
        if &table.header.signature != b"SSDT" {
            return Err(ParseError::bad_signature(offset, *b"SSDT", table.header.signature));
        }
        Ok(Ssdt { table })
    }
}

/// Fixed ACPI Description Table. Parses the standard header plus the
/// `FIRMWARE_CTRL` (FACS) and `DSDT` pointers.
#[derive(Debug, Clone)]
pub struct Fadt<'a> {
    pub header: AcpiTableHeader,
    pub firmware_ctrl: u32,
    pub dsdt: u32,
    pub preferred_pm_profile: u8,
    pub body: &'a [u8],
}

impl<'a> Fadt<'a> {
    pub fn parse(buf: &'a [u8], offset: usize) -> Result<Self> {
        let r = Reader::new(buf);
        let header = AcpiTableHeader::parse(&r, offset)?;
        if &header.signature != b"FACP" {
            return Err(ParseError::bad_signature(offset, *b"FACP", header.signature));
        }
        let firmware_ctrl = r.u32(offset + 40).ok_or_else(|| ParseError::too_short(offset + 40))?;
        let dsdt = r.u32(offset + 44).ok_or_else(|| ParseError::too_short(offset + 44))?;
        let preferred_pm_profile =
            r.u8(offset + 48).ok_or_else(|| ParseError::too_short(offset + 48))?;
        let end = offset
            .checked_add(header.length as usize)
            .ok_or_else(|| ParseError::too_short(offset))?;
        if end > buf.len() {
            return Err(ParseError::too_short(end));
        }
        let body = &buf[offset..end];
        Ok(Fadt { header, firmware_ctrl, dsdt, preferred_pm_profile, body })
    }
}

/// Validate an ACPI checksum (sum of all header bytes mod 256 == 0).
pub fn validate_checksum(buf: &[u8], offset: usize, length: usize) -> bool {
    if offset + length > buf.len() {
        return false;
    }
    let sum = buf[offset..offset + length].iter().fold(0u8, |a, &b| a.wrapping_add(b));
    sum == 0
}
