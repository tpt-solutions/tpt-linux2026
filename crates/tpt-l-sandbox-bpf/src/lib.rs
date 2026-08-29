//! # tpt-l-sandbox-bpf
//!
//! A lightweight sandbox built on LSM BPF. Filesystem and network namespaces
//! are enforced with allow/deny lists. The safe [`Policy`] builder and the
//! [`Policy::evaluate_*`] decision logic are pure and unit-tested; the actual
//! LSM BPF attach lives in [`ffi`] (Linux only).

use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
pub mod ffi;

/// Access mode for a filesystem path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Read,
    Write,
    Exec,
}

/// Socket family/type descriptor for network rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketKind {
    Tcp,
    Udp,
    Unix,
}

/// A single sandbox rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    /// Allow reading/writing/executing under this path prefix.
    PathAllow(PathBuf),
    /// Deny reading/writing/executing under this path prefix (wins over allow).
    PathDeny(PathBuf),
    /// Allow a socket kind.
    SocketAllow(SocketKind),
    /// Deny a socket kind.
    SocketDeny(SocketKind),
}

/// The outcome of a policy evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny,
}

/// A compiled sandbox policy.
#[derive(Debug, Clone, Default)]
pub struct Policy {
    rules: Vec<Rule>,
}

impl Policy {
    pub fn new() -> Self {
        Policy::default()
    }

    /// Builder: allow a path prefix.
    pub fn allow_path(mut self, p: impl Into<PathBuf>) -> Self {
        self.rules.push(Rule::PathAllow(p.into()));
        self
    }

    /// Builder: deny a path prefix.
    pub fn deny_path(mut self, p: impl Into<PathBuf>) -> Self {
        self.rules.push(Rule::PathDeny(p.into()));
        self
    }

    /// Builder: allow a socket kind.
    pub fn allow_socket(mut self, k: SocketKind) -> Self {
        self.rules.push(Rule::SocketAllow(k));
        self
    }

    /// Builder: deny a socket kind.
    pub fn deny_socket(mut self, k: SocketKind) -> Self {
        self.rules.push(Rule::SocketDeny(k));
        self
    }

    fn under(prefix: &Path, path: &Path) -> bool {
        path.starts_with(prefix)
    }

    /// Evaluate a filesystem access. Deny rules take precedence over allow rules.
    pub fn evaluate_path(&self, path: &Path, _mode: Access) -> Decision {
        let mut allowed = false;
        for rule in &self.rules {
            match rule {
                Rule::PathAllow(p) if Self::under(p, path) => allowed = true,
                Rule::PathDeny(p) if Self::under(p, path) => return Decision::Deny,
                _ => {}
            }
        }
        if allowed { Decision::Allow } else { Decision::Deny }
    }

    /// Evaluate a socket creation request.
    pub fn evaluate_socket(&self, kind: SocketKind) -> Decision {
        let mut allowed = false;
        for rule in &self.rules {
            match rule {
                Rule::SocketAllow(k) if *k == kind => allowed = true,
                Rule::SocketDeny(k) if *k == kind => return Decision::Deny,
                _ => {}
            }
        }
        if allowed { Decision::Allow } else { Decision::Deny }
    }
}
