# tpt-l-drift-trace

Track filesystem mutations via `fanotify`/`auditd` and generate Nix/Guix
expressions that reproduce the observed system state.

## Features

- `fanotify` filesystem mutation tracker (create/write/rename/delete events).
- `auditd` rule injection via netlink sockets (track `execve`, `open`, `unlink`).
- Mutation log accumulator: deduplicate and coalesce event streams.
- Nix and Guix expression generators from the accumulated mutations.
- 100% safe Rust public API surface.

## Example

```rust
use tpt_l_drift_trace::{DriftTrace, Generator};

let mut dt = DriftTrace::new()?;
dt.watch("/")?;
dt.run(Duration::from_secs(30))?;
let nix = dt.generate(Generator::Nix)?;
std::fs::write("drift.nix", nix)?;
```

## Requirements

- Linux with `fanotify` and `audit` netlink support.
- `CAP_AUDIT_CONTROL` / `CAP_SYS_ADMIN` (or root) to inject audit rules.

## License

MIT OR Apache-2.0.
