#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tpt_l_drift_trace::generate::{generate_guix, generate_nix};
    use tpt_l_drift_trace::log::{MutationEvent, MutationKind, MutationLog};

    fn ev(kind: MutationKind, path: &str) -> MutationEvent {
        MutationEvent { kind, path: PathBuf::from(path), from: None }
    }

    #[test]
    fn create_then_delete_coalesces() {
        let mut log = MutationLog::new();
        log.record(ev(MutationKind::Create, "/etc/foo"));
        log.record(ev(MutationKind::Write, "/etc/foo"));
        log.record(ev(MutationKind::Delete, "/etc/foo"));
        assert!(log.is_empty(), "create+write+delete => no net change");
    }

    #[test]
    fn write_persists() {
        let mut log = MutationLog::new();
        log.record(ev(MutationKind::Write, "/etc/bar"));
        assert_eq!(log.len(), 1);
        let net = log.net_changes();
        assert_eq!(net[0].path, PathBuf::from("/etc/bar"));
        assert_eq!(net[0].kind, MutationKind::Write);
    }

    #[test]
    fn rename_moves_state() {
        let mut log = MutationLog::new();
        let mut e = ev(MutationKind::Rename, "/etc/new");
        e.from = Some(PathBuf::from("/etc/old"));
        log.record(e);
        let net = log.net_changes();
        assert_eq!(net.len(), 1);
        assert_eq!(net[0].path, PathBuf::from("/etc/new"));
    }

    #[test]
    fn nix_generation() {
        let mut log = MutationLog::new();
        log.record(ev(MutationKind::Write, "/etc/x"));
        let expr = generate_nix("myhost", &log.net_changes());
        assert!(expr.contains("myhost"));
        assert!(expr.contains("/etc/x"));
        assert!(expr.contains("writeText"));
    }

    #[test]
    fn guix_generation() {
        let mut log = MutationLog::new();
        log.record(ev(MutationKind::Write, "/etc/y"));
        let expr = generate_guix("myhost", &log.net_changes());
        assert!(expr.contains("define-public myhost"));
        assert!(expr.contains("local-file \"/etc/y\""));
    }
}
