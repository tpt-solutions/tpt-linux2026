# tpt-l-kprobe-harness

A user-space kernel-testing harness built on `libbpf-rs` kprobes, with a
QEMU-backed runner and a kernel-function mock framework.

> **Note:** This crate is triple-licensed `MIT OR Apache-2.0 OR GPL-2.0` so it
> may be linked into GPL-2.0 kernel modules.

## Features

- `libbpf-rs` wrapper for `kprobe` / `kretprobe` attach/detach.
- QEMU process management: spawn, snapshot, and restore kernel state.
- Kernel-function mock framework: intercept a function via kprobe and redirect
  to a user-supplied stub.
- `KprobeHarness` test fixture usable directly from `cargo test`.
- All `unsafe` confined to `src/ffi.rs`.

## Example

```rust
use tpt_l_kprobe_harness::KprobeHarness;

let mut h = KprobeHarness::new("6.12")?;
h.spawn()?;
h.mock("vfs_read", |regs| Ok(0))?;
let report = h.run_test("tests/selftests/read_probe")?;
assert!(report.passed);
```

## Requirements

- Linux 6.6 / 6.12 / latest stable under QEMU.
- `libbpf` (v1.x) and `qemu-system-x86_64` on the host.

## License

MIT OR Apache-2.0 OR GPL-2.0.
