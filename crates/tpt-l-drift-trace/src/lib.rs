//! # tpt-l-drift-trace
//!
//! Records imperative filesystem mutations and derives declarative Nix/Guix
//! expressions describing the resulting system state.
//!
//! - [`log`] — the [`MutationLog`] accumulator with dedupe/coalesce (pure,
//!   tested).
//! - [`generate`] — Nix/Guix expression generators (pure, tested).
//! - [`ffi`] — `fanotify` + `auditd` netlink integration (Linux only).

#[cfg(target_os = "linux")]
pub mod ffi;

pub mod generate;
pub mod log;
