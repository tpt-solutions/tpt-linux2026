#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tpt_l_kprobe_harness::mock::{KprobeHarness, MockRegistry};
    use tpt_l_kprobe_harness::qemu::{GuestConfig, QemuManager};

    #[test]
    fn mock_invocation() {
        let mut reg = MockRegistry::new();
        reg.register("schedule", |args| args[0].wrapping_add(1));
        assert_eq!(reg.invoke("schedule", &[41]), Some(42));
        assert_eq!(reg.invoke("other", &[1]), None);
        assert!(reg.is_mocked("schedule"));
    }

    #[test]
    fn harness_as_fixture() {
        let mut h = KprobeHarness::new();
        h.mock("vfs_read", |args| {
            // Pretend reads of fd 3 always return 0 bytes.
            if args[0] == 3 { 0 } else { 64 }
        });
        assert_eq!(h.call("vfs_read", &[3]), Some(0));
        assert_eq!(h.call("vfs_read", &[7]), Some(64));
    }

    #[test]
    fn qemu_command_construction() {
        let cfg = GuestConfig {
            qemu_binary: PathBuf::from("qemu-system-x86_64"),
            memory_mb: 2048,
            smp: 4,
            kernel: PathBuf::from("/boot/vmlinuz"),
            initrd: Some(PathBuf::from("/boot/initrd")),
            drive: Some(PathBuf::from("/img/root.qcow2")),
            snapshot_dir: PathBuf::from("/snap"),
        };
        let mgr = QemuManager::with_config(cfg);
        let snap = mgr.snapshot_name("boot-state").unwrap();
        assert_eq!(snap, PathBuf::from("/snap/boot-state.qcow2"));
    }
}
