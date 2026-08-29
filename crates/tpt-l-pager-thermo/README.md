# tpt-l-pager-thermo

Intelligent memory tiering that intercepts page faults in user space via
`userfaultfd` and steers hot/cold pages using an eBPF page-fault heatmap.

## Features

- `userfaultfd` handler intercepting minor/major page faults.
- eBPF page-fault heatmap: access frequency per page range.
- Hot/cold tiering engine: `madvise(MADV_COLD)` + `migrate_pages` to NVMe swap.
- `no_std` where possible; all `unsafe` confined to `src/ffi.rs`.

## Example

```rust
use tpt_l_pager_thermo::{PagerThermo, TieringConfig};

let mut pt = PagerThermo::new(TieringConfig::default())?;
pt.register_mapping(region_ptr, region_len)?;
pt.run()?; // monitors faults and migrates cold pages
```

## Requirements

- Linux with `userfaultfd` and `bpf` support.
- `CAP_SYS_ADMIN` (or root) to attach eBPF and migrate pages.

## License

MIT OR Apache-2.0.
