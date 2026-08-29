# tpt-l-sandbox-bpf

A lightweight LSM BPF sandbox enforcing filesystem and network namespace policy
for untrusted workloads.

## Features

- LSM BPF policy engine: `bpf_lsm` hooks for `file_open` and `socket_connect`.
- Filesystem namespace enforcement via path allow/deny lists.
- Network namespace enforcement: allowed socket types, protocols, and ports.
- Ergonomic policy builder API (Rust, and Python-friendly through the bindings).
- All `unsafe` confined to `src/ffi.rs`.

## Example

```rust
use tpt_l_sandbox_bpf::{Policy, Sandbox};

let policy = Policy::builder()
    .allow_path_prefix("/usr/bin")
    .allow_path_prefix("/lib")
    .allow_tcp(443)
    .deny_all_else()
    .build();

let sb = Sandbox::new(policy)?;
sb.apply()?;
sb.spawn(|| run_untrusted())?;
```

## Requirements

- Linux 6.6 LTS+ with LSM BPF (`CONFIG_BPF_LSM=y`, `lsm=capability,bpf`).

## License

MIT OR Apache-2.0.
