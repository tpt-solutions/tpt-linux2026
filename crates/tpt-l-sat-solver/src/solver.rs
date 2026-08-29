//! Conflict-Driven Clause Learning (CDCL) core.
//!
//! Implements watched-literal unit propagation, 1-UIP conflict analysis with
//! clause learning, non-chronological backtracking, and a VSIDS decision
//! heuristic. The public, allocation-friendly API lives in [`crate::SatSolver`].

use super::literal::{Lit, Tri, Var};

struct Watcher {
    cref: usize,
    blocker: Lit,
}

struct Clause {
    lits: Vec<Lit>,
}

/// The internal CDCL state machine.
pub(crate) struct Solver {
    pub n_vars: u32,
    clauses: Vec<Clause>,

    // Assignment state
    assigns: Vec<Tri>,
    level: Vec<Option<usize>>,
    reason: Vec<Option<usize>>,

    // VSIDS
    activity: Vec<f64>,
    var_inc: f64,

    // Propagation queue
    trail: Vec<Lit>,
    trail_lim: Vec<usize>,
    qhead: usize,

    // Watched literals: indexed by `Lit::index()` (pos + neg).
    watches: Vec<Vec<Watcher>>,

    pub ok: bool,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            n_vars: 0,
            clauses: Vec::new(),
            assigns: Vec::new(),
            level: Vec::new(),
            reason: Vec::new(),
            activity: Vec::new(),
            var_inc: 1.0,
            trail: Vec::new(),
            trail_lim: Vec::new(),
            qhead: 0,
            watches: Vec::new(),
            ok: true,
        }
    }

    pub fn new_var(&mut self) -> Var {
        let v = Var(self.n_vars);
        self.n_vars += 1;
        self.assigns.push(Tri::Undef);
        self.level.push(None);
        self.reason.push(None);
        self.activity.push(0.0);
        self.watches.push(Vec::new()); // positive literal
        self.watches.push(Vec::new()); // negative literal
        v
    }

    #[inline]
    fn value(&self, lit: Lit) -> Tri {
        match self.assigns[lit.var().0 as usize] {
            Tri::Undef => Tri::Undef,
            Tri::True => {
                if lit.is_pos() {
                    Tri::True
                } else {
                    Tri::False
                }
            }
            Tri::False => {
                if lit.is_pos() {
                    Tri::False
                } else {
                    Tri::True
                }
            }
        }
    }

    #[inline]
    fn decision_level(&self) -> usize {
        self.trail_lim.len()
    }

    fn enqueue(&mut self, lit: Lit, reason: Option<usize>) -> Option<()> {
        match self.value(lit) {
            Tri::True => Some(()),
            Tri::False => None,
            Tri::Undef => {
                let v = lit.var().0 as usize;
                self.assigns[v] = Tri::from_bool(lit.is_pos());
                self.level[v] = Some(self.trail_lim.len());
                self.reason[v] = reason;
                self.trail.push(lit);
                Some(())
            }
        }
    }

    /// Add a clause. Returns `false` if the formula is already unsatisfiable.
    pub fn add_clause(&mut self, lits: &[Lit]) -> bool {
        if !self.ok {
            return false;
        }
        // Sanitize: drop duplicates, detect tautology, eval top-level.
        let mut clean: Vec<Lit> = Vec::with_capacity(lits.len());
        let mut seen_pos = 0u64;
        let mut seen_neg = 0u64;
        for &l in lits {
            match self.value(l) {
                Tri::True => return true, // already satisfied
                Tri::False => continue,   // drop
                Tri::Undef => {}
            }
            let v = l.var().0 as usize;
            if l.is_pos() {
                if seen_neg & (1u64 << (v & 63)) != 0 {
                    return true; // tautology
                }
                seen_pos |= 1u64 << (v & 63);
            } else if seen_pos & (1u64 << (v & 63)) != 0 {
                return true;
            } else {
                seen_neg |= 1u64 << (v & 63);
            }
            if !clean.contains(&l) {
                clean.push(l);
            }
        }
        match clean.len() {
            0 => {
                self.ok = false;
                false
            }
            1 => self.enqueue(clean[0], None).is_some(),
            _ => {
                let id = self.clauses.len();
                self.clauses.push(Clause { lits: clean.clone() });
                let w0 = clean[0];
                let w1 = clean[1];
                self.watches[w0.index()].push(Watcher { cref: id, blocker: w1 });
                self.watches[w1.index()].push(Watcher { cref: id, blocker: w0 });
                true
            }
        }
    }

    /// Unit propagation. Returns the conflicting clause id, if any.
    fn propagate(&mut self) -> Option<usize> {
        let mut conflict = None;
        while self.qhead < self.trail.len() {
            let p = self.trail[self.qhead];
            self.qhead += 1;
            // `p` is the literal just assigned TRUE; the watched literal that
            // has now become FALSE is its negation.
            let fl = p.negate();
            let ws = core::mem::take(&mut self.watches[fl.index()]);
            let mut keep: Vec<Watcher> = Vec::with_capacity(ws.len());
            for w in ws {
                let ci = w.cref;
                if self.value(w.blocker) == Tri::True {
                    keep.push(w);
                    continue;
                }
                let (false_pos, other) = {
                    let cl = &self.clauses[ci];
                    if cl.lits[0] == fl { (0, cl.lits[1]) } else { (1, cl.lits[0]) }
                };
                let mut found: Option<(usize, Lit)> = None;
                let len = self.clauses[ci].lits.len();
                for idx in 2..len {
                    let q = self.clauses[ci].lits[idx];
                    if self.value(q) != Tri::False {
                        found = Some((idx, q));
                        break;
                    }
                }
                if let Some((idx, q)) = found {
                    self.clauses[ci].lits.swap(false_pos, idx);
                    self.watches[q.index()].push(Watcher { cref: ci, blocker: other });
                } else {
                    keep.push(w);
                    if self.value(other) == Tri::False || self.enqueue(other, Some(ci)).is_none() {
                        conflict = Some(ci);
                        break;
                    }
                }
            }
            self.watches[fl.index()] = keep;
            if conflict.is_some() {
                break;
            }
        }
        conflict
    }

    fn var_bump(&mut self, v: usize) {
        self.activity[v] += self.var_inc;
        if self.activity[v] > 1e100 {
            // Rescale to avoid overflow.
            for a in &mut self.activity {
                *a *= 1e-100;
            }
            self.var_inc *= 1e-100;
        }
    }

    fn var_decay(&mut self) {
        self.var_inc /= 0.95;
    }

    /// 1-UIP conflict analysis.
    fn analyze(&mut self, confl: usize) -> (Vec<Lit>, usize) {
        let mut learnt: Vec<Lit> = Vec::new();
        let mut seen = vec![false; self.n_vars as usize];
        let mut i = self.trail.len();
        let mut c = confl;
        let mut p: Option<Lit> = None;
        let mut cnt: u32 = 0;

        loop {
            let lits = self.clauses[c].lits.clone();
            for &q in &lits {
                if Some(q) == p {
                    continue;
                }
                let v = q.var().0 as usize;
                if !seen[v] && self.level[v].is_some() && self.level[v].unwrap() > 0 {
                    self.var_bump(v);
                    if self.level[v].unwrap() >= self.decision_level() {
                        cnt += 1;
                        seen[v] = true;
                    } else {
                        learnt.push(q);
                    }
                }
            }
            // Select next literal to follow.
            loop {
                i -= 1;
                let x = self.trail[i];
                if seen[x.var().0 as usize] {
                    c = self.reason[x.var().0 as usize].unwrap_or(confl);
                    p = Some(x);
                    seen[x.var().0 as usize] = false;
                    cnt -= 1;
                    break;
                }
            }
            if cnt == 0 {
                break;
            }
        }

        if let Some(p) = p {
            learnt.insert(0, p.negate());
        }

        // Backtrack level = second-highest decision level in learnt clause.
        let mut levels: Vec<usize> =
            learnt.iter().map(|l| self.level[l.var().0 as usize].unwrap_or(0)).collect();
        levels.sort_unstable();
        levels.reverse();
        let back = if levels.len() >= 2 { levels[1] } else { 0 };

        (learnt, back)
    }

    fn cancel_until(&mut self, level: usize) {
        if self.decision_level() <= level {
            return;
        }
        for i in (self.trail_lim[level]..self.trail.len()).rev() {
            let x = self.trail[i];
            let v = x.var().0 as usize;
            self.assigns[v] = Tri::Undef;
            self.level[v] = None;
            self.reason[v] = None;
        }
        self.trail.truncate(self.trail_lim[level]);
        self.trail_lim.truncate(level);
        self.qhead = self.trail.len();
    }

    fn pick_branch(&mut self) -> Option<Lit> {
        let mut best: Option<Var> = None;
        let mut best_act = f64::NEG_INFINITY;
        for v in 0..self.n_vars as usize {
            if self.assigns[v].is_undef() && self.activity[v] > best_act {
                best_act = self.activity[v];
                best = Some(Var(v as u32));
            }
        }
        best.map(|v| Lit::new(v, false))
    }

    fn all_assigned(&self) -> bool {
        self.assigns.iter().all(|a| !a.is_undef())
    }

    fn add_learnt(&mut self, learnt: Vec<Lit>) {
        self.var_decay();
        if learnt.len() == 1 {
            self.enqueue(learnt[0], None);
            return;
        }
        let id = self.clauses.len();
        self.clauses.push(Clause { lits: learnt.clone() });
        let w0 = learnt[0];
        let w1 = learnt[1];
        self.watches[w0.index()].push(Watcher { cref: id, blocker: w1 });
        self.watches[w1.index()].push(Watcher { cref: id, blocker: w0 });
        self.enqueue(w0, Some(id));
    }

    /// Run CDCL search from the current state.
    fn search(&mut self) -> bool {
        loop {
            match self.propagate() {
                None => {
                    if self.all_assigned() {
                        return true;
                    }
                    if let Some(lit) = self.pick_branch() {
                        self.trail_lim.push(self.trail.len());
                        self.enqueue(lit, None);
                    } else {
                        return true;
                    }
                }
                Some(confl) => {
                    if self.decision_level() == 0 {
                        self.ok = false;
                        return false;
                    }
                    let (learnt, blvl) = self.analyze(confl);
                    self.cancel_until(blvl);
                    self.add_learnt(learnt);
                }
            }
        }
    }

    pub fn solve(&mut self) -> bool {
        self.ok && self.search()
    }

    pub fn value_of(&self, lit: Lit) -> Tri {
        self.value(lit)
    }

    /// Debug helper: verify all clauses are satisfied by the current assignment.
    pub fn verify(&self) -> bool {
        self.first_violated().is_none()
    }

    /// Debug helper: index of the first clause unsatisfied by the current
    /// assignment, if any.
    pub fn first_violated(&self) -> Option<usize> {
        for (i, cl) in self.clauses.iter().enumerate() {
            let ok = cl.lits.iter().any(|&l| self.value(l) == Tri::True);
            if !ok {
                return Some(i);
            }
        }
        None
    }

    /// Debug helper: literals (DIMACS) of the first violated clause, if any.
    pub fn debug_violated_lits(&self) -> Option<Vec<i32>> {
        let i = self.first_violated()?;
        Some(self.clauses[i].lits.iter().map(|&l| l.to_dimacs()).collect())
    }

    pub fn model(&self) -> Vec<(Var, bool)> {
        self.assigns
            .iter()
            .enumerate()
            .filter_map(|(i, a)| match a {
                Tri::True => Some((Var(i as u32), true)),
                Tri::False => Some((Var(i as u32), false)),
                Tri::Undef => None,
            })
            .collect()
    }
}
