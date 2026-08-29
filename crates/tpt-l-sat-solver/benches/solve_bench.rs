use criterion::{Criterion, criterion_group, criterion_main};
use tpt_l_sat_solver::{Lit, SatSolver, Var};

/// Solve a deterministic random 3-SAT instance and measure wall-clock time.
fn bench_random_3sat(c: &mut Criterion) {
    let n_vars = 120usize;
    let n_clauses = 500usize;

    // Deterministic instance via a fixed LCG seed.
    let mut seed: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut rng = || {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (seed >> 33) as usize
    };

    let mut instance: Vec<Vec<Lit>> = Vec::with_capacity(n_clauses);
    for _ in 0..n_clauses {
        let clause = (0..3)
            .map(|_| {
                let v = (rng() % n_vars) as u32;
                let sign = (rng() & 1) == 1;
                Lit::new(Var(v), sign)
            })
            .collect();
        instance.push(clause);
    }

    c.bench_function("solve_120var_500clause_3sat", |b| {
        b.iter(|| {
            let mut s = SatSolver::new();
            for _ in 0..n_vars {
                s.new_var();
            }
            for clause in &instance {
                s.add_clause(clause);
            }
            let _ = s.solve();
        });
    });
}

criterion_group!(benches, bench_random_3sat);
criterion_main!(benches);
