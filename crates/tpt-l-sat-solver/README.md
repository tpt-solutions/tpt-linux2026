# tpt-l-sat-solver

A from-scratch **Conflict-Driven Clause Learning (CDCL)** SAT solver in safe
Rust, plus a [`PackageResolver`] for cross-distro dependency resolution.

## Features

- Watched-literal unit propagation.
- 1-UIP conflict analysis with clause learning.
- Non-chronological backtracking.
- VSIDS decision heuristic with activity decay.
- Zero `unsafe` code.

## Example

```rust
use tpt_l_sat_solver::{Lit, SatSolver, Var};

let mut s = SatSolver::new();
let x = s.new_var();
let y = s.new_var();
s.add_clause(&[Lit::new(x, false), Lit::new(y, false)]);
s.add_clause(&[Lit::new(x, true), Lit::new(y, false)]);
assert!(s.solve());
assert_eq!(s.value(Lit::new(y, false)), Some(true));
```

## Cross-distro dependency resolution

```rust
use tpt_l_sat_solver::PackageResolver;

let mut r = PackageResolver::new();
let foo = r.add_package("foo");
let foo_1 = r.add_version(foo, "1.0");
let foo_2 = r.add_version(foo, "2.0");
let bar = r.add_package("bar");
let bar_1 = r.add_version(bar, "1.0");
r.enforce_unique_versions();
r.require_dependency(foo_1, bar, &[bar_1]);
r.request(foo_1);
let solution = r.solve().unwrap();
```

## Benchmarks

```sh
cargo bench -p tpt-l-sat-solver
```

## License

MIT OR Apache-2.0.
