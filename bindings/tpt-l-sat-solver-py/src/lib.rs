//! Python bindings for `tpt-l-sat-solver`.
//!
//! Exposes a `SatSolver` class and a `solve_cnf` helper. Literals are encoded as
//! signed integers in DIMACS convention (positive = variable, negative = its
//! negation).

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tpt_l_sat_solver::{Lit, SatSolver as Inner, Var};

#[pyclass]
struct SatSolver {
    inner: Inner,
}

#[pymethods]
impl SatSolver {
    #[new]
    fn new() -> Self {
        SatSolver { inner: Inner::new() }
    }

    /// Allocate a fresh boolean variable; returns its (1-based) index.
    fn new_var(&mut self) -> usize {
        let v: Var = self.inner.new_var();
        v.0 as usize + 1
    }

    /// Add a clause given as a list of DIMACS literals (e.g. `[1, -2, 3]`).
    fn add_clause(&mut self, lits: Vec<i32>) -> PyResult<bool> {
        let clause: Vec<Lit> = lits
            .into_iter()
            .map(|x| {
                if x == 0 {
                    return Err(PyValueError::new_err("literal 0 is not allowed"));
                }
                Ok(Lit::from_dimacs(x))
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(self.inner.add_clause(&clause))
    }

    /// Solve the formula. Returns `true` if satisfiable.
    fn solve(&mut self) -> bool {
        self.inner.solve()
    }

    /// Value of the given DIMACS literal in the model, or `None` if unassigned.
    fn value(&self, lit: i32) -> PyResult<Option<bool>> {
        if lit == 0 {
            return Err(PyValueError::new_err("literal 0 is not allowed"));
        }
        Ok(self.inner.value(Lit::from_dimacs(lit)))
    }

    /// The satisfying assignment, as a list of DIMACS literals.
    fn model(&self) -> Vec<i32> {
        self.inner.model().into_iter().map(|(v, value)| Lit::new(v, value).to_dimacs()).collect()
    }
}

/// Convenience: solve a CNF given as a list of clauses (each a list of DIMACS
/// literals). Returns the model as a list of literals, or `None` if UNSAT.
#[pyfunction]
fn solve_cnf(clauses: Vec<Vec<i32>>) -> PyResult<Option<Vec<i32>>> {
    let mut s = SatSolver::new();
    for clause in clauses {
        s.add_clause(clause)?;
    }
    if s.inner.solve() { Ok(Some(s.model())) } else { Ok(None) }
}

#[pymodule]
fn tpt_l_sat_solver_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SatSolver>()?;
    m.add_function(wrap_pyfunction!(solve_cnf, m)?)?;
    Ok(())
}
