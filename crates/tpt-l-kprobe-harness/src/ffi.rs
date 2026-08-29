//! Linux-only `libbpf-rs` wrapper for `kprobe` / `kretprobe` attach/detach.
//!
//! All `unsafe` is confined here.

#![cfg(target_os = "linux")]

use libbpf_rs::Link;
use libbpf_rs::skel::{OpenSkel, Skel, SkelBuilder};

/// Options for attaching a kprobe to a kernel symbol.
#[derive(Debug, Clone)]
pub struct KprobeOptions {
    pub symbol: String,
    pub retprobe: bool,
    pub cookie: u64,
}

/// Attach a kprobe described by `opts`. Returns a link handle that detaches on
/// drop. The BPF skeleton object is produced by `libbpf-rs`'s `skeleton!` macro
/// from the bundled object.
pub fn attach_kprobe(opts: &KprobeOptions) -> Result<Link, String> {
    // Real implementation: build the skeleton, open/load, and
    // `prog.attach_kprobe(&opts.symbol, opts.retprobe)`.
    let _ = opts;
    Err("libbpf-rs skeleton not linked in scaffold build".to_string())
}
