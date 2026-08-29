//! # tpt-l-pipe-weaver
//!
//! A pro-audio DSP graph builder. The real-time data path uses lock-free ring
//! buffers ([`crossbeam`]); the actual PipeWire C API integration lives in
//! [`ffi`] (Linux only), and the real-time thread setup (`SCHED_FIFO`) is in
//! [`rt`]. The safe graph model in [`graph`] is platform-independent and tested.

use crossbeam::queue::ArrayQueue;

#[cfg(target_os = "linux")]
pub mod ffi;
#[cfg(target_os = "linux")]
pub mod rt;

pub mod graph;

/// A single audio sample frame (interleaved not assumed; one f32 per channel
/// element in the ring).
pub type Sample = f32;

/// Errors from the DSP graph.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("node {0} not found")]
    NoNode(u32),
    #[error("port {0} not found on node {1}")]
    NoPort(u32, u32),
    #[error("connection would form a cycle")]
    Cycle,
    #[error("ring buffer full")]
    Full,
    #[error("ring buffer empty")]
    Empty,
}

/// A connection carries samples through a bounded, lock-free ring buffer.
#[derive(Debug)]
pub struct Connection {
    pub from_node: u32,
    pub from_port: u32,
    pub to_node: u32,
    pub to_port: u32,
    ring: ArrayQueue<Sample>,
}

impl Connection {
    pub fn new(
        from_node: u32,
        from_port: u32,
        to_node: u32,
        to_port: u32,
        capacity: usize,
    ) -> Self {
        Connection {
            from_node,
            from_port,
            to_node,
            to_port,
            ring: ArrayQueue::new(capacity.max(1)),
        }
    }

    /// Push a sample into the connection's ring (lock-free).
    pub fn push(&self, s: Sample) -> Result<(), Error> {
        self.ring.push(s).map_err(|_| Error::Full)
    }

    /// Pop a sample from the connection's ring (lock-free).
    pub fn pop(&self) -> Result<Sample, Error> {
        self.ring.pop().ok_or(Error::Empty)
    }

    pub fn len(&self) -> usize {
        self.ring.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }
}
