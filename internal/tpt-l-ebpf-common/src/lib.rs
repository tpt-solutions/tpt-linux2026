//! # tpt-l-ebpf-common
//!
//! Shared helpers for the eBPF crates in this workspace.
//!
//! - Platform-independent [`macros`] (`program_name!`, default map sizing).
//! - [`loader`] — thin, safe wrappers over [Aya](https://aya-rs.dev) for
//!   loading and running eBPF objects, available only on Linux targets.
//!
//! All `unsafe` is confined to the loader module.

pub mod macros;

#[cfg(target_os = "linux")]
pub mod loader;

/// Common error type for loader operations (Linux only).
#[cfg(target_os = "linux")]
pub use loader::EbpfError;
