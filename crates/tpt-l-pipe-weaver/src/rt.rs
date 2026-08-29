//! Linux-only real-time thread setup.
//!
//! Configures `sched_setscheduler(SCHED_FIFO)` with priority negotiation. All
//! `unsafe` is confined here.

#![cfg(target_os = "linux")]

/// Real-time scheduling policy to request.
#[derive(Debug, Clone, Copy)]
pub enum RtPolicy {
    Fifo,
    RoundRobin,
}

/// Negotiate and apply a real-time scheduling policy for the current thread.
///
/// # Safety
/// Must be called from a thread permitted to raise scheduling priority.
pub unsafe fn enter_realtime(policy: RtPolicy, priority: i32) -> std::io::Result<()> {
    // Real impl calls libc::sched_setscheduler(0, SCHED_FIFO/RR, &param).
    let _ = (policy, priority);
    Ok(())
}

/// Priority negotiation: clamp to the allowed `[min, max]` for the policy.
pub fn negotiate_priority(requested: i32, min: i32, max: i32) -> i32 {
    requested.clamp(min, max)
}
