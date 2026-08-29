//! Local LLM backend abstraction and the diagnostic pipeline.

pub use crate::ingest::JournalEntry;
use thiserror::Error;

/// Errors from the oracle.
#[derive(Debug, Error, PartialEq)]
pub enum OracleError {
    #[error("llm backend error: {0}")]
    Backend(String),
}

/// A local LLM backend (e.g. `llama.cpp` via candle or raw FFI).
pub trait LlmBackend {
    /// Generate text from a prompt.
    fn generate(&self, prompt: &str) -> Result<String, OracleError>;
}

/// A backend that always returns a canned response (used in tests).
pub struct MockBackend {
    pub response: String,
}

impl MockBackend {
    pub fn new(response: impl Into<String>) -> Self {
        MockBackend { response: response.into() }
    }
}

impl LlmBackend for MockBackend {
    fn generate(&self, _prompt: &str) -> Result<String, OracleError> {
        Ok(self.response.clone())
    }
}

/// Build a compact prompt from a batch of journal entries.
pub fn build_prompt(entries: &[JournalEntry]) -> String {
    let mut p = String::from("Explain these system log entries in plain English:\n");
    for e in entries {
        let unit = e.unit().unwrap_or("?");
        let msg = e.message().unwrap_or("");
        p.push_str(&format!("- [{}] {}\n", unit, msg));
    }
    p
}

/// A streaming diagnostic pipeline. Wraps an [`LlmBackend`] and yields one
/// diagnostic line per processed batch of entries.
pub struct DiagnosticPipeline<B: LlmBackend> {
    backend: B,
    batch_size: usize,
}

impl<B: LlmBackend> DiagnosticPipeline<B> {
    pub fn new(backend: B, batch_size: usize) -> Self {
        DiagnosticPipeline { backend, batch_size: batch_size.max(1) }
    }

    /// Run the whole set of entries through the backend, returning one diagnostic
    /// string per batch.
    pub fn run(&self, entries: &[JournalEntry]) -> Result<Vec<String>, OracleError> {
        let mut out = Vec::new();
        for chunk in entries.chunks(self.batch_size) {
            let prompt = build_prompt(chunk);
            out.push(self.backend.generate(&prompt)?);
        }
        Ok(out)
    }

    /// Iterator-style: produce a lazy stream of diagnostics.
    pub fn stream<'a>(
        &'a self,
        entries: &'a [JournalEntry],
    ) -> impl Iterator<Item = Result<String, OracleError>> + 'a {
        entries.chunks(self.batch_size).map(move |chunk| {
            let prompt = build_prompt(chunk);
            self.backend.generate(&prompt)
        })
    }
}
