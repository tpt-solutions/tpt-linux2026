//! # tpt-l-journal-oracle
//!
//! Consumes `systemd-journald` entries and turns them into plain-English
//! diagnostics using a local LLM backend.
//!
//! - [`ingest`] — parse `sd_journal` export entries into typed structs (pure,
//!   tested).
//! - [`oracle`] — the [`LlmBackend`] trait, a mock implementation, and the
//!   [`DiagnosticPipeline`] that streams diagnostic lines (pure, tested).
//! - [`ffi`] — `sd_journal` C API bindings (Linux only).

#[cfg(target_os = "linux")]
pub mod ffi;

pub mod ingest;
pub mod oracle;
