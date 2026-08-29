# tpt-l-pipe-weaver

A pro-audio DSP graph builder with lock-free ring buffers over the PipeWire C
API.

## Features

- FFI layer (`src/bindings.rs`) over `pw_context`, `pw_core`, and `pw_stream`.
- Lock-free DSP graph: node/port model backed by `crossbeam` ring buffers.
- Real-time thread via `sched_setscheduler(SCHED_FIFO)` with priority negotiation.
- Graph builder API: add nodes, connect ports, set buffer sizes.
- 100% safe Rust public surface.

## Example

```rust
use tpt_l_pipe_weaver::{Graph, NodeKind};

let mut g = Graph::connect()?;
let src = g.add_node(NodeKind::source("loopback"))?;
let sink = g.add_node(NodeKind::sink("loopback"))?;
g.connect_ports(src.out(0), sink.in(0))?;
g.set_buffer_size(1024)?;
g.start()?;
```

## Requirements

- A running PipeWire session (`PIPEWIRE_REMOTE` or the default socket).
- Real-time scheduling privileges for the DSP thread.

## License

MIT OR Apache-2.0.
