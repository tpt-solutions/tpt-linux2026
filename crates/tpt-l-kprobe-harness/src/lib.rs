//! # tpt-l-kprobe-harness
//!
//! A user-space kernel-testing harness. On Linux it attaches `kprobe`/`kretprobe`
//! via `libbpf-rs` (see [`ffi`]); it also manages QEMU guests for reproducible
//! kernel state ([`qemu`]) and—critically—provides a [`MockRegistry`] that lets
//! tests intercept a kernel function call and redirect it to a user-supplied stub
//! *without* a real kernel, so the mock framework itself is fully unit-tested.
//!
//! This crate is triple-licensed (`MIT OR Apache-2.0 OR GPL-2.0`) so it may be
//! linked into kernel modules.

#[cfg(target_os = "linux")]
pub mod ffi;

pub mod mock;
pub mod qemu;

pub use mock::{KprobeHarness, MockRegistry};
