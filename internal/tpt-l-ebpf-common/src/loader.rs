//! Safe wrappers over [Aya](https://aya-rs.dev) for loading eBPF objects.
//!
//! Available only on Linux targets. All `unsafe` is confined here.

use aya::Ebpf;
use aya::EbpfLoader;
use thiserror::Error;

/// Errors arising while loading or running an eBPF object.
#[derive(Debug, Error)]
pub enum EbpfError {
    #[error("failed to load eBPF object: {0}")]
    Load(#[from] aya::EbpfError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Load an eBPF object from its ELF bytes and return the loaded program set.
pub fn load(bytes: &[u8]) -> Result<Ebpf, EbpfError> {
    let bpf = EbpfLoader::new().load(bytes)?;
    Ok(bpf)
}

/// Load and immediately run `f` with the loaded eBPF object. `f` is responsible
/// for attaching programs (network/scheduler/LSM hooks) and driving any event
/// loops; this helper only manages the load lifecycle.
pub fn run<F, R>(bytes: &[u8], f: F) -> Result<R, EbpfError>
where
    F: FnOnce(&mut Ebpf) -> R,
{
    let mut bpf = load(bytes)?;
    Ok(f(&mut bpf))
}
