# tpt-linux2026 · Master Todo Checklist

**Author:** TPT Solutions  
**License:** MIT OR Apache-2.0 (kprobe-harness: MIT OR Apache-2.0 OR GPL-2.0)  
**Rust Edition:** 2024  

---

## Workspace Bootstrap

- [x] `git init` and push initial commit to GitHub
- [x] Create root `Cargo.toml` (workspace manifest listing all members)
- [x] Create `.gitignore` (standard Rust + IDE artefacts)
- [x] Create `LICENSE-MIT`
- [x] Create `LICENSE-APACHE`
- [x] Create `LICENSE-GPL-2.0` (for kprobe-harness only)
- [x] Create `README.md` (workspace-level overview)
- [x] Create `DESIGN.md` (copy/adapt from spec)
- [x] Create `CONTRIBUTING.md`
- [x] Create `CHANGELOG.md`
- [x] Configure `cargo-deny` (`deny.toml`) for license + duplicate-dep auditing
- [x] Add `rust-toolchain.toml` pinning Edition 2024 stable toolchain
- [x] Add `rustfmt.toml` and `clippy.toml` project-wide lint config

---

## Internal Crates (publish = false)

### `internal/tpt-l-ebpf-common`
- [x] `Cargo.toml` (`publish = false`, Aya dependency)
- [x] `src/lib.rs` — shared eBPF macros and helpers for Aya

### `internal/tpt-l-sys-bindings`
- [x] `Cargo.toml` (`publish = false`, bindgen build dependency)
- [x] `build.rs` — bindgen over DRM, KMS, and kernel C headers
- [x] `src/lib.rs` — re-export generated bindings

### `internal/tpt-l-math-core`
- [x] `Cargo.toml` (`publish = false`, `no_std` compatible)
- [x] `src/lib.rs` — shared SAT-solving primitives + color-space math (Rec. 2100)

---

## Phase 1 · Quick Wins (Months 1–3)

### `crates/tpt-l-sat-solver`
- [x] `Cargo.toml` (crates.io metadata: description, keywords, categories, repository)
- [x] `src/lib.rs` stub with public API skeleton
- [x] `README.md`
- [x] Implement CDCL (Conflict-Driven Clause Learning) core — unit propagation
- [x] Implement CDCL — conflict analysis + clause learning
- [x] Implement CDCL — non-chronological backtracking
- [x] Implement CDCL — VSIDS decision heuristic
- [x] Expose cross-distro dependency resolution API (`solve`, `add_clause`, `add_package_constraint`)
- [x] Unit tests — basic SAT/UNSAT cases
- [x] Integration tests — real-world dependency conflict scenarios
- [x] Benchmark (`benches/`) — compare against MiniSat baseline

### `crates/tpt-l-firmware-archaeologist`
- [x] `Cargo.toml` (crates.io metadata; deps: `zerocopy`, `bytemuck`)
- [x] `src/lib.rs` stub
- [x] `README.md`
- [x] ACPI table parser (`DSDT`, `SSDT`, `FADT`)
- [x] UEFI firmware volume + PE/COFF image parser
- [x] Intel ME region parser (FPT header, partition table)
- [x] Safe zero-copy binary-struct API (all `unsafe` confined to `src/bindings.rs`)
- [x] Unit tests — parse known-good fixture blobs
- [x] Fuzz targets (`fuzz/`) via `cargo-fuzz`

### `bindings/tpt-l-sat-solver-py`
- [x] `Cargo.toml` (PyO3 dep, `cdylib` crate-type)
- [x] `src/lib.rs` — `#[pymodule]` wrapping sat-solver public API
- [x] `pyproject.toml` + `maturin` build setup
- [x] Python smoke tests

### `bindings/tpt-l-firmware-archaeologist-py`
- [x] `Cargo.toml` (PyO3 dep, `cdylib` crate-type)
- [x] `src/lib.rs` — `#[pymodule]` wrapping firmware-archaeologist public API
- [x] `pyproject.toml` + `maturin` build setup
- [x] Python smoke tests

---

## Phase 2 · eBPF Revolution (Months 4–7)

### `crates/tpt-l-volt-orchestrator`
- [x] `Cargo.toml` (async-runtime agnostic, no tokio; deps: `aya`, internal `tpt-l-ebpf-common`)
- [x] `src/lib.rs` stub
- [x] `README.md`
- [x] eBPF program: hook into scheduler tick + interrupt traces (Aya)
- [x] `cpufreq` sysfs interface (read/write freq scaling governors without polling)
- [x] Event-driven dispatch loop (epoll over eBPF perf buffers)
- [x] `unsafe` confined to `src/ffi.rs`
- [x] Unit tests (mock sysfs paths)
- [x] Integration test against kernel 6.6 LTS + 6.12 LTS via QEMU

### `crates/tpt-l-pager-thermo`
- [x] `Cargo.toml` (`no_std` where possible; deps: `aya`, internal `tpt-l-ebpf-common`)
- [x] `src/lib.rs` stub
- [x] `README.md`
- [x] `userfaultfd` handler — intercept page faults in user-space
- [x] eBPF page-fault heatmap (access frequency per page range)
- [x] Hot/cold tiering engine: `madvise(MADV_COLD)` + `migrate_pages` to NVMe swap
- [x] `unsafe` confined to `src/ffi.rs`
- [x] Unit tests
- [x] Integration test (QEMU, measure RSS migration under memory pressure)

### `crates/tpt-l-sandbox-bpf`
- [x] `Cargo.toml` (deps: `aya`, internal `tpt-l-ebpf-common`)
- [x] `src/lib.rs` stub
- [x] `README.md`
- [x] LSM BPF policy engine — `bpf_lsm` hooks for `file_open`, `socket_connect`
- [x] Filesystem namespace enforcement (path allow/deny lists)
- [x] Network namespace enforcement (allowed socket types/ports)
- [x] Policy builder API (Rust + Python-friendly)
- [x] `unsafe` confined to `src/ffi.rs`
- [x] Unit tests (mock LSM hook calls)
- [x] Integration test (QEMU, verify sandbox blocks escape attempts)

---

## Phase 3 · Final Bosses (Months 8–12)

### `crates/tpt-l-wayland-hdr-vrr-kit`
- [x] `Cargo.toml` (deps: internal `tpt-l-sys-bindings`, `tpt-l-math-core`)
- [x] `src/lib.rs` stub
- [x] `README.md`
- [x] `src/ffi.rs` — DRM/KMS C bindings (`drmModeAtomicCommit`, plane/CRTC props)
- [x] EDID parser: extended color gamut descriptors, HDR static metadata (HDRMD)
- [x] Rec. 2100 PQ + HLG tone-mapping (uses `tpt-l-math-core` color-space math)
- [x] VRR timing control: variable refresh rate atomic prop sequence
- [x] Public API: 100% safe Rust surface
- [x] Unit tests — EDID fixture parsing, tone-map math correctness
- [x] Integration test (QEMU with `vkms` virtual KMS driver)

### `crates/tpt-l-kprobe-harness`
- [x] `Cargo.toml` — **license: `MIT OR Apache-2.0 OR GPL-2.0`**; deps: `libbpf-rs`, internal `tpt-l-ebpf-common`
- [x] `src/lib.rs` stub
- [x] `README.md` (note triple-license and kernel module compatibility)
- [x] `libbpf-rs` wrapper for `kprobe` / `kretprobe` attach/detach
- [x] QEMU process management API (spawn, snapshot, restore kernel state)
- [x] Kernel function mock framework: intercept via kprobe → redirect to user-supplied stub
- [x] `cargo test` integration: `KprobeHarness` struct as test fixture
- [x] `unsafe` confined to `src/ffi.rs`
- [x] Tests against kernel 6.6 LTS (QEMU)
- [x] Tests against kernel 6.12 LTS (QEMU)
- [x] Tests against latest stable kernel (QEMU)

### `crates/tpt-l-pipe-weaver`
- [x] `Cargo.toml` (deps: internal `tpt-l-sys-bindings`; `crossbeam`)
- [x] `src/lib.rs` stub
- [x] `README.md`
- [x] `src/bindings.rs` — PipeWire C API FFI (`pw_context`, `pw_core`, `pw_stream`)
- [x] Lock-free DSP graph: node/port model using `crossbeam` ring buffers
- [x] Real-time thread: `sched_setscheduler(SCHED_FIFO)` with priority negotiation
- [x] Graph builder API: add nodes, connect ports, set buffer sizes
- [x] Public API: 100% safe Rust surface
- [x] Unit tests (mock PipeWire callbacks)
- [x] Integration test (real PipeWire socket, loopback DSP graph)

---

## Phase 4 · AI & Glue (Months 12+)

### `crates/tpt-l-journal-oracle`
- [x] `Cargo.toml` (deps: internal `tpt-l-sys-bindings`; `candle-core` or `llama-cpp-rs`)
- [x] `src/lib.rs` stub
- [x] `README.md`
- [x] `src/ffi.rs` — `sd_journal_*` C API bindings (tail, seek, field enumeration)
- [x] Structured log ingestion: parse journal entries into typed Rust structs
- [x] Local LLM backend: route structured logs to `llama.cpp` (candle or raw FFI)
- [x] Diagnostic pipeline: structured log → prompt → plain-English explanation
- [x] Streaming output API (iterator of diagnostic lines)
- [x] Unit tests (fixture journal entries, mock LLM backend)
- [x] Integration test (live journald, real llama.cpp model)

### `crates/tpt-l-drift-trace`
- [x] `Cargo.toml` (deps: `nix` crate for netlink)
- [x] `src/lib.rs` stub
- [x] `README.md`
- [x] `fanotify` filesystem mutation tracker (create/write/rename/delete events)
- [x] `auditd` rule injection via netlink sockets (track `execve`, `open`, `unlink`)
- [x] Mutation log accumulator: deduplicate + coalesce events
- [x] Nix expression generator from accumulated mutations
- [x] Guix expression generator from accumulated mutations
- [x] Unit tests (replay fixture event streams)
- [x] Integration test (QEMU, install a package, verify generated expression)

### PyO3 Bindings Finalization
- [x] `bindings/tpt-l-journal-oracle-py` — `Cargo.toml`, `#[pymodule]`, `pyproject.toml`, maturin build, Python smoke tests
- [x] Audit + harden `tpt-l-sat-solver-py` (API coverage, error propagation, GIL hygiene)
- [x] Audit + harden `tpt-l-firmware-archaeologist-py` (API coverage, error propagation)
- [ ] Publish all three `*-py` wheels to PyPI

---

## CI/CD

- [x] `.github/workflows/ci.yml` — `cargo check`, `cargo clippy`, `cargo fmt --check`, `cargo test` on all crates
- [x] CI matrix: kernel 6.6 LTS + 6.12 LTS + latest stable (QEMU runner or dedicated action)
- [x] QEMU kernel image build/cache step in CI
- [x] `cargo-deny` check step (license + advisories)
- [x] `cargo-audit` security audit step
- [x] `.github/workflows/publish.yml` — triggered on version tag; publishes to crates.io in phase order
- [ ] Add `CRATES_IO_TOKEN` + `PYPI_TOKEN` to GitHub repository secrets (manual step)
- [ ] Branch protection: require CI green before merge to `main`
- [x] Dependabot config for Rust + GitHub Actions updates

---

## crates.io Publishing

> Publish in phase order; each crate requires all its workspace dependencies to be published first.

- [x] Verify all 10 public crate `Cargo.toml` fields: `description`, `keywords` (≤5), `categories`, `repository`, `homepage`, `documentation`
- [ ] **Phase 1 release** — publish `tpt-l-sat-solver` → `tpt-l-firmware-archaeologist`
- [ ] **Phase 2 release** — publish `tpt-l-volt-orchestrator` → `tpt-l-pager-thermo` → `tpt-l-sandbox-bpf`
- [ ] **Phase 3 release** — publish `tpt-l-wayland-hdr-vrr-kit` → `tpt-l-kprobe-harness` → `tpt-l-pipe-weaver`
- [ ] **Phase 4 release** — publish `tpt-l-journal-oracle` → `tpt-l-drift-trace`
- [ ] Reserve crate names on crates.io early (publish `0.0.1` stubs if needed to avoid squatting)
