//! EDID base-block parsing and HDR static-metadata extraction.
//!
//! Parses the 128-byte base block (and HDR metadata from a CEA-861 extension)
//! using only safe, bounds-checked reads. No heap allocations beyond the decoded
//! structures.

use thiserror::Error;
use tpt_l_math_core::color;

#[derive(Debug, Error)]
pub enum EdidError {
    #[error("EDID too short: {0} bytes")]
    TooShort(usize),
    #[error("bad header magic (expected 00FFFFFFFFFFFF00)")]
    BadHeader,
    #[error("checksum mismatch")]
    BadChecksum,
}

/// Decoded EDID base block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edid {
    pub manufacturer: [u8; 3],
    pub product_code: u16,
    pub serial: u32,
    pub version_major: u8,
    pub version_minor: u8,
    /// Detected HDR static metadata, if present.
    pub hdr: Option<HdrStaticMetadata>,
}

/// HDR static metadata block (CTA-861-G, tag 0x87).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdrStaticMetadata {
    pub eotf: u8,
    pub desired_max_luminance: u16,
    pub desired_max_frame_avg_luminance: u16,
    pub desired_min_luminance: u16,
}

impl Edid {
    /// Parse a full EDID blob (base block + extensions). Only the base block is
    /// required; the first CEA-861 extension carrying an HDR descriptor is read.
    pub fn parse(buf: &[u8]) -> Result<Self, EdidError> {
        if buf.len() < 128 {
            return Err(EdidError::TooShort(buf.len()));
        }
        let header = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];
        if buf[0..8] != header {
            return Err(EdidError::BadHeader);
        }
        let mut sum = 0u8;
        for &b in &buf[..128] {
            sum = sum.wrapping_add(b);
        }
        if sum != 0 {
            return Err(EdidError::BadChecksum);
        }

        // Manufacturer: 3 letters encoded in 16 bits (5 bits each).
        let m = u16::from_be_bytes([buf[8], buf[9]]);
        let c = |shift: u8| -> u8 { (((m >> shift) & 0x1F) + (b'A' as u16) - 1) as u8 };
        let manufacturer = [c(10), c(5), c(0)];

        let product_code = u16::from_le_bytes([buf[10], buf[11]]);
        let serial = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
        let version_major = buf[18];
        let version_minor = buf[19];

        // Scan extensions for a CEA-861 block with an HDR descriptor.
        let mut hdr = None;
        let num_ext = buf[126] as usize;
        let mut off = 128;
        for _ in 0..num_ext {
            if off + 128 > buf.len() {
                break;
            }
            if buf[off] == 0x02 {
                // CEA-861 extension.
                if let Some(h) = parse_cea_hdr(&buf[off..off + 128]) {
                    hdr = Some(h);
                    break;
                }
            }
            off += 128;
        }

        Ok(Edid { manufacturer, product_code, serial, version_major, version_minor, hdr })
    }
}

/// Parse a CEA-861 extension block for the HDR static metadata descriptor.
fn parse_cea_hdr(block: &[u8]) -> Option<HdrStaticMetadata> {
    // Data block collection starts at offset 4; dtd_start at offset 2.
    let dtd_start = block[2] as usize;
    let mut pos = 4usize;
    while pos + 1 < dtd_start && pos + 1 < block.len() {
        let tag_len = block[pos] as usize;
        let len = tag_len & 0x1F;
        let tag = tag_len >> 5;
        if len == 0 {
            break;
        }
        if tag == 0x07 {
            // Extended tag; the second byte is the extended tag code.
            let ext = block.get(pos + 1)?;
            if *ext == 0x87 && len >= 3 {
                let eotf = block[pos + 2];
                // Bytes 4..6: desired max luminance (12-bit). 0x0000 means N/A.
                let max_lum = u16::from_le_bytes([block[pos + 3], block[pos + 4] & 0x0F]);
                let favg = u16::from_le_bytes([block[pos + 5], block[pos + 6] & 0x0F]);
                let min_lum = block[pos + 7] as u16; // 8-bit in 1 cd/m² units (simplified)
                return Some(HdrStaticMetadata {
                    eotf,
                    desired_max_luminance: max_lum,
                    desired_max_frame_avg_luminance: favg,
                    desired_min_luminance: min_lum,
                });
            }
        }
        pos += len + 1;
    }
    None
}

/// Convert an SDR (linear) value to a PQ-encoded signal in `[0, 1]`.
pub fn to_pq(linear: f64) -> f64 {
    color::pq_oetf(linear)
}

/// Convert a PQ-encoded signal back to linear light in `[0, 1]`.
pub fn from_pq(signal: f64) -> f64 {
    color::pq_eotf(signal)
}
