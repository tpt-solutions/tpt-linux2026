//! Filesystem mutation event model and the coalescing accumulator.

use std::collections::HashMap;
use std::path::PathBuf;

/// A single observed filesystem mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationKind {
    Create,
    Write,
    Rename,
    Delete,
}

/// A mutation event captured from `fanotify`/`auditd`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationEvent {
    pub kind: MutationKind,
    pub path: PathBuf,
    /// For renames: the previous path.
    pub from: Option<PathBuf>,
}

/// Accumulates mutation events, coalescing redundant sequences:
///
/// - `Write` followed later by `Delete` on the same path => drop both (the file
///   was created and removed; net effect: nothing persisted).
/// - Repeated `Write` on the same path => keep a single `Write`.
/// - `Delete` then `Create` on the same path => collapse to `Write`.
#[derive(Debug, Clone, Default)]
pub struct MutationLog {
    /// Current net state per path.
    state: HashMap<PathBuf, MutationKind>,
}

impl MutationLog {
    pub fn new() -> Self {
        MutationLog::default()
    }

    /// Replay a single event, applying coalescing rules.
    pub fn record(&mut self, ev: MutationEvent) {
        match ev.kind {
            MutationKind::Create | MutationKind::Write => {
                self.state.insert(ev.path, MutationKind::Write);
            }
            MutationKind::Rename => {
                if let Some(from) = ev.from {
                    self.state.remove(&from);
                }
                self.state.insert(ev.path, MutationKind::Write);
            }
            MutationKind::Delete => {
                // If the file was created in this session, erase the net effect.
                self.state.remove(&ev.path);
            }
        }
    }

    /// Replay a batch of events.
    pub fn replay(&mut self, events: impl IntoIterator<Item = MutationEvent>) {
        for e in events {
            self.record(e);
        }
    }

    /// The resulting net mutation set (paths that persist with their kind).
    pub fn net_changes(&self) -> Vec<MutationEvent> {
        let mut out: Vec<MutationEvent> = self
            .state
            .iter()
            .map(|(p, k)| MutationEvent { kind: k.clone(), path: p.clone(), from: None })
            .collect();
        out.sort_by(|a, b| a.path.cmp(&b.path));
        out
    }

    pub fn len(&self) -> usize {
        self.state.len()
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }
}
