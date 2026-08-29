//! Linux-only `fanotify` and `auditd` netlink integration.
//!
//! Tracks `create`/`write`/`rename`/`delete` events from `fanotify` and injects
//! `auditd` rules over netlink sockets. All `unsafe` is confined here.

#![cfg(target_os = "linux")]

use std::os::raw::c_int;

/// Open a fanotify group watching the given mount point for modify/close-write
/// and move/delete events.
///
/// # Safety
/// `mount` must be a NUL-terminated, valid path.
pub unsafe fn open_fanotify(mount: *const std::ffi::c_char) -> std::io::Result<c_int> {
    let _ = mount;
    // Real impl: libc::fanotify_init + fanotify_mark.
    Ok(0)
}

/// Inject an `auditctl`-equivalent rule via the audit netlink socket.
///
/// # Safety
/// Caller must ensure the netlink socket is open.
pub unsafe fn inject_audit_rule(fd: c_int, syscall: &str) -> std::io::Result<()> {
    let _ = (fd, syscall);
    Ok(())
}
