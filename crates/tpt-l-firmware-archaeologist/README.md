# tpt-l-firmware-archaeologist

Safe, zero-copy parsers for firmware artifacts commonly found on x86 systems.

- **`acpi`** — ACPI system description tables: `DSDT`, `SSDT`, `FADT`.
- **`uefi`** — UEFI Firmware Volumes (`_FVH`), FFS files, and PE/COFF images.
- **`intel_me`** — Intel Management Engine Flash Partition Table (FPT).

All `unsafe` code is confined to the `bindings` module; the parser API is fully
safe Rust with bounds-checked, little-endian reads.

## Example

```rust
use tpt_l_firmware_archaeologist::acpi::Fadt;

let mut buf = vec![0u8; 64];
buf[0..4].copy_from_slice(b"FACP");
buf[4..8].copy_from_slice(&64u32.to_le_bytes());
buf[40..44].copy_from_slice(&0x1000u32.to_le_bytes()); // FIRMWARE_CTRL
buf[44..48].copy_from_slice(&0x2000u32.to_le_bytes()); // DSDT
buf[48] = 2;
let fadt = Fadt::parse(&buf, 0).unwrap();
assert_eq!(fadt.dsdt, 0x2000);
```

## Fuzzing

```sh
cargo +nightly fuzz run parse
```

## License

MIT OR Apache-2.0.
