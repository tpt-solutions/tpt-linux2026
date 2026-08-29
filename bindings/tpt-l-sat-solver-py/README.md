# tpt-l-sat-solver-py

Python bindings (PyO3) for [`tpt-l-sat-solver`](../crates/tpt-l-sat-solver).

```python
from tpt_l_sat_solver_py import SatSolver, solve_cnf

s = SatSolver()
x, y = s.new_var(), s.new_var()
s.add_clause([x, y])
s.add_clause([-x, y])
assert s.solve()
print(s.model())  # e.g. [2] (y is true)

assert solve_cnf([[1, 2], [-1, 2]]) is not None
```

Build with [maturin](https://www.maturin.rs/):

```sh
maturin develop --release
```

## License

MIT OR Apache-2.0.
