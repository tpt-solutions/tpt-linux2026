//! Cross-distro dependency resolution built on the CDCL solver.
//!
//! Models packages and their version alternatives as boolean variables and
//! encodes dependency / conflict constraints as CNF clauses. This is a
//! simplified but correct SAT-based dependency resolver suitable for
//! experimentation and as a backend for package managers.

use crate::literal::{Lit, Tri, Var};
use crate::solver::Solver;

/// Identifier of a package (independent of version).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PackageId(pub u32);

/// Identifier of a specific `(package, version)` alternative.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VersionId(pub u32);

/// A constraint failure.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ResolveError {
    #[error("the dependency constraints are unsatisfiable")]
    Unsatisfiable,
    #[error("package {0:?} has no versions")]
    NoVersions(PackageId),
}

/// A SAT-backed package dependency resolver.
pub struct PackageResolver {
    solver: Solver,
    packages: Vec<String>,
    versions: Vec<(PackageId, String)>,
    /// For each package, the set of version variable ids.
    pkg_versions: Vec<Vec<VersionId>>,
}

impl PackageResolver {
    pub fn new() -> Self {
        PackageResolver {
            solver: Solver::new(),
            packages: Vec::new(),
            versions: Vec::new(),
            pkg_versions: Vec::new(),
        }
    }

    /// Register a package by name. Returns its id.
    pub fn add_package(&mut self, name: &str) -> PackageId {
        let id = PackageId(self.packages.len() as u32);
        self.packages.push(name.to_string());
        self.pkg_versions.push(Vec::new());
        id
    }

    /// Add an installable `(package, version)` alternative. Returns its id.
    pub fn add_version(&mut self, pkg: PackageId, version: &str) -> VersionId {
        let id = VersionId(self.versions.len() as u32);
        self.versions.push((pkg, version.to_string()));
        self.pkg_versions[pkg.0 as usize].push(id);
        // The underlying SAT variable.
        self.solver.new_var();
        id
    }

    fn lit_of(&self, v: VersionId) -> Lit {
        Lit::new(Var(v.0), false)
    }

    /// At most one version of a package may be installed.
    pub fn enforce_unique_versions(&mut self) {
        let vers = self.pkg_versions.clone();
        for vs in vers {
            for i in 0..vs.len() {
                for j in (i + 1)..vs.len() {
                    // ¬a ∨ ¬b
                    self.solver
                        .add_clause(&[self.lit_of(vs[i]).negate(), self.lit_of(vs[j]).negate()]);
                }
            }
        }
    }

    /// Package `pkg` (any version) requires that dependency `dep` be satisfied
    /// by at least one of `allowed` versions.
    pub fn require_dependency(&mut self, pkg: VersionId, _dep: PackageId, allowed: &[VersionId]) {
        if allowed.is_empty() {
            return;
        }
        let mut clause: Vec<Lit> = allowed.iter().map(|&v| self.lit_of(v)).collect();
        clause.insert(0, self.lit_of(pkg).negate());
        self.solver.add_clause(&clause);
    }

    /// Two version alternatives may not be installed simultaneously.
    pub fn conflict(&mut self, a: VersionId, b: VersionId) {
        self.solver.add_clause(&[self.lit_of(a).negate(), self.lit_of(b).negate()]);
    }

    /// Force a particular version to be installed (a user request / lockfile).
    pub fn request(&mut self, v: VersionId) {
        self.solver.add_clause(&[self.lit_of(v)]);
    }

    /// Solve the constraints. Returns the set of installed `(package, version)`.
    pub fn solve(&mut self) -> Result<Vec<(String, String)>, ResolveError> {
        if !self.solver.solve() {
            return Err(ResolveError::Unsatisfiable);
        }
        let mut out = Vec::new();
        for (i, (pkg, ver)) in self.versions.iter().enumerate() {
            if self.solver.value_of(Lit::new(Var(i as u32), false)) == Tri::True {
                out.push((self.packages[pkg.0 as usize].clone(), ver.clone()));
            }
        }
        Ok(out)
    }
}

impl Default for PackageResolver {
    fn default() -> Self {
        Self::new()
    }
}
