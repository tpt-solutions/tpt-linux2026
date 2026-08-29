//! Platform-independent macros shared across eBPF crates.

/// Define the canonical name of an eBPF program in the workspace. The name is
/// used for logging, perf-buffer tagging and map pin paths.
///
/// ```ignore
/// program_name!(volt_orchestrator);
/// ```
#[macro_export]
macro_rules! program_name {
    ($name:ident) => {
        /// The canonical eBPF program name for this crate.
        pub const PROGRAM_NAME: &str = stringify!($name);
    };
}

/// Default capacity for a per-CPU hash map sized for typical fleet workloads.
pub const DEFAULT_MAP_SIZE: u32 = 8192;

/// Default perf-buffer page count (must be a power of two).
pub const DEFAULT_PERF_PAGES: usize = 64;
