# tpt-l-firmware-archaeologist-py

Python bindings (PyO3) for
[`tpt-l-firmware-archaeologist`](../crates/tpt-l-firmware-archaeologist).

```python
from tpt_l_firmware_archaeologist_py import parse_fadt, parse_fpt

d = parse_fadt(fadt_bytes)
print(d["dsdt"], d["firmware_ctrl"])

p = parse_fpt(me_region_bytes)
print(p["partitions"])
```

Build with [maturin](https://www.maturin.rs/):

```sh
maturin develop --release
```

## License

MIT OR Apache-2.0.
