# tpt-l-volt-orchestrator

Event-driven, eBPF-powered CPU power management that drives `cpufreq` sysfs
governors without polling.

## Features

- Aya eBPF programs hooking scheduler ticks and interrupt traces.
- `cpufreq` sysfs read/write interface for frequency-scaling governors.
- Event-driven dispatch loop (`epoll` over eBPF perf buffers) — no busy polling.
- Async-runtime agnostic: no `tokio` dependency, usable from any reactor.
- All `unsafe` confined to `src/ffi.rs`.

## Example

```rust
use tpt_l_volt_orchestrator::{VoltOrchestrator, Policy};

let mut orch = VoltOrchestrator::new()?;
orch.load_programs()?;
orch.set_policy(Policy::powersave())?;
orch.run()?; // blocks, dispatching on eBPF events
```

## Requirements

- Linux 6.6 LTS or newer with `bpf` and `perf_event` support.
- `CAP_BPF` / `CAP_SYS_ADMIN` (or root) to load programs.

## License

MIT OR Apache-2.0.
