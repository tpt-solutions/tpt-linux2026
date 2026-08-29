//! # tpt-l-firmware-archaeologist
//!
//! Safe, zero-copy parsers for firmware artifacts commonly found on x86 systems:
//!
//! - [`acpi`] — ACPI system description tables: `DSDT`, `SSDT`, `FADT`.
//! - [`uefi`] — UEFI Firmware Volumes (`_FVH`), FFS files, and PE/COFF images.
//! - [`intel_me`] — Intel Management Engine Flash Partition Table (FPT).
//!
//! All `unsafe` code is confined to [`bindings`]; the parser modules expose a
//! fully safe API.
//!
//! ```rust
//! use tpt_l_firmware_archaeologist::acpi::Fadt;
//!
//! // A minimal, checksum-valid FADT blob.
//! let mut buf = vec![0u8; 64];
//! buf[0..4].copy_from_slice(b"FACP");
//! buf[4..8].copy_from_slice(&64u32.to_le_bytes()); // length
//! buf[40..44].copy_from_slice(&0x1000u32.to_le_bytes()); // FIRMWARE_CTRL
//! buf[44..48].copy_from_slice(&0x2000u32.to_le_bytes()); // DSDT
//! buf[48] = 2; // preferred PM profile
//! let fadt = Fadt::parse(&buf, 0).unwrap();
//! assert_eq!(fadt.firmware_ctrl, 0x1000);
//! assert_eq!(fadt.dsdt, 0x2000);
//! ```

pub mod acpi;
pub mod bindings;
pub mod error;
pub mod intel_me;
pub mod uefi;

pub use error::{ParseError, ParseErrorKind, Result};
