//! # tpt-l-sat-solver
//!
//! A from-scratch Conflict-Driven Clause Learning (CDCL) SAT solver written in
//! safe Rust, plus a [`PackageResolver`] for cross-distro dependency
//! resolution expressed as boolean satisfiability.
//!
//! ```rust
//! use tpt_l_sat_solver::{SatSolver, Lit, Var};
//!
//! let mut s = SatSolver::new();
//! let x = s.new_var();
//! let y = s.new_var();
//! // (x OR y) AND (NOT x OR y)  =>  y must be true, x free
//! s.add_clause(&[Lit::new(x, false), Lit::new(y, false)]);
//! s.add_clause(&[Lit::new(x, true), Lit::new(y, false)]);
//! assert!(s.solve());
//! assert_eq!(s.value(Lit::new(y, false)), Some(true));
//! ```
//!
//! ## Public API
//!
//! - [`SatSolver`] — the low-level CDCL solver (`new_var`, `add_clause`,
//!   `solve`, `value`, `model`).
//! - [`PackageResolver`] — high-level dependency resolution over packages and
//!   version alternatives.
//!
//! Re-exported literal primitives: [`Var`], [`Lit`], [`Tri`].

mod distro;
mod literal;
mod solver;

pub use distro::{PackageId, PackageResolver, ResolveError, VersionId};
pub use literal::{Lit, Tri, Var};

/// A low-level CDCL SAT solver.
///
/// Variables are created with [`new_var`](SatSolver::new_var) and clauses with
/// [`add_clause`](SatSolver::add_clause). Call [`solve`](SatSolver::solve) and
/// inspect the resulting assignment with [`value`](SatSolver::value) or
/// [`model`](SatSolver::model).
pub struct SatSolver {
    inner: solver::Solver,
}

impl SatSolver {
    /// Create an empty solver with no variables.
    pub fn new() -> Self {
        SatSolver { inner: solver::Solver::new() }
    }

    /// Allocate a fresh boolean variable and return its id.
    pub fn new_var(&mut self) -> Var {
        self.inner.new_var()
    }

    /// Add a clause (disjunction) of literals. Returns `false` if the formula
    /// is already known to be unsatisfiable.
    pub fn add_clause(&mut self, lits: &[Lit]) -> bool {
        self.inner.add_clause(lits)
    }

    /// Solve the formula. `true` means satisfiable (a model exists).
    pub fn solve(&mut self) -> bool {
        self.inner.solve()
    }

    /// Value of `lit` in the current assignment, if assigned.
    pub fn value(&self, lit: Lit) -> Option<bool> {
        match self.inner.value_of(lit) {
            Tri::True => Some(true),
            Tri::False => Some(false),
            Tri::Undef => None,
        }
    }

    /// The full variable assignment (variables only; polarity per variable).
    pub fn model(&self) -> Vec<(Var, bool)> {
        self.inner.model()
    }

    /// Debug helper: verify all clauses are satisfied by the current assignment.
    pub fn verify(&self) -> bool {
        self.inner.verify()
    }

    /// Debug helper: index of first violated clause, if any.
    pub fn first_violated(&self) -> Option<usize> {
        self.inner.first_violated()
    }

    /// Debug helper: DIMACS literals of the first violated clause, if any.
    pub fn debug_violated_lits(&self) -> Option<Vec<i32>> {
        self.inner.debug_violated_lits()
    }
}

impl Default for SatSolver {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export the shared primitive types for downstream crates and bindings.
pub use tpt_l_math_core::sat::{Clause, Cnf, Literal, Variable};
