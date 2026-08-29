//! Shared mathematics for the `tpt-linux2026` workspace.
//!
//! This crate is `#![no_std]` and dependency-free. It provides the low-level
//! SAT primitives used by [`tpt-l-sat-solver`](crate-adjacent) and the
//! Rec. 2100 color-space math used by [`tpt-l-wayland-hdr-vrr-kit`].

#![no_std]
#![forbid(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]

extern crate alloc;

#[cfg(test)]
extern crate std;

pub mod color;
pub mod sat;
