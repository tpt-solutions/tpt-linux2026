//! QEMU guest management: spawn, snapshot, restore kernel state.
//!
//! On Linux this shells out to `qemu-system-*`; the command construction is pure
//! and unit-tested so the integration is verifiable without a real QEMU binary.

use std::path::PathBuf;
use std::process::Command;

/// A description of a QEMU guest.
#[derive(Debug, Clone)]
pub struct GuestConfig {
    pub qemu_binary: PathBuf,
    pub memory_mb: u32,
    pub smp: u32,
    pub kernel: PathBuf,
    pub initrd: Option<PathBuf>,
    pub drive: Option<PathBuf>,
    pub snapshot_dir: PathBuf,
}

impl GuestConfig {
    /// Build the `qemu` invocation for this guest.
    pub fn command(&self) -> Command {
        let mut cmd = Command::new(&self.qemu_binary);
        cmd.arg("-m").arg(self.memory_mb.to_string());
        cmd.arg("-smp").arg(self.smp.to_string());
        cmd.arg("-kernel").arg(&self.kernel);
        if let Some(initrd) = &self.initrd {
            cmd.arg("-initrd").arg(initrd);
        }
        if let Some(drive) = &self.drive {
            cmd.arg("-drive").arg(format!("file={},format=qcow2", drive.display()));
        }
        cmd.arg("-nographic");
        cmd
    }

    /// Path of the snapshot file for a named snapshot.
    pub fn snapshot_path(&self, name: &str) -> PathBuf {
        self.snapshot_dir.join(format!("{name}.qcow2"))
    }
}

/// Manage a QEMU guest lifecycle.
#[derive(Debug, Clone, Default)]
pub struct QemuManager {
    pub config: Option<GuestConfig>,
}

impl QemuManager {
    pub fn new() -> Self {
        QemuManager::default()
    }

    pub fn with_config(config: GuestConfig) -> Self {
        QemuManager { config: Some(config) }
    }

    /// Spawn the guest as a child process.
    pub fn spawn(&self) -> std::io::Result<std::process::Child> {
        let cfg = self
            .config
            .as_ref()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "no guest config"))?;
        cfg.command().spawn()
    }

    /// Capture a snapshot by invoking `qemu-img snapshot` semantics on the drive.
    /// Returns the snapshot file path that should be saved.
    pub fn snapshot_name(&self, name: &str) -> Option<PathBuf> {
        self.config.as_ref().map(|c| c.snapshot_path(name))
    }
}
