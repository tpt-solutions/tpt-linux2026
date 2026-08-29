//! Kernel-function mock framework.
//!
//! Registers user-supplied stubs for named kernel functions and dispatches calls
//! to them. This is the piece that makes `cargo test` kernel tests possible
//! without a live kernel; it is pure and platform-independent.

use std::collections::HashMap;
use std::sync::Arc;

/// A stub handler: receives the call arguments and returns a replacement value.
pub type Handler = Arc<dyn Fn(&[u64]) -> u64 + Send + Sync>;

/// Registry of mocked kernel functions.
#[derive(Clone, Default)]
pub struct MockRegistry {
    handlers: HashMap<String, Handler>,
}

impl MockRegistry {
    pub fn new() -> Self {
        MockRegistry::default()
    }

    /// Register a mock handler for `fn_name`.
    pub fn register<F>(&mut self, fn_name: impl Into<String>, f: F)
    where
        F: Fn(&[u64]) -> u64 + Send + Sync + 'static,
    {
        self.handlers.insert(fn_name.into(), Arc::new(f));
    }

    /// Invoke the mock for `fn_name`. Returns `None` if no mock is registered.
    pub fn invoke(&self, fn_name: &str, args: &[u64]) -> Option<u64> {
        self.handlers.get(fn_name).map(|h| h(args))
    }

    /// Whether a mock is registered for `fn_name`.
    pub fn is_mocked(&self, fn_name: &str) -> bool {
        self.handlers.contains_key(fn_name)
    }
}

/// A test fixture wrapping a [`MockRegistry`].
#[derive(Clone, Default)]
pub struct KprobeHarness {
    pub mocks: MockRegistry,
}

impl KprobeHarness {
    /// Create a fresh harness for a test.
    pub fn new() -> Self {
        KprobeHarness { mocks: MockRegistry::new() }
    }

    /// Register a kernel-function mock.
    pub fn mock<F>(&mut self, fn_name: impl Into<String>, f: F) -> &mut Self
    where
        F: Fn(&[u64]) -> u64 + Send + Sync + 'static,
    {
        self.mocks.register(fn_name, f);
        self
    }

    /// Simulate a call into the kernel; returns the mock's result, or `None`.
    pub fn call(&self, fn_name: &str, args: &[u64]) -> Option<u64> {
        self.mocks.invoke(fn_name, args)
    }
}
