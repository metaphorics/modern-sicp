// SPDX-License-Identifier: GPL-3.0-only
//
// Section 4.2: the named lazy experiments (grammar §7). The core
// evaluator stays strict; `lazy-recompute/1` and `lazy-memo/1` are
// separate engines over explicit `Thunk`/`Force` data, where a delayed
// operand evaluates only when forced. `lazy-recompute/1` reevaluates
// every force; `lazy-memo/1` stores the first successful result per
// thunk identity and runs its effects once. A failed force stays
// delayed in both. Application binds arguments as delayed thunks and
// demands the operator first (the 4.28 ordering lesson); lazy pairs
// carry explicit pair data whose renderings are themselves evaluated
// (4.33/4.34). No closure call or iterator becomes lazy implicitly.

use std::collections::HashMap;

/// The builtin operator values the experiment data can carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimOp {
    /// Integer addition of two demanded operands.
    Add,
    /// Integer multiplication of two demanded operands.
    Mul,
    /// `first` of a demanded pair.
    First,
    /// `rest` of a demanded pair.
    Rest,
    /// Pair construction.
    PairBuild,
    /// Emptiness test.
    IsEmpty,
    /// Pair test.
    IsPair,
}

/// The experiment value domain: produced integers, delayed thunks,
/// explicit pair data, and callable values.
#[derive(Debug, Clone)]
pub enum LazyVal {
    /// A produced integer.
    Now(i64),
    /// An emitted effect tag: integer value 0, rendered as its tag so
    /// lazy pair printing shows the demanded part (exercise 4.34).
    Emitted(String),
    /// A still-delayed thunk identity.
    Later(usize),
    /// An explicit lazy pair.
    PairVal(Box<LazyVal>, Box<LazyVal>),
    /// The empty list datum.
    EmptyVal,
    /// A builtin operator value.
    Prim(PrimOp),
    /// A closure over the experiment's own data, with its captured
    /// environment.
    Closure(Vec<String>, Box<LazyExpr>, Vec<(String, LazyVal)>),
}

/// The experiment language: explicit delayed operands and demands. A
/// thunk carries its identity so the memo mode can key results.
#[derive(Debug, Clone)]
pub enum LazyExpr {
    /// An integer literal.
    Int(i64),
    /// A named binding reference.
    Var(String),
    /// A delayed operand with its identity.
    Thunk(usize, Box<LazyExpr>),
    /// A demand for a delayed operand's value.
    Force(Box<LazyExpr>),
    /// Integer addition of two demanded operands.
    Add(Box<LazyExpr>, Box<LazyExpr>),
    /// Integer multiplication of two demanded operands.
    Mul(Box<LazyExpr>, Box<LazyExpr>),
    /// `let NAME = VALUE in BODY`; the value may be a thunk.
    Let(String, Box<LazyExpr>, Box<LazyExpr>),
    /// An observable effect: it lands in the effect log each time the
    /// expression around it evaluates. Its value is integer 0 carrying
    /// the tag for rendering.
    Emit(String),
    /// `if` in ordinary order: the condition is demanded first and
    /// selects the second argument when nonzero, the third when zero.
    /// The `unless` lesson swaps the two branches at construction.
    If(Box<LazyExpr>, Box<LazyExpr>, Box<LazyExpr>),
    /// Application: the operator is demanded first and every argument
    /// binds as a delayed thunk (the 4.28 ordering lesson).
    Apply(Box<LazyExpr>, Vec<LazyExpr>),
    /// A closure over the experiment's own expression data.
    Lambda(Vec<String>, Box<LazyExpr>),
    /// Pair construction.
    Pair(Box<LazyExpr>, Box<LazyExpr>),
    /// The empty list datum.
    Empty,
    /// Explicit quoted data.
    Quote(LazyVal),
}

/// The two named experiment modes (grammar §7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `lazy-recompute/1`: every force reevaluates.
    Recompute,
    /// `lazy-memo/1`: the first successful result is stored per thunk
    /// identity and its effects occur once.
    Memo,
}

/// One experiment run's observable outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyOutcome {
    /// The produced integer value, or `None` when the program failed
    /// or produced a non-integer datum.
    pub value: Option<i64>,
    /// The ordered effect log.
    pub effects: Vec<String>,
    /// The rendered forms of produced values (lazy pair printing).
    pub rendered: Vec<String>,
}

/// The lazy experiment engine: one named mode, a per-thunk memo table,
/// and an explicit effect log.
pub struct LazyEngine {
    mode: Mode,
    memo: HashMap<usize, Option<LazyVal>>,
    effects: Vec<String>,
    /// Each delayed operand stores its body AND its creating
    /// environment: forcing an escaping thunk reads its own bindings.
    thunks: HashMap<usize, (LazyExpr, HashMap<String, LazyVal>)>,
    next_id: usize,
}

impl LazyEngine {
    /// Builds one engine in one named mode.
    #[must_use]
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            memo: HashMap::new(),
            effects: Vec::new(),
            thunks: HashMap::new(),
            next_id: 0,
        }
    }

    /// Runs one experiment program in the engine's mode.
    #[must_use]
    pub fn run(&mut self, expr: &LazyExpr) -> LazyOutcome {
        // A reused engine keeps no memoized value or thunk
        // environment from an earlier run: a matching thunk id
        // would otherwise force a stale body.
        self.memo.clear();
        self.thunks.clear();
        self.next_id = max_thunk_id(expr);
        let env = HashMap::new();
        let produced = self.eval(expr, &env);
        let (value, rendered) = match &produced {
            Some(val) => (integer_of(val), vec![render_val(val)]),
            None => (None, Vec::new()),
        };
        LazyOutcome {
            value,
            effects: std::mem::take(&mut self.effects),
            rendered,
        }
    }

    fn eval(&mut self, expr: &LazyExpr, env: &HashMap<String, LazyVal>) -> Option<LazyVal> {
        match expr {
            LazyExpr::Int(value) => Some(LazyVal::Now(*value)),
            LazyExpr::Var(name) => env.get(name).cloned(),
            LazyExpr::Thunk(id, body) => {
                self.next_id = self.next_id.max(id + 1);
                self.thunks.insert(*id, ((**body).clone(), env.clone()));
                Some(LazyVal::Later(*id))
            }
            LazyExpr::Force(target) => {
                let demanded = self.eval(target, env)?;
                match demanded {
                    LazyVal::Later(id) => self.force_id(id),
                    other => Some(other),
                }
            }
            LazyExpr::Add(left, right) => {
                let a = self.demand_int(left, env)?;
                let b = self.demand_int(right, env)?;
                Some(LazyVal::Now(a.checked_add(b)?))
            }
            LazyExpr::Mul(left, right) => {
                let a = self.demand_int(left, env)?;
                let b = self.demand_int(right, env)?;
                Some(LazyVal::Now(a.checked_mul(b)?))
            }
            LazyExpr::Let(name, value, body) => {
                let bound = self.eval(value, env)?;
                let mut extended = env.clone();
                extended.insert(name.clone(), bound);
                self.eval(body, &extended)
            }
            LazyExpr::Emit(tag) => {
                self.effects.push(tag.clone());
                Some(LazyVal::Emitted(tag.clone()))
            }
            LazyExpr::If(condition, then, otherwise) => {
                let decision = self.demand_int(condition, env)?;
                if decision != 0 {
                    self.eval(then, env)
                } else {
                    self.eval(otherwise, env)
                }
            }
            LazyExpr::Lambda(params, body) => Some(LazyVal::Closure(
                params.clone(),
                body.clone(),
                env.iter()
                    .map(|(name, value)| (name.clone(), value.clone()))
                    .collect(),
            )),
            LazyExpr::Apply(operator, args) => {
                // The operator is demanded first (4.28 ordering).
                let callable = self.eval(operator, env)?;
                let (params, body, captured) = match &callable {
                    LazyVal::Closure(params, body, captured) => {
                        (params.clone(), body.clone(), captured.clone())
                    }
                    LazyVal::Prim(_) => {
                        // Primitive application demands its operands.
                        let mut values = Vec::with_capacity(args.len());
                        for arg in args {
                            values.push(match self.eval(arg, env)? {
                                LazyVal::Later(id) => self.force_id(id)?,
                                other => other,
                            });
                        }
                        return Self::apply_prim(&callable, &values);
                    }
                    _ => return None,
                };
                if params.len() != args.len() {
                    return None;
                }
                let mut extended: HashMap<String, LazyVal> = captured.into_iter().collect();
                for (param, arg) in params.iter().zip(args) {
                    // Arguments bind as delayed thunks in the caller env.
                    let id = self.register_fresh(arg, env);
                    extended.insert(param.clone(), LazyVal::Later(id));
                }
                self.eval(&body, &extended)
            }
            LazyExpr::Pair(left, right) => {
                let a = self.eval(left, env)?;
                let b = self.eval(right, env)?;
                Some(LazyVal::PairVal(Box::new(a), Box::new(b)))
            }
            LazyExpr::Empty => Some(LazyVal::EmptyVal),
            LazyExpr::Quote(value) => Some(value.clone()),
        }
    }

    fn register_fresh(&mut self, arg: &LazyExpr, env: &HashMap<String, LazyVal>) -> usize {
        let fresh = self.next_id;
        self.next_id += 1;
        self.thunks.insert(fresh, (arg.clone(), env.clone()));
        fresh
    }

    fn apply_prim(callable: &LazyVal, values: &[LazyVal]) -> Option<LazyVal> {
        let LazyVal::Prim(op) = callable else {
            return None;
        };
        match (*op, values) {
            (PrimOp::Add, [a, b]) => {
                Some(LazyVal::Now(integer_of(a)?.checked_add(integer_of(b)?)?))
            }
            (PrimOp::Mul, [a, b]) => {
                Some(LazyVal::Now(integer_of(a)?.checked_mul(integer_of(b)?)?))
            }
            (PrimOp::First, [LazyVal::PairVal(first, _)]) => Some((**first).clone()),
            (PrimOp::Rest, [LazyVal::PairVal(_, rest)]) => Some((**rest).clone()),
            (PrimOp::PairBuild, [a, b]) => {
                Some(LazyVal::PairVal(Box::new(a.clone()), Box::new(b.clone())))
            }
            (PrimOp::IsEmpty, [value]) => {
                Some(LazyVal::Now(i64::from(matches!(value, LazyVal::EmptyVal))))
            }
            (PrimOp::IsPair, [value]) => Some(LazyVal::Now(i64::from(matches!(
                value,
                LazyVal::PairVal(..)
            )))),
            _ => None,
        }
    }

    fn demand_int(&mut self, expr: &LazyExpr, env: &HashMap<String, LazyVal>) -> Option<i64> {
        integer_of(&self.eval(expr, env)?)
    }

    fn force_id(&mut self, id: usize) -> Option<LazyVal> {
        let (body, thunk_env) = self.thunks.get(&id)?.clone();
        match self.mode {
            Mode::Memo => {
                if let Some(cached) = self.memo.get(&id) {
                    return cached.clone();
                }
                let value = self.eval(&body, &thunk_env);
                if value.is_some() {
                    self.memo.insert(id, value.clone());
                }
                value
            }
            Mode::Recompute => self.eval(&body, &thunk_env),
        }
    }
}

fn integer_of(value: &LazyVal) -> Option<i64> {
    match value {
        LazyVal::Now(value) => Some(*value),
        LazyVal::Emitted(_) => Some(0),
        _ => None,
    }
}

fn render_val(value: &LazyVal) -> String {
    match value {
        LazyVal::Now(value) => value.to_string(),
        LazyVal::Emitted(tag) => tag.clone(),
        LazyVal::Later(id) => format!("<thunk {id}>"),
        LazyVal::EmptyVal => "()".to_owned(),
        LazyVal::Prim(op) => format!("<prim {op:?}>"),
        LazyVal::Closure(params, _, _) => format!("<closure {params:?}>"),
        LazyVal::PairVal(first, rest) => {
            let head = render_val(first);
            match rest.as_ref() {
                LazyVal::EmptyVal => format!("({head})"),
                LazyVal::PairVal(..) => format!("({head} . {})", render_val(rest)),
                other => format!("({head} . {})", render_val(other)),
            }
        }
    }
}

/// The smallest id fresh thunks may take: every explicit thunk id in
/// the program is reserved first, so a fresh id can never collide with
/// an id a later evaluation registers.
fn max_thunk_id(expr: &LazyExpr) -> usize {
    match expr {
        LazyExpr::Thunk(id, body) => (*id + 1).max(max_thunk_id(body)),
        LazyExpr::Force(inner) | LazyExpr::Lambda(_, inner) => max_thunk_id(inner),
        LazyExpr::Let(_, value, body) => max_thunk_id(value).max(max_thunk_id(body)),
        LazyExpr::Add(left, right) | LazyExpr::Mul(left, right) | LazyExpr::Pair(left, right) => {
            max_thunk_id(left).max(max_thunk_id(right))
        }
        LazyExpr::If(a, b, c) => max_thunk_id(a).max(max_thunk_id(b)).max(max_thunk_id(c)),
        LazyExpr::Apply(operator, args) => args.iter().fold(max_thunk_id(operator), |acc, arg| {
            acc.max(max_thunk_id(arg))
        }),
        _ => 0,
    }
}

/// The independent finite reference model of grammar §7: a separate
/// interpreter with its own environment, delayed-value table, memo,
/// and effect log. It never calls [`LazyEngine`]; behavioral
/// invariants are checked against this model rather than against the
/// engine's own state.
#[must_use]
pub fn reference_model(mode: Mode, expr: &LazyExpr) -> LazyOutcome {
    let mut model = RefModel {
        mode,
        memo: HashMap::new(),
        thunks: HashMap::new(),
        next_id: max_thunk_id(expr),
        effects: Vec::new(),
    };
    let env = HashMap::new();
    let produced = model.eval(expr, &env);
    let (value, rendered) = match &produced {
        Some(val) => (integer_of(val), vec![render_val(val)]),
        None => (None, Vec::new()),
    };
    LazyOutcome {
        value,
        effects: std::mem::take(&mut model.effects),
        rendered,
    }
}

/// The reference model's own state: nothing here aliases the engine.
struct RefModel {
    mode: Mode,
    memo: HashMap<usize, Option<LazyVal>>,
    thunks: HashMap<usize, (LazyExpr, HashMap<String, LazyVal>)>,
    next_id: usize,
    effects: Vec<String>,
}

impl RefModel {
    fn eval(&mut self, expr: &LazyExpr, env: &HashMap<String, LazyVal>) -> Option<LazyVal> {
        match expr {
            LazyExpr::Int(value) => Some(LazyVal::Now(*value)),
            LazyExpr::Var(name) => env.get(name).cloned(),
            LazyExpr::Thunk(id, body) => {
                self.next_id = self.next_id.max(id + 1);
                self.thunks.insert(*id, ((**body).clone(), env.clone()));
                Some(LazyVal::Later(*id))
            }
            LazyExpr::Force(target) => {
                let demanded = self.eval(target, env)?;
                match demanded {
                    LazyVal::Later(id) => self.force_id(id),
                    other => Some(other),
                }
            }
            LazyExpr::Add(left, right) => {
                let a = integer_of(&self.eval(left, env)?)?;
                let b = integer_of(&self.eval(right, env)?)?;
                Some(LazyVal::Now(a.checked_add(b)?))
            }
            LazyExpr::Mul(left, right) => {
                let a = integer_of(&self.eval(left, env)?)?;
                let b = integer_of(&self.eval(right, env)?)?;
                Some(LazyVal::Now(a.checked_mul(b)?))
            }
            LazyExpr::Let(name, value, body) => {
                let bound = self.eval(value, env)?;
                let mut extended = env.clone();
                extended.insert(name.clone(), bound);
                self.eval(body, &extended)
            }
            LazyExpr::Emit(tag) => {
                self.effects.push(tag.clone());
                Some(LazyVal::Emitted(tag.clone()))
            }
            LazyExpr::If(condition, then, otherwise) => {
                let decision = integer_of(&self.eval(condition, env)?)?;
                if decision != 0 {
                    self.eval(then, env)
                } else {
                    self.eval(otherwise, env)
                }
            }
            LazyExpr::Lambda(params, body) => Some(LazyVal::Closure(
                params.clone(),
                body.clone(),
                env.iter()
                    .map(|(name, value)| (name.clone(), value.clone()))
                    .collect(),
            )),
            LazyExpr::Apply(operator, args) => {
                let callable = self.eval(operator, env)?;
                let (params, body, captured) = match &callable {
                    LazyVal::Closure(params, body, captured) => {
                        (params.clone(), body.clone(), captured.clone())
                    }
                    LazyVal::Prim(_) => {
                        let mut values = Vec::with_capacity(args.len());
                        for arg in args {
                            values.push(match self.eval(arg, env)? {
                                LazyVal::Later(id) => self.force_id(id)?,
                                other => other,
                            });
                        }
                        return Self::apply_prim(&callable, &values);
                    }
                    _ => return None,
                };
                if params.len() != args.len() {
                    return None;
                }
                let mut extended: HashMap<String, LazyVal> = captured.into_iter().collect();
                for (param, arg) in params.iter().zip(args) {
                    let id = self.next_id;
                    self.next_id += 1;
                    self.thunks.insert(id, (arg.clone(), env.clone()));
                    extended.insert(param.clone(), LazyVal::Later(id));
                }
                self.eval(&body, &extended)
            }
            LazyExpr::Pair(left, right) => {
                let a = self.eval(left, env)?;
                let b = self.eval(right, env)?;
                Some(LazyVal::PairVal(Box::new(a), Box::new(b)))
            }
            LazyExpr::Empty => Some(LazyVal::EmptyVal),
            LazyExpr::Quote(value) => Some(value.clone()),
        }
    }

    fn force_id(&mut self, id: usize) -> Option<LazyVal> {
        let (body, thunk_env) = self.thunks.get(&id)?.clone();
        match self.mode {
            Mode::Memo => {
                if let Some(cached) = self.memo.get(&id) {
                    return cached.clone();
                }
                let value = self.eval(&body, &thunk_env);
                if value.is_some() {
                    self.memo.insert(id, value.clone());
                }
                value
            }
            Mode::Recompute => self.eval(&body, &thunk_env),
        }
    }

    fn apply_prim(callable: &LazyVal, values: &[LazyVal]) -> Option<LazyVal> {
        let LazyVal::Prim(op) = callable else {
            return None;
        };
        match (*op, values) {
            (PrimOp::Add, [a, b]) => {
                Some(LazyVal::Now(integer_of(a)?.checked_add(integer_of(b)?)?))
            }
            (PrimOp::Mul, [a, b]) => {
                Some(LazyVal::Now(integer_of(a)?.checked_mul(integer_of(b)?)?))
            }
            (PrimOp::First, [LazyVal::PairVal(first, _)]) => Some((**first).clone()),
            (PrimOp::Rest, [LazyVal::PairVal(_, rest)]) => Some((**rest).clone()),
            (PrimOp::PairBuild, [a, b]) => {
                Some(LazyVal::PairVal(Box::new(a.clone()), Box::new(b.clone())))
            }
            (PrimOp::IsEmpty, [value]) => {
                Some(LazyVal::Now(i64::from(matches!(value, LazyVal::EmptyVal))))
            }
            (PrimOp::IsPair, [value]) => Some(LazyVal::Now(i64::from(matches!(
                value,
                LazyVal::PairVal(..)
            )))),
            _ => None,
        }
    }
}

/// Case `lazy/01-non-strict-application`: the unused exceptional
/// argument is an explicit thunk whose demand would emit; the taken
/// branch never forces it. Strict evaluation would emit.
#[must_use]
pub fn lazy_non_strict() -> LazyExpr {
    LazyExpr::Let(
        "result".to_owned(),
        Box::new(LazyExpr::Force(Box::new(LazyExpr::Apply(
            Box::new(LazyExpr::Lambda(
                vec!["c".to_owned(), "u".to_owned(), "e".to_owned()],
                Box::new(LazyExpr::If(
                    Box::new(LazyExpr::Var("c".to_owned())),
                    Box::new(LazyExpr::Var("u".to_owned())),
                    Box::new(LazyExpr::Var("e".to_owned())),
                )),
            )),
            vec![
                LazyExpr::Int(1),
                LazyExpr::Int(42),
                LazyExpr::Thunk(1, Box::new(LazyExpr::Emit("evaluated".to_owned()))),
            ],
        )))),
        Box::new(LazyExpr::Var("result".to_owned())),
    )
}

/// Case `lazy/02-delay-force`: one thunk forced twice;
/// `lazy-recompute/1` evaluates (and emits) twice, `lazy-memo/1` once.
#[must_use]
pub fn lazy_delay_force() -> LazyExpr {
    LazyExpr::Let(
        "t".to_owned(),
        Box::new(LazyExpr::Thunk(
            1,
            Box::new(LazyExpr::Add(
                Box::new(LazyExpr::Emit("work".to_owned())),
                Box::new(LazyExpr::Int(21)),
            )),
        )),
        Box::new(LazyExpr::Add(
            Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("t".to_owned())))),
            Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("t".to_owned())))),
        )),
    )
}

/// Case `lazy/03-church-pairs`: explicit pair data through the builtin
/// `first` and `pair-build` operators.
#[must_use]
pub fn lazy_church_pairs() -> LazyExpr {
    LazyExpr::Apply(
        Box::new(LazyExpr::Quote(LazyVal::Prim(PrimOp::First))),
        vec![LazyExpr::Apply(
            Box::new(LazyExpr::Quote(LazyVal::Prim(PrimOp::PairBuild))),
            vec![LazyExpr::Int(1), LazyExpr::Int(2)],
        )],
    )
}

/// Case `lazy/04-lazy-list`: a lazy list whose tail is a thunk; the
/// demanded second element forces the tail once.
#[must_use]
pub fn lazy_list() -> LazyExpr {
    LazyExpr::Let(
        "list".to_owned(),
        Box::new(LazyExpr::Pair(
            Box::new(LazyExpr::Int(1)),
            Box::new(LazyExpr::Thunk(
                1,
                Box::new(LazyExpr::Pair(
                    Box::new(LazyExpr::Int(2)),
                    Box::new(LazyExpr::Empty),
                )),
            )),
        )),
        Box::new(LazyExpr::Apply(
            Box::new(LazyExpr::Quote(LazyVal::Prim(PrimOp::First))),
            vec![LazyExpr::Force(Box::new(LazyExpr::Apply(
                Box::new(LazyExpr::Quote(LazyVal::Prim(PrimOp::Rest))),
                vec![LazyExpr::Var("list".to_owned())],
            )))],
        )),
    )
}
