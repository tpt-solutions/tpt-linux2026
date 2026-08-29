# DESIGN

> Adapted from `spec.txt`. The original proposal lives at the repository root as
> `spec.txt`; this document is the maintained design reference.

## 1. Goals

`tpt-linux2026` is a Cargo workspace of 10 specialized, deeply technical Rust
libraries targeting systemic friction points of the Linux ecosystem: power
management, HDR/VRR display finalization, kernel-space testing, memory tiering,
and declarative system migration.

The goal is *not* end-user applications but **missing foundational plumbing** for
distro maintainers, compositor developers, and sysadmins.

## 2. PyO3 strategy

- **Core is pure Rust**, `no_std`-compatible where possible. Zero Python deps so
  they link into C/Go/Rust without the Python runtime.
- **Glue is PyO3**, used only where the primary consumer is a scripting
  environment: `tpt-l-sat-solver`, `tpt-l-firmware-archaeologist`,
  `tpt-l-journal-oracle`. Avoided for low-level/real-time crates
  (`volt-orchestrator`, `wayland-hdr-vrr-kit`, `kprobe-harness`, `pipe-weaver`).

## 3. Architecture

See [`README.md`](./README.md#workspace-layout). Internal crates are
`publish = false`; the 10 public crates publish to crates.io in phase order.

## 4. Crate groups

- **Kernel & Hardware:** `volt-orchestrator` (Aya eBPF + cpufreq sysfs),
  `pager-thermo` (userfaultfd + eBPF heatmap), `kprobe-harness`
  (libbpf-rs + QEMU), `firmware-archaeologist` (ACPI/UEFI/ME via zerocopy).
- **Desktop & Media:** `wayland-hdr-vrr-kit` (DRM/KMS atomic + Rec.2100
  tonemap), `pipe-weaver` (PipeWire lock-free DSP).
- **System & Packaging:** `sandbox-bpf` (LSM BPF), `sat-solver` (CDCL),
  `drift-trace` (fanotify/auditd → Nix/Guix).
- **UX & AI:** `journal-oracle` (journald + local LLM diagnostics).

## 5. Cross-cutting rules

1. **Licensing** — `MIT OR Apache-2.0`, with `GPL-2.0` added for
   `tpt-l-kprobe-harness` (kernel-module compat).
2. **`unsafe` isolation** — confined to a single `ffi.rs`/`bindings.rs`; public
   API is 100% safe.
3. **No Tokio in the kernel path** — async-runtime agnostic / `no_std`.
4. **CI kills** — test against LTS kernels (6.6, 6.12) + latest stable via QEMU.

## 6. Roadmap

| Phase | Months | Crates |
|-------|--------|--------|
| 1 Quick Wins | 1–3 | sat-solver, firmware-archaeologist |
| 2 eBPF | 4–7 | volt-orchestrator, pager-thermo, sandbox-bpf |
| 3 Final Bosses | 8–12 | wayland-hdr-vrr-kit, kprobe-harness, pipe-weaver |
| 4 AI & Glue | 12+ | journal-oracle, drift-trace, PyO3 finalization |
