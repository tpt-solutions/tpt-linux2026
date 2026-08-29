//! Journal entry ingestion and structured parsing.

use std::collections::HashMap;

/// A single parsed journal entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntry {
    pub fields: HashMap<String, String>,
}

impl JournalEntry {
    /// The `MESSAGE` field, if present.
    pub fn message(&self) -> Option<&str> {
        self.fields.get("MESSAGE").map(|s| s.as_str())
    }

    /// The `PRIORITY` field as a numeric syslog level (0..=7), if present.
    pub fn priority(&self) -> Option<u8> {
        self.fields.get("PRIORITY")?.parse().ok()
    }

    /// The `_SYSTEMD_UNIT` field, if present.
    pub fn unit(&self) -> Option<&str> {
        self.fields.get("_SYSTEMD_UNIT").map(|s| s.as_str())
    }
}

/// Parse one journal "export" entry: newline-separated `KEY=VALUE` lines,
/// terminated by a blank line.
pub fn parse_entry(block: &str) -> Option<JournalEntry> {
    let mut fields = HashMap::new();
    for line in block.split('\n') {
        if line.is_empty() {
            continue;
        }
        let (k, v) = line.split_once('=')?;
        if k.is_empty() {
            continue;
        }
        fields.insert(k.to_string(), v.to_string());
    }
    if fields.is_empty() { None } else { Some(JournalEntry { fields }) }
}

/// Parse a multi-entry journal export stream (entries separated by blank lines).
pub fn parse_stream(input: &str) -> Vec<JournalEntry> {
    input.split("\n\n").filter_map(parse_entry).collect()
}
