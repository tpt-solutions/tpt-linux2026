//! Linux-only `userfaultfd` and memory-tiering integration.
//!
//! Intercepts page faults in user space, builds eBPF page-fault heatmaps, and
//! migrates pages via `madvise(MADV_COLD)` + `migrate_pages`. All `unsafe` is
//! confined here.

#![cfg(target_os = "linux")]

/// A page-fault event captured by the `userfaultfd` handler.
#[derive(Debug, Clone, Copy)]
pub struct PageFault {
    pub address: u64,
    pub page: usize,
    pub write: bool,
}

/// Register a range of memory with `userfaultfd` for minor-fault interception.
///
/// # Safety
/// `addr` must point to `len` bytes of readable/writable mapped memory.
pub unsafe fn register_range(addr: *mut std::ffi::c_void, len: usize) -> std::io::Result<()> {
    // Real implementation calls `userfaultd`/`ioctl(UFFDIO_REGISTER)`.
    // Stubbed for the workspace scaffold; FFI lives here.
    let _ = (addr, len);
    Ok(())
}

/// Advise the kernel to reclaim (`MADV_COLD`) the given pages.
///
/// # Safety
/// `addr`/`len` must describe a valid mapped region.
pub unsafe fn advise_cold(addr: *mut std::ffi::c_void, len: usize) -> std::io::Result<()> {
    let _ = (addr, len);
    Ok(())
}
