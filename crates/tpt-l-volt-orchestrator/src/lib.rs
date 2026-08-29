//! # tpt-l-volt-orchestrator
//!
//! Event-driven CPU power management. On Linux this hooks into scheduler-tick and
//! interrupt traces via eBPF (see [`ffi`]) and drives `cpufreq` sysfs without
//! polling. The safe, mockable [`CpufreqInterface`] and the [`Orchestrator`]
//! decision logic are platform-independent and unit-tested.
//!
//! ```rust
//! use tpt_l_volt_orchestrator::CpufreqInterface;
//! use std::collections::HashMap;
//!
//! // Inject a fake sysfs root.
//! let mut fs = HashMap::new();
//! fs.insert("cpu0/cpufreq/scaling_governor".into(), "powersave".into());
//! fs.insert("cpu0/cpufreq/scaling_max_freq".into(), "2400000".into());
//! let iface = CpufreqInterface::with_store(fs);
//! assert_eq!(iface.governor(0).unwrap(), "powersave");
//! ```

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "linux")]
pub mod ffi;

/// Errors raised while talking to `cpufreq` sysfs.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cpu {0} not found")]
    NoCpu(usize),
    #[error("sysfs read/write error: {0}")]
    Sysfs(String),
}

/// Abstraction over the `cpufreq` sysfs layout. The default implementation
/// reads `/sys/devices/system/cpu`, but [`CpufreqInterface::with_store`] lets
/// tests inject a fake store.
#[derive(Clone, Default)]
pub struct CpufreqInterface {
    base: String,
    store: Option<Arc<CpufreqStore>>,
}

#[derive(Default)]
struct CpufreqStore {
    files: Mutex<HashMap<String, String>>,
}

impl CpufreqInterface {
    /// Use the real Linux sysfs root.
    pub fn new() -> Self {
        CpufreqInterface { base: "/sys/devices/system/cpu".to_string(), store: None }
    }

    /// Use an in-memory store (for tests / simulation).
    pub fn with_store(files: HashMap<String, String>) -> Self {
        CpufreqInterface {
            base: String::new(),
            store: Some(Arc::new(CpufreqStore { files: Mutex::new(files) })),
        }
    }

    fn path(&self, cpu: usize, attr: &str) -> String {
        format!("cpu{cpu}/cpufreq/{attr}")
    }

    fn read(&self, cpu: usize, attr: &str) -> Result<String, Error> {
        if let Some(s) = &self.store {
            let files = s.files.lock().unwrap();
            return files.get(&self.path(cpu, attr)).cloned().ok_or(Error::NoCpu(cpu));
        }
        let full = format!("{}/{}", self.base, self.path(cpu, attr));
        std::fs::read_to_string(&full).map_err(|e| Error::Sysfs(e.to_string()))
    }

    fn write(&self, cpu: usize, attr: &str, value: &str) -> Result<(), Error> {
        if let Some(s) = &self.store {
            let mut files = s.files.lock().unwrap();
            files.insert(self.path(cpu, attr), value.to_string());
            return Ok(());
        }
        let full = format!("{}/{}", self.base, self.path(cpu, attr));
        std::fs::write(&full, value).map_err(|e| Error::Sysfs(e.to_string()))
    }

    /// Current scaling governor for `cpu`.
    pub fn governor(&self, cpu: usize) -> Result<String, Error> {
        self.read(cpu, "scaling_governor")
    }

    /// Set the scaling governor for `cpu`.
    pub fn set_governor(&self, cpu: usize, gov: &str) -> Result<(), Error> {
        self.write(cpu, "scaling_governor", gov)
    }

    /// Maximum allowed frequency (kHz) for `cpu`.
    pub fn max_freq(&self, cpu: usize) -> Result<u64, Error> {
        self.read(cpu, "scaling_max_freq")?
            .trim()
            .parse()
            .map_err(|_| Error::Sysfs("bad freq".into()))
    }

    /// Set the maximum frequency (kHz) for `cpu`.
    pub fn set_max_freq(&self, cpu: usize, khz: u64) -> Result<(), Error> {
        self.write(cpu, "scaling_max_freq", &khz.to_string())
    }
}

/// A simple load-driven decision: high load -> performance governor + boost
/// clocks; low load -> powersave. Pure and unit-testable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    Powersave,
    Performance,
}

impl Policy {
    /// Decide a policy from a normalized load in `[0.0, 1.0]`.
    pub fn from_load(load: f64) -> Policy {
        if load >= 0.7 { Policy::Performance } else { Policy::Powersave }
    }

    pub fn governor(&self) -> &'static str {
        match self {
            Policy::Powersave => "powersave",
            Policy::Performance => "performance",
        }
    }
}

/// Apply a policy to a range of CPUs via the given interface.
pub fn apply_policy(
    iface: &CpufreqInterface,
    cpus: std::ops::Range<usize>,
    p: Policy,
) -> Result<(), Error> {
    for cpu in cpus {
        iface.set_governor(cpu, p.governor())?;
    }
    Ok(())
}
