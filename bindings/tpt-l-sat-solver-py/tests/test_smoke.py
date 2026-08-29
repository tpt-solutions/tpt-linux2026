"""Smoke tests for the tpt-l-sat-solver Python bindings."""

from tpt_l_sat_solver_py import SatSolver, solve_cnf


def test_unit_clause():
    s = SatSolver()
    x = s.new_var()
    s.add_clause([x])
    assert s.solve() is True
    assert s.value(x) is True


def test_unsat():
    s = SatSolver()
    x = s.new_var()
    s.add_clause([x])
    s.add_clause([-x])
    assert s.solve() is False


def test_solve_cnf():
    # (x OR y) AND (NOT x OR y) => y must be true
    model = solve_cnf([[1, 2], [-1, 2]])
    assert model is not None
    assert 2 in model
