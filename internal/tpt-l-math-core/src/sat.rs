//! Low-level boolean satisfiability primitives.
//!
//! These types are intentionally minimal and allocation-light. They form the
//! vocabulary shared between the workspace-wide SAT helpers and the full CDCL
//! solver in `tpt-l-sat-solver`.

use alloc::vec::Vec;

/// A boolean variable, identified by a zero-based index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Variable(pub u32);

impl Variable {
    /// The positive literal of this variable.
    #[inline]
    pub fn positive(self) -> Literal {
        Literal::new(self, true)
    }

    /// The negative literal of this variable.
    #[inline]
    pub fn negative(self) -> Literal {
        Literal::new(self, false)
    }
}

/// A literal: a variable with a polarity.
///
/// Internally encoded in DIMACS convention: a positive literal `v` is `v + 1`,
/// a negative literal `v` is `-(v + 1)`. Variable index `0` is therefore
/// represented as `1` / `-1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal(i32);

impl Literal {
    /// Construct a literal for `var` with the given polarity.
    ///
    /// # Panics
    /// Panics if `var.0 > i32::MAX as u32 - 1` (practically unreachable).
    #[inline]
    pub fn new(var: Variable, positive: bool) -> Self {
        let v = (var.0 as i32) + 1;
        Literal(if positive { v } else { -v })
    }

    /// Decode a DIMACS integer into a literal.
    #[inline]
    pub fn from_dimacs(x: i32) -> Self {
        debug_assert!(x != 0, "literal 0 is not valid");
        Literal(x)
    }

    /// The underlying variable.
    #[inline]
    pub fn variable(self) -> Variable {
        Variable((self.0.abs() - 1) as u32)
    }

    /// Whether the literal is the positive polarity.
    #[inline]
    pub fn is_positive(self) -> bool {
        self.0 > 0
    }

    /// Whether the literal is the negative polarity.
    #[inline]
    pub fn is_negative(self) -> bool {
        self.0 < 0
    }

    /// The negated literal.
    #[inline]
    pub fn negate(self) -> Literal {
        Literal(-self.0)
    }

    /// Encode as a DIMACS integer.
    #[inline]
    pub fn to_dimacs(self) -> i32 {
        self.0
    }
}

/// A disjunction of literals (a clause).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Clause(pub Vec<Literal>);

impl Clause {
    /// Build a clause from a list of literals.
    pub fn new(lits: impl IntoIterator<Item = Literal>) -> Self {
        Clause(lits.into_iter().collect())
    }

    /// Number of literals.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the clause is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterator over the literals.
    #[inline]
    pub fn literals(&self) -> core::slice::Iter<'_, Literal> {
        self.0.iter()
    }
}

/// A conjunction of clauses (a CNF formula).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Cnf(pub Vec<Clause>);

impl Cnf {
    /// Build a CNF from clauses.
    pub fn new(clauses: impl IntoIterator<Item = Clause>) -> Self {
        Cnf(clauses.into_iter().collect())
    }

    /// Number of clauses.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the formula has no clauses.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The number of distinct variables referenced, assuming contiguous
    /// zero-based indexing. Returns `max_var + 1`.
    pub fn num_vars(&self) -> u32 {
        self.0.iter().flat_map(|c| c.0.iter()).map(|l| l.variable().0).max().map_or(0, |m| m + 1)
    }
}
