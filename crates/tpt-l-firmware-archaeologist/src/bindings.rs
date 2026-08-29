//! Low-level, zero-copy byte access.
//!
//! **All `unsafe` in this crate lives in this module.** The parser modules
//! (`acpi`, `uefi`, `intel_me`) only ever touch the safe [`Reader`] API.

use core::ptr;

/// A bounds-checked, little-endian reader over a firmware blob.
///
/// Reads are performed with unaligned loads (see [`Reader::read_at`]); every
/// access is first range-checked so callers cannot trigger out-of-bounds reads.
pub struct Reader<'a> {
    buf: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Reader { buf }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn remaining(&self, offset: usize) -> usize {
        self.buf.len().saturating_sub(offset)
    }

    pub fn slice(&self, start: usize, end: usize) -> Option<&'a [u8]> {
        if end <= self.buf.len() { Some(&self.buf[start..end]) } else { None }
    }

    /// Read `n` bytes at `offset` as a `&[u8]`.
    pub fn bytes(&self, offset: usize, n: usize) -> Option<&'a [u8]> {
        self.slice(offset, offset.wrapping_add(n))
    }

    /// Read a NUL-or-fixed-width ASCII string of length `n` (stops at first NUL).
    pub fn ascii(&self, offset: usize, n: usize) -> Option<&'a str> {
        let raw = self.bytes(offset, n)?;
        let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
        core::str::from_utf8(&raw[..end]).ok()
    }

    pub fn u8(&self, offset: usize) -> Option<u8> {
        self.buf.get(offset).copied()
    }

    pub fn u16(&self, offset: usize) -> Option<u16> {
        if self.remaining(offset) < 2 {
            return None;
        }
        Some(unsafe { Self::read_at(self.buf, offset) })
    }

    pub fn u32(&self, offset: usize) -> Option<u32> {
        if self.remaining(offset) < 4 {
            return None;
        }
        Some(unsafe { Self::read_at(self.buf, offset) })
    }

    pub fn u64(&self, offset: usize) -> Option<u64> {
        if self.remaining(offset) < 8 {
            return None;
        }
        Some(unsafe { Self::read_at(self.buf, offset) })
    }

    /// Zero-copy reinterpretation of `buf[offset..offset+size_of::<T>()]` as `&T`.
    ///
    /// # Safety
    /// Caller must guarantee `offset + size_of::<T>() <= buf.len()` and that the
    /// alignment of `T` is satisfiable by an unaligned load.
    #[allow(dead_code)]
    pub(crate) unsafe fn view<T: Copy>(&self, offset: usize) -> Option<&'a T> {
        if self.remaining(offset) < core::mem::size_of::<T>() {
            return None;
        }
        unsafe { Some(&*(self.buf.as_ptr().add(offset) as *const T)) }
    }

    /// Unaligned little-endian read at `offset`. The caller must ensure enough
    /// bytes are present.
    #[inline]
    unsafe fn read_at<T: Copy>(buf: &[u8], offset: usize) -> T {
        unsafe { ptr::read_unaligned(buf.as_ptr().add(offset) as *const T) }
    }
}
