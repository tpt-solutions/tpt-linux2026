# tpt-l-journal-oracle

Tail `systemd-journald` and turn structured logs into plain-English diagnostics
with a local LLM backend.

## Features

- `src/ffi.rs`: `sd_journal_*` C API bindings (seek, tail, field enumeration).
- Structured log ingestion: journal entries parsed into typed Rust structs.
- Local LLM backend routing structured logs to `llama.cpp` (candle or raw FFI).
- Diagnostic pipeline: structured log → prompt → plain-English explanation.
- Streaming output API: an iterator of diagnostic lines.

## Example

```rust
use tpt_l_journal_oracle::{JournalOracle, LlmBackend};

let mut oracle = JournalOracle::new(LlmBackend::llama_cpp("model.gguf")?)?;
oracle.seek_tail()?;
for line in oracle.diagnose()? {
    println!("{line}");
}
```

## Requirements

- A host running `systemd-journald` (or a journal export stream).
- A local GGUF model for the `llama.cpp` backend.

## License

MIT OR Apache-2.0.
