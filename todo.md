# tpt-linux2026 · Master Todo Checklist

**Author:** TPT Solutions  
**License:** MIT OR Apache-2.0 (kprobe-harness: MIT OR Apache-2.0 OR GPL-2.0)  
**Rust Edition:** 2024  

---

## Workspace Bootstrap

- [ ] `git init` and push initial commit to GitHub
- [ ] Create root `Cargo.toml` (workspace manifest listing all members)
- [ ] Create `.gitignore` (standard Rust + IDE artefacts)
- [ ] Create `LICENSE-MIT`
- [ ] Create `LICENSE-APACHE`
- [ ] Create `LICENSE-GPL-2.0` (for kprobe-harness only)
- [ ] Create `README.md` (workspace-level overview)
- [ ] Create `DESIGN.md` (copy/adapt from spec)
- [ ] Create `CONTRIBUTING.md`
- [ ] Create `CHANGELOG.md`
- [ ] Configure `cargo-deny` (`deny.toml`) for license + duplicate-dep auditing
- [ ] Add `rust-toolchain.toml` pinning Edition 2024 stable toolchain
- [ ] Add `rustfmt.toml` and `clippy.toml` project-wide lint config

---

## Internal Crates (publish = false)

### `internal/tpt-l-ebpf-common`
- [ ] `Cargo.toml` (`publish = false`, Aya dependency)
- [ ] `src/lib.rs` — shared eBPF macros and helpers for Aya

### `internal/tpt-l-sys-bindings`
- [ ] `Cargo.toml` (`publish = false`, bindgen build dependency)
- [ ] `build.rs` — bindgen over DRM, KMS, and kernel C headers
- [ ] `src/lib.rs` — re-export generated bindings

### `internal/tpt-l-math-core`
- [ ] `Cargo.toml` (`publish = false`, `no_std` compatible)
- [ ] `src/lib.rs` — shared SAT-solving primitives + color-space math (Rec. 2100)

---

## Phase 1 · Quick Wins (Months 1–3)

### `crates/tpt-l-sat-solver`
- [ ] `Cargo.toml` (crates.io metadata: description, keywords, categories, repository)
- [ ] `src/lib.rs` stub with public API skeleton
- [ ] `README.md`
- [ ] Implement CDCL (Conflict-Driven Clause Learning) core — unit propagation
- [ ] Implement CDCL — conflict analysis + clause learning
- [ ] Implement CDCL — non-chronological backtracking
- [ ] Implement CDCL — VSIDS decision heuristic
- [ ] Expose cross-distro dependency resolution API (`solve`, `add_clause`, `add_package_constraint`)
- [ ] Unit tests — basic SAT/UNSAT cases
- [ ] Integration tests — real-world dependency conflict scenarios
- [ ] Benchmark (`benches/`) — compare against MiniSat baseline

### `crates/tpt-l-firmware-archaeologist`
- [ ] `Cargo.toml` (crates.io metadata; deps: `zerocopy`, `bytemuck`)
- [ ] `src/lib.rs` stub
- [ ] `README.md`
- [ ] ACPI table parser (`DSDT`, `SSDT`, `FADT`)
- [ ] UEFI firmware volume + PE/COFF image parser
- [ ] Intel ME region parser (FPT header, partition table)
- [ ] Safe zero-copy binary-struct API (all `unsafe` confined to `src/bindings.rs`)
- [ ] Unit tests — parse known-good fixture blobs
- [ ] Fuzz targets (`fuzz/`) via `cargo-fuzz`

### `bindings/tpt-l-sat-solver-py`
- [ ] `Cargo.toml` (PyO3 dep, `cdylib` crate-type)
- [ ] `src/lib.rs` — `#[pymodule]` wrapping sat-solver public API
- [ ] `pyproject.toml` + `maturin` build setup
- [ ] Python smoke tests

### `bindings/tpt-l-firmware-archaeologist-py`
- [ ] `Cargo.toml` (PyO3 dep, `cdylib` crate-type)
- [ ] `src/lib.rs` — `#[pymodule]` wrapping firmware-archaeologist public API
- [ ] `pyproject.toml` + `maturin` build setup
- [ ] Python smoke tests

---

## Phase 2 · eBPF Revolution (Months 4–7)

### `crates/tpt-l-volt-orchestrator`
- [ ] `Cargo.toml` (async-runtime agnostic, no tokio; deps: `aya`, internal `tpt-l-ebpf-common`)
- [ ] `src/lib.rs` stub
- [ ] `README.md`
- [ ] eBPF program: hook into scheduler tick + interrupt traces (Aya)
- [ ] `cpufreq` sysfs interface (read/write freq scaling governors without polling)
- [ ] Event-driven dispatch loop (epoll over eBPF perf buffers)
- [ ] `unsafe` confined to `src/ffi.rs`
- [ ] Unit tests (mock sysfs paths)
- [ ] Integration test against kernel 6.6 LTS + 6.12 LTS via QEMU

### `crates/tpt-l-pager-thermo`
- [ ] `Cargo.toml` (`no_std` where possible; deps: `aya`, internal `tpt-l-ebpf-common`)
- [ ] `src/lib.rs` stub
- [ ] `README.md`
- [ ] `userfaultfd` handler — intercept page faults in user-space
- [ ] eBPF page-fault heatmap (access frequency per page range)
- [ ] Hot/cold tiering engine: `madvise(MADV_COLD)` + `migrate_pages` to NVMe swap
- [ ] `unsafe` confined to `src/ffi.rs`
- [ ] Unit tests
- [ ] Integration test (QEMU, measure RSS migration under memory pressure)

### `crates/tpt-l-sandbox-bpf`
- [ ] `Cargo.toml` (deps: `aya`, internal `tpt-l-ebpf-common`)
- [ ] `src/lib.rs` stub
- [ ] `README.md`
- [ ] LSM BPF policy engine — `bpf_lsm` hooks for `file_open`, `socket_connect`
- [ ] Filesystem namespace enforcement (path allow/deny lists)
- [ ] Network namespace enforcement (allowed socket types/ports)
- [ ] Policy builder API (Rust + Python-friendly)
- [ ] `unsafe` confined to `src/ffi.rs`
- [ ] Unit tests (mock LSM hook calls)
- [ ] Integration test (QEMU, verify sandbox blocks escape attempts)

---

## Phase 3 · Final Bosses (Months 8–12)

### `crates/tpt-l-wayland-hdr-vrr-kit`
- [ ] `Cargo.toml` (deps: internal `tpt-l-sys-bindings`, `tpt-l-math-core`)
- [ ] `src/lib.rs` stub
- [ ] `README.md`
- [ ] `src/ffi.rs` — DRM/KMS C bindings (`drmModeAtomicCommit`, plane/CRTC props)
- [ ] EDID parser: extended color gamut descriptors, HDR static metadata (HDRMD)
- [ ] Rec. 2100 PQ + HLG tone-mapping (uses `tpt-l-math-core` color-space math)
- [ ] VRR timing control: variable refresh rate atomic prop sequence
- [ ] Public API: 100% safe Rust surface
- [ ] Unit tests — EDID fixture parsing, tone-map math correctness
- [ ] Integration test (QEMU with `vkms` virtual KMS driver)

### `crates/tpt-l-kprobe-harness`
- [ ] `Cargo.toml` — **license: `MIT OR Apache-2.0 OR GPL-2.0`**; deps: `libbpf-rs`, internal `tpt-l-ebpf-common`
- [ ] `src/lib.rs` stub
- [ ] `README.md` (note triple-license and kernel module compatibility)
- [ ] `libbpf-rs` wrapper for `kprobe` / `kretprobe` attach/detach
- [ ] QEMU process management API (spawn, snapshot, restore kernel state)
- [ ] Kernel function mock framework: intercept via kprobe → redirect to user-supplied stub
- [ ] `cargo test` integration: `KprobeHarness` struct as test fixture
- [ ] `unsafe` confined to `src/ffi.rs`
- [ ] Tests against kernel 6.6 LTS (QEMU)
- [ ] Tests against kernel 6.12 LTS (QEMU)
- [ ] Tests against latest stable kernel (QEMU)

### `crates/tpt-l-pipe-weaver`
- [ ] `Cargo.toml` (deps: internal `tpt-l-sys-bindings`; `crossbeam`)
- [ ] `src/lib.rs` stub
- [ ] `README.md`
- [ ] `src/bindings.rs` — PipeWire C API FFI (`pw_context`, `pw_core`, `pw_stream`)
- [ ] Lock-free DSP graph: node/port model using `crossbeam` ring buffers
- [ ] Real-time thread: `sched_setscheduler(SCHED_FIFO)` with priority negotiation
- [ ] Graph builder API: add nodes, connect ports, set buffer sizes
- [ ] Public API: 100% safe Rust surface
- [ ] Unit tests (mock PipeWire callbacks)
- [ ] Integration test (real PipeWire socket, loopback DSP graph)

---

## Phase 4 · AI & Glue (Months 12+)

### `crates/tpt-l-journal-oracle`
- [ ] `Cargo.toml` (deps: internal `tpt-l-sys-bindings`; `candle-core` or `llama-cpp-rs`)
- [ ] `src/lib.rs` stub
- [ ] `README.md`
- [ ] `src/ffi.rs` — `sd_journal_*` C API bindings (tail, seek, field enumeration)
- [ ] Structured log ingestion: parse journal entries into typed Rust structs
- [ ] Local LLM backend: route structured logs to `llama.cpp` (candle or raw FFI)
- [ ] Diagnostic pipeline: structured log → prompt → plain-English explanation
- [ ] Streaming output API (iterator of diagnostic lines)
- [ ] Unit tests (fixture journal entries, mock LLM backend)
- [ ] Integration test (live journald, real llama.cpp model)

### `crates/tpt-l-drift-trace`
- [ ] `Cargo.toml` (deps: `nix` crate for netlink)
- [ ] `src/lib.rs` stub
- [ ] `README.md`
- [ ] `fanotify` filesystem mutation tracker (create/write/rename/delete events)
- [ ] `auditd` rule injection via netlink sockets (track `execve`, `open`, `unlink`)
- [ ] Mutation log accumulator: deduplicate + coalesce events
- [ ] Nix expression generator from accumulated mutations
- [ ] Guix expression generator from accumulated mutations
- [ ] Unit tests (replay fixture event streams)
- [ ] Integration test (QEMU, install a package, verify generated expression)

### PyO3 Bindings Finalization
- [ ] `bindings/tpt-l-journal-oracle-py` — `Cargo.toml`, `#[pymodule]`, `pyproject.toml`, maturin build, Python smoke tests
- [ ] Audit + harden `tpt-l-sat-solver-py` (API coverage, error propagation, GIL hygiene)
- [ ] Audit + harden `tpt-l-firmware-archaeologist-py` (API coverage, error propagation)
- [ ] Publish all three `*-py` wheels to PyPI

---

## CI/CD

- [ ] `.github/workflows/ci.yml` — `cargo check`, `cargo clippy`, `cargo fmt --check`, `cargo test` on all crates
- [ ] CI matrix: kernel 6.6 LTS + 6.12 LTS + latest stable (QEMU runner or dedicated action)
- [ ] QEMU kernel image build/cache step in CI
- [ ] `cargo-deny` check step (license + advisories)
- [ ] `cargo-audit` security audit step
- [ ] `.github/workflows/publish.yml` — triggered on version tag; publishes to crates.io in phase order
- [ ] Add `CRATES_IO_TOKEN` + `PYPI_TOKEN` to GitHub repository secrets (manual step)
- [ ] Branch protection: require CI green before merge to `main`
- [ ] Dependabot config for Rust + GitHub Actions updates

---

## crates.io Publishing

> Publish in phase order; each crate requires all its workspace dependencies to be published first.

- [ ] Verify all 10 public crate `Cargo.toml` fields: `description`, `keywords` (≤5), `categories`, `repository`, `homepage`, `documentation`
- [ ] **Phase 1 release** — publish `tpt-l-sat-solver` → `tpt-l-firmware-archaeologist`
- [ ] **Phase 2 release** — publish `tpt-l-volt-orchestrator` → `tpt-l-pager-thermo` → `tpt-l-sandbox-bpf`
- [ ] **Phase 3 release** — publish `tpt-l-wayland-hdr-vrr-kit` → `tpt-l-kprobe-harness` → `tpt-l-pipe-weaver`
- [ ] **Phase 4 release** — publish `tpt-l-journal-oracle` → `tpt-l-drift-trace`
- [ ] Reserve crate names on crates.io early (publish `0.0.1` stubs if needed to avoid squatting)
