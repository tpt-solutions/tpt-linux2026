//! # tpt-l-wayland-hdr-vrr-kit
//!
//! Compositor-agnostic HDR / VRR support for Linux KMS:
//!
//! - [`edid`] — EDID base-block parsing, including HDR static metadata (HDRMD)
//!   descriptors from CEA-861 extensions.
//! - [`tonemap`] — Rec. 2100 PQ/HLG tone-mapping built on
//!   [`tpt-l-math-core`].
//! - [`ffi`] — DRM/KMS atomic-commit and VRR prop sequences (Linux only).
//!
//! The public API is 100% safe Rust.

#[cfg(target_os = "linux")]
pub mod ffi;

pub mod edid;
pub mod tonemap;
