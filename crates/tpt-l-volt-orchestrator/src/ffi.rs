//! Linux-only eBPF integration for `tpt-l-volt-orchestrator`.
//!
//! Hooks into scheduler-tick and interrupt traces and drives the event loop over
//! eBPF perf buffers. All `unsafe` is confined to this module.

#![cfg(target_os = "linux")]

use tpt_l_ebpf_common as ebpf;

/// Load the bundled scheduler-tick eBPF program from ELF bytes and return a
/// handle. The actual object is produced by the workspace eBPF build pipeline.
pub fn load_scheduler_hook(elf: &[u8]) -> Result<ebpf::Ebpf, ebpf::EbpfError> {
    ebpf::load(elf)
}

/// A single scheduler-tick sample delivered from the perf buffer.
#[derive(Debug, Clone, Copy)]
pub struct TickSample {
    pub cpu: u32,
    pub pid: u32,
    pub runtime_ns: u64,
}
