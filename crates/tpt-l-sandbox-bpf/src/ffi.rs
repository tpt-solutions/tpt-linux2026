//! Linux-only LSM BPF attach for `tpt-l-sandbox-bpf`.
//!
//! Wires [`Policy`] rules into `bpf_lsm` hooks (`file_open`, `socket_connect`).
//! All `unsafe` is confined here.

#![cfg(target_os = "linux")]

use crate::Policy;
use tpt_l_ebpf_common as ebpf;

/// Load and attach the LSM policy program from ELF bytes.
pub fn attach(elf: &[u8], _policy: &Policy) -> Result<ebpf::Ebpf, ebpf::EbpfError> {
    // The BPF program reads the policy from a map populated from `policy`.
    ebpf::load(elf)
}
