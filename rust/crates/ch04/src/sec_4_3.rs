// SPDX-License-Identifier: GPL-3.0-only
//
// Section 4.3: the named search experiment (grammar §7).
// `search-depth-first/1` explores explicit `Choose`/`Fail`/`Success`
// data depth-first in the vector's written order, records mutations on
// an explicit trail, and rolls back only trailed assignments when
// backtracking. Generator alternatives (`ChooseRange`, `ChooseFrom`)
// supply the between/starting-from lessons; `Persist` is the
// permanent-set! lesson; `Guard(Predicate, ...)` gives binding-dependent
// constraints; `Success(Vec<AnswerTerm>)` answers compute from the
// bindings. It is deterministic for a fixed program. `?`, `Result`,
// panics, and ordinary `return` never trigger backtracking.

use std::collections::HashMap;

pub use sicp_runtime::host::query::{Predicate, Term};

/// One resolved answer operand: a literal, a bound variable, or a
/// computed combination of bound variables (grammar §7 success data).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnswerTerm {
    /// A literal answer component.
    Const(i64),
    /// The current binding of one variable.
    Var(String),
    /// The sum of the listed variables' bindings, in order.
    Sum(Vec<String>),
    /// The product of the listed variables' bindings, in order.
    Product(Vec<String>),
    /// A symbolic answer component.
    Atom(String),
}

/// One resolved answer component: an integer or a symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnswerValue {
    /// An integer component.
    Int(i64),
    /// A symbolic component.
    Sym(String),
}

/// The search data language of grammar §7.
#[derive(Debug, Clone)]
pub enum Search {
    /// A produced answer, computed from the current bindings.
    Success(Vec<AnswerTerm>),
    /// A dead end.
    Fail,
    /// An explicit choice point, alternatives in written order.
    Choose(Vec<Search>),
    /// Uses the primary search's answers, or the fallback if it yields none.
    IfFail {
        /// The preferred search.
        primary: Box<Search>,
        /// The search used when the primary yields no answers.
        fallback: Box<Search>,
    },
    /// `an-integer-between`: `var` ranges over `lo..=hi`.
    ChooseRange {
        /// The variable to bind.
        var: String,
        /// The inclusive low bound.
        lo: i64,
        /// The inclusive high bound.
        hi: i64,
        /// The continuation.
        body: Box<Search>,
    },
    /// `an-integer-starting-from`: the unbounded generator.
    ChooseFrom {
        /// The variable to bind.
        var: String,
        /// The first value.
        start: i64,
        /// The continuation.
        body: Box<Search>,
    },
    /// A trailed assignment: rolled back when backtracking crosses it.
    Set(String, i64, Box<Search>),
    /// `permanent-set!`: an assignment the trail does not restore.
    Persist(String, i64, Box<Search>),
    /// A binding-dependent constraint.
    Guard(Predicate, Box<Search>),
    /// An ordered effect before the continuation.
    Emit(String, Box<Search>),
}

/// One search run's observable outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchOutcome {
    /// Every answer tuple, in the order the search produced them.
    pub answers: Vec<Vec<AnswerValue>>,
    /// The ordered effect log.
    pub effects: Vec<String>,
}

/// The search engine: one explicit trail, one ordered effect log, and
/// an answer limit that makes unbounded generators observable as
/// prefixes.
pub struct SearchEngine {
    trail: Vec<(String, Option<i64>)>,
    bindings: HashMap<String, i64>,
    answers: Vec<Vec<AnswerValue>>,
    effects: Vec<String>,
    limit: usize,
    /// A randomized rambling order, when seeded; `None` keeps the
    /// depth-first written order of `search-depth-first/1`.
    seed: Option<u64>,
    stream: u64,
}
impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// One splitmix64 step: the deterministic source behind seeded
/// rambling.
fn splitmix64(stream: &mut u64) -> u64 {
    *stream = stream.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *stream;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A Fisher-Yates permutation driven by the engine's stream.
fn shuffle<T>(stream: &mut u64, items: &mut [T]) {
    let mut index = items.len();
    while index > 1 {
        index -= 1;
        let modulus = u64::try_from(index + 1).expect("slice length fits in u64");
        let pick = usize::try_from(splitmix64(stream) % modulus)
            .expect("pick is bounded by the slice index");
        items.swap(index, pick);
    }
}

impl SearchEngine {
    /// Builds one engine.
    #[must_use]
    pub fn new() -> Self {
        Self {
            trail: Vec::new(),
            bindings: HashMap::new(),
            answers: Vec::new(),
            effects: Vec::new(),
            limit: usize::MAX,
            seed: None,
            stream: 0,
        }
    }

    /// Builds one rambling engine: finite choice points visit their
    /// alternatives in a deterministic seeded order (splitmix64 per
    /// choice point). Unseeded depth-first behavior is unchanged.
    #[must_use]
    pub fn with_seed(seed: u64) -> Self {
        Self {
            trail: Vec::new(),
            bindings: HashMap::new(),
            answers: Vec::new(),
            effects: Vec::new(),
            limit: usize::MAX,
            seed: Some(seed),
            stream: seed,
        }
    }

    /// Runs one finite search program under the engine's rambling
    /// order: the same seed and program answer identically.
    #[must_use]
    pub fn run_seeded(&mut self, program: &Search) -> SearchOutcome {
        self.limit = usize::MAX;
        self.explore(program);
        self.finish()
    }

    /// Runs one finite search program, answering every success in
    /// depth-first written order.
    #[must_use]
    pub fn run(&mut self, program: &Search) -> SearchOutcome {
        self.limit = usize::MAX;
        self.explore(program);
        self.finish()
    }

    /// Runs one search program for at most `n` answers: the observable
    /// prefix discipline for unbounded generators and fair searches.
    #[must_use]
    pub fn run_prefix(&mut self, program: &Search, n: usize) -> SearchOutcome {
        self.limit = n;
        self.explore(program);
        self.finish()
    }

    /// One binding's current value, for tests that observe the trail.
    #[must_use]
    pub fn binding(&self, name: &str) -> Option<i64> {
        self.bindings.get(name).copied()
    }

    fn finish(&mut self) -> SearchOutcome {
        SearchOutcome {
            answers: std::mem::take(&mut self.answers),
            effects: std::mem::take(&mut self.effects),
        }
    }

    fn explore(&mut self, program: &Search) {
        if self.answers.len() >= self.limit {
            return;
        }
        match program {
            Search::Success(parts) => {
                if let Some(answer) = self.resolve_answer(parts) {
                    self.answers.push(answer);
                }
            }
            Search::Fail => {}
            Search::Choose(alternatives) => {
                if self.seed.is_some() {
                    let mut order: Vec<usize> = (0..alternatives.len()).collect();
                    shuffle(&mut self.stream, &mut order);
                    for index in order {
                        if self.answers.len() >= self.limit {
                            return;
                        }
                        let mark = self.trail.len();
                        self.explore(&alternatives[index]);
                        self.rollback(mark);
                    }
                    return;
                }
                for alternative in alternatives {
                    if self.answers.len() >= self.limit {
                        return;
                    }
                    let mark = self.trail.len();
                    self.explore(alternative);
                    self.rollback(mark);
                }
            }
            Search::IfFail { primary, fallback } => {
                let answer_mark = self.answers.len();
                let trail_mark = self.trail.len();
                self.explore(primary);
                if self.answers.len() == answer_mark {
                    self.rollback(trail_mark);
                    self.explore(fallback);
                }
            }
            Search::ChooseRange { var, lo, hi, body } => {
                if self.seed.is_some() {
                    // Finite ranges enumerate in seeded order; ranges
                    // too wide to materialize keep written order.
                    let width = hi.saturating_sub(*lo);
                    if (0..1_000_000).contains(&width) {
                        let mut values: Vec<i64> = (*lo..=*hi).collect();
                        shuffle(&mut self.stream, &mut values);
                        for value in values {
                            if self.answers.len() >= self.limit {
                                return;
                            }
                            self.bind(var, value);
                            self.explore(body);
                            self.rollback_binding(var);
                        }
                        return;
                    }
                }
                for value in *lo..=*hi {
                    if self.answers.len() >= self.limit {
                        return;
                    }
                    self.bind(var, value);
                    self.explore(body);
                    self.rollback_binding(var);
                }
            }
            Search::ChooseFrom { var, start, body } => {
                let mut value = *start;
                loop {
                    if self.answers.len() >= self.limit {
                        return;
                    }
                    self.bind(var, value);
                    self.explore(body);
                    self.rollback_binding(var);
                    value = value.saturating_add(1);
                }
            }
            Search::Set(name, value, body) => {
                self.bind(name, *value);
                self.explore(body);
                // The caller's rollback discipline restores the trail.
            }
            Search::Persist(name, value, body) => {
                self.bindings.insert(name.clone(), *value);
                self.explore(body);
            }
            Search::Guard(predicate, body) => {
                if self.holds(predicate) {
                    self.explore(body);
                }
            }
            Search::Emit(tag, body) => {
                self.effects.push(tag.clone());
                self.explore(body);
            }
        }
    }

    fn bind(&mut self, name: &str, value: i64) {
        let previous = self.bindings.insert(name.to_owned(), value);
        self.trail.push((name.to_owned(), previous));
    }

    fn rollback_binding(&mut self, name: &str) {
        while let Some((bound, previous)) = self.trail.pop() {
            let restoring = bound == name;
            match previous {
                Some(value) => {
                    self.bindings.insert(bound, value);
                }
                None => {
                    self.bindings.remove(&bound);
                }
            }
            if restoring {
                break;
            }
        }
    }

    fn rollback(&mut self, mark: usize) {
        while self.trail.len() > mark {
            let Some((name, previous)) = self.trail.pop() else {
                break;
            };
            match previous {
                Some(value) => {
                    self.bindings.insert(name, value);
                }
                None => {
                    self.bindings.remove(&name);
                }
            }
        }
    }

    fn resolve_answer(&self, parts: &[AnswerTerm]) -> Option<Vec<AnswerValue>> {
        let mut answer = Vec::with_capacity(parts.len());
        for part in parts {
            answer.push(match part {
                AnswerTerm::Const(value) => AnswerValue::Int(*value),
                AnswerTerm::Var(name) => AnswerValue::Int(*self.bindings.get(name)?),
                AnswerTerm::Sum(names) => {
                    let mut total = 0_i64;
                    for name in names {
                        total = total.checked_add(*self.bindings.get(name)?)?;
                    }
                    AnswerValue::Int(total)
                }
                AnswerTerm::Product(names) => {
                    let mut total = 1_i64;
                    for name in names {
                        total = total.checked_mul(*self.bindings.get(name)?)?;
                    }
                    AnswerValue::Int(total)
                }
                AnswerTerm::Atom(name) => AnswerValue::Sym(name.clone()),
            });
        }
        Some(answer)
    }

    fn holds(&self, predicate: &Predicate) -> bool {
        self.try_holds(predicate).unwrap_or(false)
    }

    fn try_holds(&self, predicate: &Predicate) -> Option<bool> {
        let value = |term: &Term| -> Option<i64> {
            match term {
                Term::Integer(v) => Some(*v),
                Term::Variable(name) => self.bindings.get(name).copied(),
                _ => None,
            }
        };
        Some(match predicate {
            Predicate::Eq(a, b) => value(a)? == value(b)?,
            Predicate::Ne(a, b) => value(a)? != value(b)?,
            Predicate::Lt(a, b) => value(a)? < value(b)?,
            Predicate::Le(a, b) => value(a)? <= value(b)?,
            Predicate::Gt(a, b) => value(a)? > value(b)?,
            Predicate::Ge(a, b) => value(a)? >= value(b)?,
            Predicate::TextLt(a, b) => render_term(a, self) < render_term(b, self),
            Predicate::SumEq(terms, bound) => {
                let mut total = 0_i64;
                for t in terms {
                    total = total.checked_add(value(t)?)?;
                }
                total == *bound
            }
            Predicate::Bound(t) => value(t).is_some(),
            Predicate::Or(alternatives) => alternatives.iter().any(|sub| self.holds(sub)),
            Predicate::DiffEq(a, b, c, d) => {
                let left = value(&Term::Variable(a.clone()))?
                    .checked_sub(value(&Term::Variable(b.clone()))?)?;
                let right = value(&Term::Variable(c.clone()))?
                    .checked_sub(value(&Term::Variable(d.clone()))?)?;
                left == right
            }
            Predicate::Pythagorean(a, b, c) => {
                let a = value(&Term::Variable(a.clone()))?;
                let b = value(&Term::Variable(b.clone()))?;
                let c = value(&Term::Variable(c.clone()))?;
                a.checked_mul(a)?.checked_add(b.checked_mul(b)?)? == c.checked_mul(c)?
            }
        })
    }
}

fn render_term(term: &Term, engine: &SearchEngine) -> String {
    match term {
        Term::Integer(value) => value.to_string(),
        Term::Text(text) | Term::Atom(text) => text.clone(),
        Term::Variable(name) => engine
            .bindings
            .get(name)
            .map_or_else(|| name.clone(), std::string::ToString::to_string),
        other => format!("{other:?}"),
    }
}

/// The independent finite reference model of grammar §7: a separate
/// enumerator with its own trail, bindings, and answer bookkeeping —
/// not a call into [`SearchEngine`] — so answer order, rollback, and
/// permanent-state behavior can be checked against it. Unbounded
/// generators are observed through the engine's prefix runs.
#[must_use]
pub fn reference_model(program: &Search) -> SearchOutcome {
    let mut answers = Vec::new();
    let mut effects = Vec::new();
    let mut trail: Vec<(String, Option<i64>)> = Vec::new();
    let mut bindings: HashMap<String, i64> = HashMap::new();
    reference_explore(
        program,
        &mut answers,
        &mut effects,
        &mut trail,
        &mut bindings,
    );
    SearchOutcome { answers, effects }
}

#[allow(clippy::too_many_lines)]
fn reference_explore(
    program: &Search,
    answers: &mut Vec<Vec<AnswerValue>>,
    effects: &mut Vec<String>,
    trail: &mut Vec<(String, Option<i64>)>,
    bindings: &mut HashMap<String, i64>,
) {
    match program {
        Search::Success(parts) => {
            if let Some(answer) = reference_answer(parts, bindings) {
                answers.push(answer);
            }
        }
        Search::Fail => {}
        Search::Choose(alternatives) => {
            for alternative in alternatives {
                let mark = trail.len();
                reference_explore(alternative, answers, effects, trail, bindings);
                reference_rollback(trail, bindings, mark);
            }
        }
        Search::IfFail { primary, fallback } => {
            let answer_mark = answers.len();
            let trail_mark = trail.len();
            reference_explore(primary, answers, effects, trail, bindings);
            if answers.len() == answer_mark {
                reference_rollback(trail, bindings, trail_mark);
                reference_explore(fallback, answers, effects, trail, bindings);
            }
        }
        Search::ChooseRange { var, lo, hi, body } => {
            for value in *lo..=*hi {
                reference_bind(var, value, trail, bindings);
                reference_explore(body, answers, effects, trail, bindings);
                reference_rollback(trail, bindings, trail.len() - 1);
            }
        }
        Search::ChooseFrom { var, start, body } => {
            // The reference model covers finite prefixes only; the
            // engine's run_prefix is the unbounded observation.
            let mut value = *start;
            let mut produced = 0;
            while produced < 32 {
                reference_bind(var, value, trail, bindings);
                reference_explore(body, answers, effects, trail, bindings);
                reference_rollback(trail, bindings, trail.len() - 1);
                value = value.saturating_add(1);
                produced += 1;
            }
        }
        Search::Set(name, value, body) => {
            reference_bind(name, *value, trail, bindings);
            reference_explore(body, answers, effects, trail, bindings);
        }
        Search::Persist(name, value, body) => {
            bindings.insert(name.clone(), *value);
            reference_explore(body, answers, effects, trail, bindings);
        }
        Search::Guard(predicate, body) => {
            if reference_holds(predicate, bindings) {
                reference_explore(body, answers, effects, trail, bindings);
            }
        }
        Search::Emit(tag, body) => {
            effects.push(tag.clone());
            reference_explore(body, answers, effects, trail, bindings);
        }
    }
}

fn reference_bind(
    name: &str,
    value: i64,
    trail: &mut Vec<(String, Option<i64>)>,
    bindings: &mut HashMap<String, i64>,
) {
    let previous = bindings.insert(name.to_owned(), value);
    trail.push((name.to_owned(), previous));
}

fn reference_rollback(
    trail: &mut Vec<(String, Option<i64>)>,
    bindings: &mut HashMap<String, i64>,
    mark: usize,
) {
    while trail.len() > mark {
        let Some((name, previous)) = trail.pop() else {
            break;
        };
        match previous {
            Some(value) => {
                bindings.insert(name, value);
            }
            None => {
                bindings.remove(&name);
            }
        }
    }
}

fn reference_answer(
    parts: &[AnswerTerm],
    bindings: &HashMap<String, i64>,
) -> Option<Vec<AnswerValue>> {
    let mut answer = Vec::with_capacity(parts.len());
    for part in parts {
        answer.push(match part {
            AnswerTerm::Const(value) => AnswerValue::Int(*value),
            AnswerTerm::Var(name) => AnswerValue::Int(*bindings.get(name)?),
            AnswerTerm::Sum(names) => {
                let mut total = 0_i64;
                for name in names {
                    total = total.checked_add(*bindings.get(name)?)?;
                }
                AnswerValue::Int(total)
            }
            AnswerTerm::Product(names) => {
                let mut total = 1_i64;
                for name in names {
                    total = total.checked_mul(*bindings.get(name)?)?;
                }
                AnswerValue::Int(total)
            }
            AnswerTerm::Atom(name) => AnswerValue::Sym(name.clone()),
        });
    }
    Some(answer)
}

fn reference_holds(predicate: &Predicate, bindings: &HashMap<String, i64>) -> bool {
    let value = |name: &str| -> Option<i64> { bindings.get(name).copied() };
    match predicate {
        Predicate::Eq(a, b) => {
            reference_term(a, bindings) == reference_term(b, bindings)
                && reference_term(a, bindings).is_some()
        }
        Predicate::Ne(a, b) => {
            let (a, b) = (reference_term(a, bindings), reference_term(b, bindings));
            a.is_some() && b.is_some() && a != b
        }
        Predicate::Lt(a, b) => reference_cmp(a, b, bindings, |x, y| x < y),
        Predicate::Le(a, b) => reference_cmp(a, b, bindings, |x, y| x <= y),
        Predicate::Gt(a, b) => reference_cmp(a, b, bindings, |x, y| x > y),
        Predicate::Ge(a, b) => reference_cmp(a, b, bindings, |x, y| x >= y),
        Predicate::TextLt(a, b) => reference_text(a, bindings) < reference_text(b, bindings),
        Predicate::SumEq(terms, bound) => {
            let mut total = 0_i64;
            for term in terms {
                match reference_term(term, bindings).and_then(|value| total.checked_add(value)) {
                    Some(next) => total = next,
                    None => return false,
                }
            }
            total == *bound
        }
        Predicate::Bound(term) => reference_term(term, bindings).is_some(),
        Predicate::Or(alternatives) => alternatives
            .iter()
            .any(|sub| reference_holds(sub, bindings)),
        Predicate::DiffEq(a, b, c, d) => {
            let (a, b, c, d) = (value(a), value(b), value(c), value(d));
            match (a, b, c, d) {
                (Some(a), Some(b), Some(c), Some(d)) => a
                    .checked_sub(b)
                    .zip(c.checked_sub(d))
                    .is_some_and(|(l, r)| l == r),
                _ => false,
            }
        }
        Predicate::Pythagorean(a, b, c) => match (value(a), value(b), value(c)) {
            (Some(a), Some(b), Some(c)) => {
                match (a.checked_mul(a), b.checked_mul(b), c.checked_mul(c)) {
                    (Some(aa), Some(bb), Some(cc)) => aa.checked_add(bb) == Some(cc),
                    _ => false,
                }
            }
            _ => false,
        },
    }
}

fn reference_cmp(
    a: &Term,
    b: &Term,
    bindings: &HashMap<String, i64>,
    order: fn(i64, i64) -> bool,
) -> bool {
    match (reference_term(a, bindings), reference_term(b, bindings)) {
        (Some(a), Some(b)) => order(a, b),
        _ => false,
    }
}

fn reference_term(term: &Term, bindings: &HashMap<String, i64>) -> Option<i64> {
    match term {
        Term::Integer(value) => Some(*value),
        Term::Variable(name) => bindings.get(name).copied(),
        _ => None,
    }
}

fn reference_text(term: &Term, bindings: &HashMap<String, i64>) -> String {
    match term {
        Term::Text(text) | Term::Atom(text) => text.clone(),
        Term::Integer(value) => value.to_string(),
        Term::Variable(name) => bindings
            .get(name)
            .map_or_else(|| name.clone(), std::string::ToString::to_string),
        other => format!("{other:?}"),
    }
}

/// Case `amb/01-amb-basics`: `an-integer-between` 1 and 3; answers in
/// written order.
#[must_use]
pub fn search_basics() -> Search {
    Search::ChooseRange {
        var: "x".to_owned(),
        lo: 1,
        hi: 3,
        body: Box::new(Search::Success(vec![AnswerTerm::Var("x".to_owned())])),
    }
}

/// Case `amb/02-prime-sum-pair`: choice points with guards; the pairs
/// `i < j` whose sum is one of the taught primes.
#[must_use]
pub fn search_prime_sum() -> Search {
    use sicp_runtime::host::query::{Predicate, Term};
    let pair_sum = |prime: i64| {
        Predicate::SumEq(
            vec![
                Term::Variable("i".to_owned()),
                Term::Variable("j".to_owned()),
            ],
            prime,
        )
    };
    Search::ChooseRange {
        var: "i".to_owned(),
        lo: 1,
        hi: 6,
        body: Box::new(Search::ChooseRange {
            var: "j".to_owned(),
            lo: 1,
            hi: 6,
            body: Box::new(Search::Guard(
                Predicate::Lt(
                    Term::Variable("i".to_owned()),
                    Term::Variable("j".to_owned()),
                ),
                Box::new(Search::Guard(
                    Predicate::Or(vec![pair_sum(7), pair_sum(11)]),
                    Box::new(Search::Success(vec![
                        AnswerTerm::Var("i".to_owned()),
                        AnswerTerm::Var("j".to_owned()),
                    ])),
                )),
            )),
        }),
    }
}

/// Case `amb/03-multiple-dwelling`: five distinct floors under the
/// taught constraints; trailed assignments roll back on dead ends.
#[must_use]
pub fn search_dwelling() -> Search {
    use sicp_runtime::host::query::{Predicate, Term};
    let name = |who: &str| Term::Variable(who.to_owned());
    let floor = |value: i64| Term::Integer(value);
    let distinct = [
        ("baker", "cooper"),
        ("baker", "fletcher"),
        ("baker", "miller"),
        ("baker", "smith"),
        ("cooper", "fletcher"),
        ("cooper", "miller"),
        ("cooper", "smith"),
        ("fletcher", "miller"),
        ("fletcher", "smith"),
        ("miller", "smith"),
    ];
    let mut guards: Vec<Predicate> = distinct
        .iter()
        .map(|(a, b)| Predicate::Ne(name(a), name(b)))
        .collect();
    guards.push(Predicate::Ne(name("baker"), floor(5)));
    guards.push(Predicate::Ne(name("cooper"), floor(1)));
    guards.push(Predicate::Ne(name("fletcher"), floor(1)));
    guards.push(Predicate::Ne(name("fletcher"), floor(5)));
    guards.push(Predicate::Gt(name("miller"), name("cooper")));
    // Adjacent floors differ by one, so each adjacency exclusion
    // admits only the differences two, three, or four either way.
    for (high, low) in [("smith", "fletcher"), ("fletcher", "cooper")] {
        guards.push(Predicate::Or(
            ["c2", "c3", "c4"]
                .iter()
                .flat_map(|plus| {
                    [
                        Predicate::DiffEq(
                            high.to_owned(),
                            low.to_owned(),
                            (*plus).to_owned(),
                            "c0".to_owned(),
                        ),
                        Predicate::DiffEq(
                            high.to_owned(),
                            low.to_owned(),
                            "c0".to_owned(),
                            (*plus).to_owned(),
                        ),
                    ]
                })
                .collect(),
        ));
    }
    let mut program = Search::Success(vec![
        AnswerTerm::Var("baker".to_owned()),
        AnswerTerm::Var("cooper".to_owned()),
        AnswerTerm::Var("fletcher".to_owned()),
        AnswerTerm::Var("miller".to_owned()),
        AnswerTerm::Var("smith".to_owned()),
    ]);
    for guard in guards.into_iter().rev() {
        program = Search::Guard(guard, Box::new(program));
    }
    for who in ["baker", "cooper", "fletcher", "miller", "smith"]
        .into_iter()
        .rev()
    {
        program = Search::ChooseRange {
            var: who.to_owned(),
            lo: 1,
            hi: 5,
            body: Box::new(program),
        };
    }
    for (constant, value) in [("c0", 0), ("c2", 2), ("c3", 3), ("c4", 4)]
        .into_iter()
        .rev()
    {
        program = Search::Set(constant.to_owned(), value, Box::new(program));
    }
    program
}

/// Case `amb/04-pythagorean-triples`: nested choice enumeration under
/// the shared `Pythagorean` guard.
#[must_use]
pub fn search_pythagorean() -> Search {
    let mut program = Search::Success(vec![
        AnswerTerm::Var("a".to_owned()),
        AnswerTerm::Var("b".to_owned()),
        AnswerTerm::Var("c".to_owned()),
    ]);
    program = Search::Guard(
        sicp_runtime::host::query::Predicate::Pythagorean(
            "a".to_owned(),
            "b".to_owned(),
            "c".to_owned(),
        ),
        Box::new(program),
    );
    for who in ["a", "b", "c"] {
        program = Search::ChooseRange {
            var: who.to_owned(),
            lo: 1,
            hi: 12,
            body: Box::new(program),
        };
    }
    program
}
