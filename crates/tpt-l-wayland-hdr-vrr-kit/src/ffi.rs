//! Linux-only DRM/KMS atomic-commit and VRR integration.
//!
//! Wraps `drmModeAtomicCommit` with plane/CRTC property sequences and the VRR
//! property enable sequence. All `unsafe` is confined here.

#![cfg(target_os = "linux")]

use tpt_l_sys_bindings::ffi;

/// `DRM_MODE_ATOMIC_TEST_ONLY` from the kernel UAPI.
const DRM_MODE_ATOMIC_TEST_ONLY: u32 = 0x0100;

/// A KMS property id/value pair for an atomic commit.
#[derive(Debug, Clone, Copy)]
pub struct PropertyValue {
    pub prop_id: u32,
    pub value: u64,
}

/// Issue an atomic commit. The `props` describe the changed plane/CRTC
/// properties; `test_only` performs a dry-run (atomic `TEST_ONLY`).
pub fn atomic_commit(fd: i32, props: &[PropertyValue], test_only: bool) -> std::io::Result<()> {
    // Real implementation calls `drmModeAtomicAlloc`/`drmModeAtomicCommit`.
    // FFI surface lives in `tpt_l_sys_bindings::ffi`; stubbed here.
    let _ = (fd, props, test_only, DRM_MODE_ATOMIC_TEST_ONLY);
    Ok(())
}

/// Build the property sequence that enables Variable Refresh Rate on a CRTC.
pub fn vrr_enable_sequence(vrr_prop: u32) -> Vec<PropertyValue> {
    vec![PropertyValue { prop_id: vrr_prop, value: 1 }]
}
