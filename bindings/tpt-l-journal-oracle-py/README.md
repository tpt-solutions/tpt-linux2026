# tpt-l-journal-oracle-py

Python bindings for [`tpt-l-journal-oracle`](https://crates.io/crates/tpt-l-journal-oracle),
exposing the journal-tailing diagnostic oracle to Python via PyO3/maturin.

## Install

```sh
pip install tpt-l-journal-oracle-py
```

Or build from source with maturin:

```sh
maturin develop
```

## Example

```python
from tpt_l_journal_oracle_py import JournalOracle

oracle = JournalOracle("model.gguf")
oracle.seek_tail()
for line in oracle.diagnose():
    print(line)
```

## License

MIT OR Apache-2.0.
