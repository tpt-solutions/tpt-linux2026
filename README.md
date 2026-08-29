# tpt-linux2026

A foundational Cargo workspace of deeply technical, pure-Rust Linux system
libraries — the missing plumbing for distro maintainers, Wayland compositor
developers, and system administrators.

> **Status:** Active development. See [`todo.md`](./todo.md) for the phased
> roadmap and [`DESIGN.md`](./DESIGN.md) for the architecture.

## Workspace layout

```text
tpt-linux2026/
├── internal/                  # Private, unpublished utility crates
│   ├── tpt-l-ebpf-common/     # Shared eBPF macros and helpers (Aya)
│   ├── tpt-l-sys-bindings/    # Shared bindgen C-headers (DRM, KMS, Kernel)
│   └── tpt-l-math-core/       # Shared SAT + color-space math (no_std)
├── crates/                    # The 10 public libraries
│   ├── tpt-l-sat-solver/          # CDCL SAT solver + distro dep resolution
│   ├── tpt-l-firmware-archaeologist/  # ACPI / UEFI / Intel ME parsers
│   ├── tpt-l-volt-orchestrator/  # eBPF event-driven power management
│   ├── tpt-l-pager-thermo/       # userfaultfd + eBPF memory tiering
│   ├── tpt-l-sandbox-bpf/        # LSM BPF sandbox
│   ├── tpt-l-wayland-hdr-vrr-kit/ # HDR / VRR KMS kit
│   ├── tpt-l-kprobe-harness/     # kprobe test/mock harness (triple-licensed)
│   ├── tpt-l-pipe-weaver/        # Lock-free PipeWire DSP graph
│   ├── tpt-l-journal-oracle/     # systemd-journal + local LLM diagnostics
│   └── tpt-l-drift-trace/        # fanotify/auditd → Nix/Guix migration
└── bindings/                  # Optional PyO3 wrappers
    ├── tpt-l-sat-solver-py/
    ├── tpt-l-firmware-archaeologist-py/
    └── tpt-l-journal-oracle-py/
```

## Design principles

1. **Pure Rust core** — every crate is `#![forbid(unsafe_op_in_unsafe_fn)]` safe
   at its public surface; `unsafe` is confined to a single `ffi.rs`/`bindings.rs`
   module per FFI crate.
2. **Dual license** — `MIT OR Apache-2.0` everywhere, except
   `tpt-l-kprobe-harness`, which is `MIT OR Apache-2.0 OR GPL-2.0` for kernel
   module compatibility.
3. **No Tokio in the kernel path** — real-time/eBPF crates stay async-runtime
   agnostic or `no_std` where possible.

## Building

```sh
cargo build --workspace
cargo test  --workspace
```

Kernel-adjacent crates compile on Linux targets; the algorithmic crates
(`tpt-l-sat-solver`, `tpt-l-firmware-archaeologist`, `tpt-l-math-core`) build and
test on any platform.

## License

Licensed under either of [MIT](./LICENSE-MIT) or
[Apache-2.0](./LICENSE-APACHE) at your option.
See [LICENSE-GPL-2.0](./LICENSE-GPL-2.0) for the `tpt-l-kprobe-harness`
exception.
