//! # tpt-l-pager-thermo
//!
//! Intelligent memory tiering. On Linux this uses `userfaultfd` and an eBPF
//! page-fault tracer (see [`ffi`]) to build access heatmaps and migrate hot/cold
//! pages between RAM and NVMe swap via `madvise(MADV_COLD)` +
//! `migrate_pages`. The safe, testable [`PageHeatmap`] and [`TieringPolicy`]
//! logic is platform-independent.

use std::collections::HashMap;

#[cfg(target_os = "linux")]
pub mod ffi;

/// Errors from the tiering engine.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("page index out of range: {0}")]
    OutOfRange(usize),
}

/// A fixed-range access heatmap over pages. Records per-page access counts.
#[derive(Debug, Clone)]
pub struct PageHeatmap {
    pages: Vec<u64>,
}

impl PageHeatmap {
    pub fn new(num_pages: usize) -> Self {
        PageHeatmap { pages: vec![0u64; num_pages] }
    }

    /// Record one access to `page`.
    pub fn record_access(&mut self, page: usize) -> Result<(), Error> {
        let e = self.pages.get_mut(page).ok_or(Error::OutOfRange(page))?;
        *e += 1;
        Ok(())
    }

    /// Access count for `page`.
    pub fn accesses(&self, page: usize) -> Option<u64> {
        self.pages.get(page).copied()
    }

    /// Pages whose access count meets or exceeds `threshold`.
    pub fn hot_pages(&self, threshold: u64) -> Vec<usize> {
        self.pages.iter().enumerate().filter(|&(_, &c)| c >= threshold).map(|(i, _)| i).collect()
    }

    /// Pages whose access count is below `threshold` (cold / tier-down
    /// candidates).
    pub fn cold_pages(&self, threshold: u64) -> Vec<usize> {
        self.pages.iter().enumerate().filter(|&(_, &c)| c < threshold).map(|(i, _)| i).collect()
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }
}

/// Sparse heatmap variant for workloads where the working set is scattered.
#[derive(Debug, Clone, Default)]
pub struct SparseHeatmap {
    pages: HashMap<usize, u64>,
}

impl SparseHeatmap {
    pub fn record_access(&mut self, page: usize) {
        *self.pages.entry(page).or_insert(0) += 1;
    }

    pub fn accesses(&self, page: usize) -> u64 {
        self.pages.get(&page).copied().unwrap_or(0)
    }
}

/// Decision policy: a minimum hot threshold above which pages are promoted to
/// fast storage, and a cold threshold below which they are demoted.
#[derive(Debug, Clone, Copy)]
pub struct TieringPolicy {
    pub hot_threshold: u64,
    pub cold_threshold: u64,
}

impl TieringPolicy {
    pub fn new(hot_threshold: u64, cold_threshold: u64) -> Self {
        TieringPolicy { hot_threshold, cold_threshold }
    }

    /// Classify a page's access count into a tiering action.
    pub fn classify(&self, accesses: u64) -> Tier {
        if accesses >= self.hot_threshold {
            Tier::Hot
        } else if accesses <= self.cold_threshold {
            Tier::Cold
        } else {
            Tier::Neutral
        }
    }
}

/// A page's relative temperature tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Hot,
    Neutral,
    Cold,
}
