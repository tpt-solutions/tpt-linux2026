# Changelog

All notable changes to this workspace are documented in this file. The format is
based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Workspace bootstrap: root manifest, toolchain pin, lint configs, licenses.
- `internal/tpt-l-math-core`: shared SAT primitives + Rec. 2100 color math.
- `internal/tpt-l-ebpf-common`: shared Aya eBPF helpers.
- `internal/tpt-l-sys-bindings`: bindgen DRM/KMS/kernel header bindings.
- `crates/tpt-l-sat-solver`: CDCL solver with cross-distro dep API.
- `crates/tpt-l-firmware-archaeologist`: ACPI/UEFI/Intel ME parsers.
- Phase 2–4 crates: initial implementations with FFI isolated.
- PyO3 bindings: `tpt-l-sat-solver-py`, `tpt-l-firmware-archaeologist-py`,
  `tpt-l-journal-oracle-py`.
- CI/CD workflows, `cargo-deny` and `cargo-audit` configs.
