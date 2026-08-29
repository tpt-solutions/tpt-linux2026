# tpt-l-wayland-hdr-vrr-kit

Compositor-agnostic HDR/VRR KMS kit: EDID parsing, Rec. 2100 tone-mapping, and
atomic variable-refresh-rate control.

## Features

- `src/ffi.rs`: DRM/KMS C bindings (`drmModeAtomicCommit`, plane/CRTC props).
- EDID parser: extended color-gamut descriptors and HDR static metadata (HDRMD).
- Rec. 2100 PQ + HLG tone-mapping (reuses `tpt-l-math-core` color-space math).
- VRR timing control: the variable-refresh-rate atomic prop sequence.
- 100% safe Rust public API surface.

## Example

```rust
use tpt_l_wayland_hdr_vrr_kit::{Edid, ToneMap, VrrController};

let edid = Edid::parse(edid_blob)?;
let tm = ToneMap::rec2100_pq(edid.hdr_metadata()?);
let mut vrr = VrrController::open(card_path)?;
vrr.enable(edid.connector, tm)?;
```

## Requirements

- Linux with a KMS driver (e.g. `vkms` in QEMU for integration tests).
- DRM master privileges for atomic commits.

## License

MIT OR Apache-2.0.
