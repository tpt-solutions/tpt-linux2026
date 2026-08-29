#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tpt_l_sandbox_bpf::*;

    fn policy() -> Policy {
        Policy::new()
            .allow_path("/usr/bin")
            .deny_path("/usr/bin/evil")
            .allow_socket(SocketKind::Tcp)
            .deny_socket(SocketKind::Udp)
    }

    #[test]
    fn path_allow() {
        let p = policy();
        assert_eq!(p.evaluate_path(&PathBuf::from("/usr/bin/ls"), Access::Exec), Decision::Allow);
    }

    #[test]
    fn deny_takes_precedence() {
        let p = policy();
        assert_eq!(
            p.evaluate_path(&PathBuf::from("/usr/bin/evil/run"), Access::Exec),
            Decision::Deny
        );
    }

    #[test]
    fn unmatched_path_is_denied() {
        let p = policy();
        assert_eq!(p.evaluate_path(&PathBuf::from("/home/user/x"), Access::Read), Decision::Deny);
    }

    #[test]
    fn socket_rules() {
        let p = policy();
        assert_eq!(p.evaluate_socket(SocketKind::Tcp), Decision::Allow);
        assert_eq!(p.evaluate_socket(SocketKind::Udp), Decision::Deny);
    }
}
