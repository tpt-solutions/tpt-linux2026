//! Internal literal / variable representation for the CDCL solver.
//!
//! Encoding follows MiniSat: a literal is `2 * var + sign`, where `sign` is
//! `0` for positive and `1` for negative. A variable is a `u32`. This packs
//! `var`/`sign` into a single integer so watched-literal maps can be indexed
//! by `lit.index()`.

/// A boolean variable, zero-based.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Var(pub u32);

/// A literal: variable plus polarity, packed as `2 * var + sign`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Lit(pub u32);

impl Lit {
    #[inline]
    pub fn new(var: Var, sign: bool) -> Self {
        Lit((var.0 << 1) | (sign as u32))
    }

    #[inline]
    pub fn var(self) -> Var {
        Var(self.0 >> 1)
    }

    /// `true` if positive polarity.
    #[inline]
    pub fn is_pos(self) -> bool {
        (self.0 & 1) == 0
    }

    #[inline]
    pub fn sign(self) -> bool {
        (self.0 & 1) == 1
    }

    /// Negated literal.
    #[inline]
    pub fn negate(self) -> Lit {
        Lit(self.0 ^ 1)
    }

    /// Dense index used for watch lists and assignment tables.
    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }

    /// DIMACS signed integer (`var + 1`, negated for negative polarity).
    #[inline]
    pub fn to_dimacs(self) -> i32 {
        let v = (self.var().0 as i32) + 1;
        if self.is_pos() { v } else { -v }
    }

    #[inline]
    pub fn from_dimacs(x: i32) -> Self {
        debug_assert!(x != 0);
        let var = Var((x.abs() - 1) as u32);
        Lit::new(var, x < 0)
    }
}

/// Truth value of an assignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tri {
    True,
    False,
    Undef,
}

impl Tri {
    #[inline]
    pub fn from_bool(b: bool) -> Tri {
        if b { Tri::True } else { Tri::False }
    }
    #[inline]
    pub fn is_undef(self) -> bool {
        self == Tri::Undef
    }
}
