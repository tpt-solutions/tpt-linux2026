#[cfg(test)]
mod tests {
    use tpt_l_sat_solver::{Lit, SatSolver, Var};

    #[test]
    fn trivial_sat() {
        let mut s = SatSolver::new();
        let x = s.new_var();
        s.add_clause(&[Lit::new(x, false)]);
        assert!(s.solve());
        assert_eq!(s.value(Lit::new(x, false)), Some(true));
    }

    #[test]
    fn trivial_unsat() {
        let mut s = SatSolver::new();
        let x = s.new_var();
        s.add_clause(&[Lit::new(x, false)]);
        s.add_clause(&[Lit::new(x, true)]);
        assert!(!s.solve());
    }

    #[test]
    fn unit_propagation() {
        let mut s = SatSolver::new();
        let x = s.new_var();
        let y = s.new_var();
        // (x) AND (¬x ∨ y)  => x true, y true
        s.add_clause(&[Lit::new(x, false)]);
        s.add_clause(&[Lit::new(x, true), Lit::new(y, false)]);
        assert!(s.solve());
        assert_eq!(s.value(Lit::new(y, false)), Some(true));
    }

    #[test]
    fn pigeonhole_2_3_unsat() {
        // 3 pigeons into 2 holes, each pigeon in exactly one hole,
        // each hole at most one pigeon -> UNSAT.
        let p = 3;
        let h = 2;
        let mut s = SatSolver::new();
        let mut vars: Vec<Vec<Var>> = Vec::with_capacity(p);
        for _ in 0..p {
            let row: Vec<Var> = (0..h).map(|_| s.new_var()).collect();
            vars.push(row);
        }
        // each pigeon in at least one hole
        for row in &vars {
            let mut clause = Vec::new();
            for &var in row {
                clause.push(Lit::new(var, false));
            }
            s.add_clause(&clause);
        }
        // each pigeon in at most one hole
        for row in &vars {
            for j in 0..h {
                for k in (j + 1)..h {
                    s.add_clause(&[Lit::new(row[j], true), Lit::new(row[k], true)]);
                }
            }
        }
        // each hole at most one pigeon
        for j in 0..h {
            for (i, row_i) in vars.iter().enumerate() {
                for row_k in vars.iter().skip(i + 1) {
                    s.add_clause(&[Lit::new(row_i[j], true), Lit::new(row_k[j], true)]);
                }
            }
        }
        let sat = s.solve();
        if sat {
            let violated = s.debug_violated_lits();
            let model = s.model();
            panic!(
                "solver returned SAT with invalid model; violated clause (dimacs) = {:?}; model = {:?}",
                violated, model
            );
        }
        assert!(!sat);
    }

    #[test]
    fn simple_unsat_with_learning() {
        let mut s = SatSolver::new();
        let x = s.new_var();
        let y = s.new_var();
        // Block every assignment of (x, y) -> UNSAT.
        s.add_clause(&[Lit::new(x, false), Lit::new(y, false)]);
        s.add_clause(&[Lit::new(x, true), Lit::new(y, false)]);
        s.add_clause(&[Lit::new(x, false), Lit::new(y, true)]);
        s.add_clause(&[Lit::new(x, true), Lit::new(y, true)]);
        assert!(!s.solve());
    }

    #[test]
    fn sat_finds_model() {
        let mut s = SatSolver::new();
        let x = s.new_var();
        let y = s.new_var();
        let z = s.new_var();
        s.add_clause(&[Lit::new(x, false), Lit::new(y, false)]);
        s.add_clause(&[Lit::new(y, true), Lit::new(z, false)]);
        s.add_clause(&[Lit::new(z, false)]);
        assert!(s.solve());
        // z must be true
        assert_eq!(s.value(Lit::new(z, false)), Some(true));
        let model = s.model();
        assert_eq!(model.len(), 3);
    }
}

#[cfg(test)]
mod distro_tests {
    use tpt_l_sat_solver::PackageResolver;

    #[test]
    fn resolves_simple_dependency() {
        let mut r = PackageResolver::new();
        let foo = r.add_package("foo");
        let foo_1 = r.add_version(foo, "1.0");
        let bar = r.add_package("bar");
        let bar_1 = r.add_version(bar, "1.0");
        r.enforce_unique_versions();
        r.require_dependency(foo_1, bar, &[bar_1]);
        r.request(foo_1);
        let sol = r.solve().unwrap();
        assert!(sol.contains(&("foo".to_string(), "1.0".to_string())));
        assert!(sol.contains(&("bar".to_string(), "1.0".to_string())));
    }

    #[test]
    fn detects_conflict() {
        let mut r = PackageResolver::new();
        let foo = r.add_package("foo");
        let foo_1 = r.add_version(foo, "1.0");
        let foo_2 = r.add_version(foo, "2.0");
        let bar = r.add_package("bar");
        let bar_1 = r.add_version(bar, "1.0");
        let baz = r.add_package("baz");
        let baz_1 = r.add_version(baz, "1.0");
        r.enforce_unique_versions();
        r.require_dependency(foo_1, baz, &[baz_1]);
        r.require_dependency(bar_1, baz, &[baz_1]);
        // Request two mutually exclusive foo versions.
        r.request(foo_1);
        r.request(foo_2);
        assert!(r.solve().is_err());
    }
}
