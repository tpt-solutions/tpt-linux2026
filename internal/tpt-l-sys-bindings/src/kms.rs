//! KMS (Kernel Mode Setting) constant tables and safe helpers.

/// Content type hints reported by displays (DRM_MODE_CONTENT_TYPE_*).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ContentType {
    NoData = 0,
    Graphics = 1,
    Photo = 2,
    Cinema = 3,
    Game = 4,
}

/// HDR static metadata (HDR_STATIC_METADATA) block type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HdrMetadataType {
    Static = 0,
}

/// Colorimetry / transfer-function (EOTF) bits from the HDR metadata block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Eotf {
    TraditionalSdr = 0,
    TraditionalHdr = 1,
    Pq = 2,
    Hlg = 3,
}

/// Map an [`Eotf`] to a human-readable name.
pub fn eotf_name(e: Eotf) -> &'static str {
    match e {
        Eotf::TraditionalSdr => "Traditional SDR",
        Eotf::TraditionalHdr => "Traditional HDR",
        Eotf::Pq => "PQ (SMPTE ST.2084)",
        Eotf::Hlg => "HLG (BT.2100)",
    }
}

/// A supported display mode resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mode {
    pub hdisplay: u16,
    pub vdisplay: u16,
    pub vrefresh: u32,
    pub flags: u32,
}

#[cfg(target_os = "linux")]
pub use crate::ffi;
