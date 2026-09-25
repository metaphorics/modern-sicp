// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.3

//! Section 4.3: the `amb` evaluator as an explicit backtracking engine.
//!
//! The evaluator runs the chapter's `Value` syntax with one new piece of
//! control: an `amb` form is the book's nondeterministic choice point,
//! searched depth-first. The engine keeps the search in plain data -- a
//! choice frame per open `amb`, each holding its pending alternatives
//! and the continuation of the computation that followed the choice --
//! so a dead end unwinds by returning [`SchemeError::Backtrack`] to the
//! nearest `amb` site, and a solution returned to the driver leaves its
//! frames resumable: `try_again` takes the deepest pending alternative
//! and continues it without replaying the work the answer already did.
//! Assignments made on a branch land on an undo trail that backtracking
//! rolls back; `permanent-set!` skips the trail, `if-fail` catches the
//! failure of its first expression, and `ramb` searches its
//! alternatives in the order a seeded xorshift draws, so a
//! `ramb`-driven program is reproducible for a fixed seed.
//!
//! The book's continuation-passing shape maps onto the engine as
//! follows: the book's success continuation is a [`Cont`], a plain
//! closure from one value to the rest of the computation; the book's
//! failure continuation is the engine's choice stack plus the
//! `Backtrack` unwind, because the alternatives are values in a frame,
//! not closures the program threads through itself; and the driver
//! loop's `try-again` is [`Amb::try_again`], which resumes the saved
//! stack instead of re-running the problem from scratch.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

use sicp_runtime::{Closure, Env, Handler, Random, SchemeError, Symbol, Value, print_value};

use crate::sec_4_1::{
    OutputSink, assignment_value, assignment_variable, cond_to_if, definition_value,
    definition_variable, extend_environment, first_of, if_parts, is_application, is_assignment,
    is_begin, is_cond, is_definition, is_if, is_lambda, is_let, is_quoted, is_self_evaluating,
    is_tagged_list, is_true, is_variable, let_to_combination, operand_items, primitive_table,
    split_params, text_of_quotation,
};

/// One evaluation's answer.
pub type EvalResult = Result<Value, SchemeError>;

/// The rest of one expression's evaluation: the book's success
/// continuation, a closure from the value just obtained to everything
/// that follows it. Failures need no closure -- they travel as
/// [`SchemeError::Backtrack`] to the nearest choice frame.
pub type Cont = Rc<dyn Fn(&Amb, Value) -> EvalResult>;

/// An extension's dispatch clause, the book's new `analyze` case: when
/// it recognizes the expression it answers the evaluation, and when it
/// does not it answers `None` and the section dispatch takes over. The
/// clause sees every expression at every nesting depth, because the
/// engine checks it inside [`Amb::eval_with`].
pub type SpecialHook = Rc<dyn Fn(&Amb, &Value, &Rc<Env>, &Cont) -> Option<EvalResult>>;

/// Builds one continuation.
fn cont(f: impl Fn(&Amb, Value) -> EvalResult + 'static) -> Cont {
    Rc::new(f)
}

/// One undo entry of the trail: the frame, the name, and the value the
/// binding held before the assignment, restored when backtracking
/// crosses the assignment.
struct Undo {
    env: Rc<Env>,
    name: Symbol,
    old: Value,
}

/// One resumable choice frame: the book's `try-next` loop stored as
/// data. `alts` holds the alternatives not yet tried, each with the
/// environment to evaluate it in; `k` is the continuation that consumes
/// each chosen value; `trail_len` is the trail length at the push, the
/// line backtracking rolls the trail back to.
struct AmbFrame {
    alts: VecDeque<(Value, Rc<Env>)>,
    k: Cont,
    trail_len: usize,
}

/// The next step a resume loop takes: one pending alternative of the
/// deepest frame, or the frame's exhaustion.
enum Job {
    Alternative {
        expr: Value,
        env: Rc<Env>,
        k: Cont,
        trail_len: usize,
    },
    Exhausted(Box<AmbFrame>),
}

/// Whether a search runs its alternatives in written order or in the
/// order the seeded xorshift draws.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Search {
    InOrder,
    Shuffled,
}

/// Whether an assignment lands on the undo trail: the book's `set!`
/// versus exercise 4.51's `permanent-set!`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Assignment {
    Undoable,
    Permanent,
}

/// The `amb` evaluator: the choice stack, the undo trail, and the
/// seeded xorshift `ramb` draws from. All state is per-instance and
/// behind interior mutability, so the seed is threaded through
/// construction and no evaluator state is global.
pub struct Amb {
    stack: RefCell<Vec<AmbFrame>>,
    trail: RefCell<Vec<Undo>>,
    rng: RefCell<Random>,
    failures: Cell<u64>,
    hook: RefCell<Option<SpecialHook>>,
}

impl Amb {
    /// Creates an evaluator whose `ramb` searches from `seed`.
    ///
    /// # Errors
    /// [`SchemeError::ZeroSeed`] when `seed` is zero, because
    /// `xorshift64*` never leaves the zero state.
    pub fn new(seed: u64) -> Result<Self, SchemeError> {
        Ok(Self {
            stack: RefCell::new(Vec::new()),
            trail: RefCell::new(Vec::new()),
            rng: RefCell::new(Random::new(seed)?),
            failures: Cell::new(0),
            hook: RefCell::new(None),
        })
    }

    /// Installs an extension's dispatch clause: the engine consults it
    /// first for every expression it evaluates, at every nesting depth.
    /// Exercise 4.54's variant installs one clause for `require`.
    pub fn install_special_hook(&self, hook: SpecialHook) {
        *self.hook.borrow_mut() = Some(hook);
    }

    /// The number of failures the search has delivered to choice
    /// frames: one per failed alternative replay and one per frame
    /// exhaustion, the count exercises 4.37, 4.39, and 4.40 compare.
    #[must_use]
    pub fn failures(&self) -> u64 {
        self.failures.get()
    }

    /// Evaluates `exp` in `env` and hands the value to `k`: the entry
    /// an extension's clause uses to run the rest of the computation,
    /// the book's `ambeval` with one continuation. `require` is no
    /// clause here -- the base language installs it as an ordinary
    /// procedure -- so exercise 4.54's variant recognizes it before
    /// dispatching everything else through this entry.
    ///
    /// # Errors
    /// Whatever the expression's evaluation raises.
    pub fn eval_with(&self, exp: &Value, env: &Rc<Env>, k: Cont) -> EvalResult {
        let hook = self.hook.borrow().clone();
        if let Some(result) = hook.and_then(|hook| hook(self, exp, env, &k)) {
            return result;
        }
        if is_self_evaluating(exp) {
            return k(self, exp.clone());
        }
        if is_quoted(exp) {
            return k(self, text_of_quotation(exp)?);
        }
        if is_variable(exp) {
            return k(self, lookup_variable(exp, env)?);
        }
        if is_lambda(exp) {
            return k(self, Value::Closure(lambda_closure(exp, env)?));
        }
        if is_tagged_list(exp, "amb") {
            return self.search(exp, env, k, Search::InOrder);
        }
        if is_tagged_list(exp, "ramb") {
            return self.search(exp, env, k, Search::Shuffled);
        }
        if is_if(exp) {
            return self.eval_if(exp, env, &k);
        }
        if is_definition(exp) {
            return self.eval_definition(exp, env, &k);
        }
        if is_assignment(exp) {
            return self.eval_assignment(exp, env, &k, Assignment::Undoable);
        }
        if is_tagged_list(exp, "permanent-set!") {
            return self.eval_assignment(exp, env, &k, Assignment::Permanent);
        }
        if is_tagged_list(exp, "if-fail") {
            return self.eval_if_fail(exp, env, &k);
        }
        if is_begin(exp) {
            let forms = operand_items(exp)?;
            return self.eval_sequence(&forms, env, &k);
        }
        if is_cond(exp) {
            return self.eval_with(&cond_to_if(exp)?, env, k);
        }
        if is_let(exp) {
            return self.eval_with(&let_to_combination(exp)?, env, k);
        }
        if is_application(exp) {
            return self.eval_application(exp, env, &k);
        }
        Err(SchemeError::TypeMismatch(format!(
            "unknown expression type: {exp}"
        )))
    }

    /// Starts a new problem: discards the unexplored alternatives and
    /// the trail of any earlier problem, then answers the first
    /// non-failing execution of `form`.
    ///
    /// # Errors
    /// [`SchemeError::Backtrack`] when every execution fails; whatever
    /// the evaluation raises otherwise.
    pub fn run_form(&self, form: &Value, env: &Rc<Env>) -> EvalResult {
        self.discard_problem();
        let answer: Cont = cont(|_, value| Ok(value));
        match self.eval_with(form, env, answer) {
            Err(error) => {
                // A completed failed problem leaves nothing in flight.
                self.discard_problem();
                Err(error)
            }
            other => other,
        }
    }

    /// Reads `text` as one form and runs it as a new problem in `env`,
    /// the driver's entry for a non-`try-again` line.
    ///
    /// # Errors
    /// The reader's parse error, or whatever [`Amb::run_form`] raises.
    pub fn run(&self, text: &str, env: &Rc<Env>) -> EvalResult {
        let form = sicp_runtime::read(text)?;
        self.run_form(&form, env)
    }

    /// Runs every form of `text` in `env`, each as a new problem, and
    /// answers the last value: the entry that loads a library of
    /// definitions before a session.
    ///
    /// # Errors
    /// The reader's parse errors and the first evaluation error.
    pub fn run_program(&self, text: &str, env: &Rc<Env>) -> EvalResult {
        let mut answer = Ok(Value::sym("ok"));
        for form in sicp_runtime::read_program(text)? {
            answer = self.run_form(&form, env);
            if answer.is_err() {
                break;
            }
        }
        answer
    }

    /// Yields the next answer of the problem in flight: the deepest
    /// pending choice resumes its next alternative and continues,
    /// without replaying the work earlier answers already did. Exhausted
    /// frames unwind in order, undoing their trail segments.
    ///
    /// # Errors
    /// [`SchemeError::Backtrack`] when no choice is pending any more --
    /// either the search ran dry or no problem is in flight; whatever
    /// the resumed computation raises otherwise.
    pub fn try_again(&self) -> EvalResult {
        while let Some(job) = self.next_job() {
            match job {
                Job::Alternative {
                    expr,
                    env,
                    k,
                    trail_len,
                } => {
                    // Rolling the trail back to the frame's push line
                    // discards the mutations of the path this resume
                    // leaves behind -- the answered path, or the failed
                    // alternative before it.
                    self.undo_to(trail_len);
                    let result = self.eval_with(&expr, &env, k);
                    if let Err(SchemeError::Backtrack) = result {
                        self.count_failure();
                        self.undo_to(trail_len);
                    } else {
                        return result;
                    }
                }
                Job::Exhausted(frame) => {
                    self.count_failure();
                    self.undo_to(frame.trail_len);
                }
            }
        }
        Err(SchemeError::Backtrack)
    }

    /// The engine's failure signal: the book's `(amb)` and a `require`
    /// that does not hold both raise it, and the nearest choice frame
    /// catches it.
    ///
    /// # Errors
    /// Always [`SchemeError::Backtrack`].
    pub fn fail(&self) -> EvalResult {
        Err(SchemeError::Backtrack)
    }

    /// Pushes one choice frame holding `alts` under continuation `k`:
    /// the book's `analyze-amb` entry an extension's choice form uses.
    /// The frame must then be driven by [`Amb::drive`].
    pub fn push_frame(&self, alts: Vec<(Value, Rc<Env>)>, k: Cont) {
        let trail_len = self.trail.borrow().len();
        self.stack.borrow_mut().push(AmbFrame {
            alts: alts.into(),
            k,
            trail_len,
        });
    }

    /// Runs the newest frame's pending alternatives: evaluates each
    /// front alternative, retries through [`SchemeError::Backtrack`],
    /// and unwinds when the frame runs dry. A frame pushed by
    /// [`Amb::push_frame`] is driven exactly here: the loop owns it
    /// until it exhausts, and the exhausted unwind is what makes the
    /// enclosing `amb` site's own loop try its next alternative.
    ///
    /// # Errors
    /// [`SchemeError::Backtrack`] when the frame's alternatives run
    /// dry; whatever the chosen alternative raises otherwise.
    pub fn drive(&self) -> EvalResult {
        while let Some(job) = self.next_job() {
            match job {
                Job::Alternative {
                    expr,
                    env,
                    k,
                    trail_len,
                } => {
                    // A no-op for the first alternative (the trail ends
                    // at the push line) and the rollback of the failed
                    // alternative's mutations for every later one.
                    self.undo_to(trail_len);
                    let result = self.eval_with(&expr, &env, k);
                    if let Err(SchemeError::Backtrack) = result {
                        self.count_failure();
                        self.undo_to(trail_len);
                    } else {
                        return result;
                    }
                }
                Job::Exhausted(frame) => {
                    self.count_failure();
                    self.undo_to(frame.trail_len);
                    return Err(SchemeError::Backtrack);
                }
            }
        }
        Err(SchemeError::Backtrack)
    }

    /// Records `old` on the trail for the binding `name` in `env`, the
    /// book's `*2*` failure continuation made data.
    pub fn record_undo(&self, env: &Rc<Env>, name: &Symbol, old: Value) {
        self.trail.borrow_mut().push(Undo {
            env: Rc::clone(env),
            name: Rc::clone(name),
            old,
        });
    }

    /// The seeded xorshift draw of `(random n)` for `0 <= n`: what
    /// `ramb` shuffles with, threaded through the evaluator's seed.
    ///
    /// # Panics
    /// Panics when `n` is zero, as `(random 0)` does in the book's
    /// Scheme.
    #[must_use]
    pub fn random(&self, n: u64) -> u64 {
        self.rng.borrow_mut().random(n)
    }

    /// Applies a procedure value to evaluated arguments: primitives run
    /// their handler, compound procedures extend their captured
    /// environment with the frame and run the body against the
    /// continuation.
    ///
    /// # Errors
    /// [`SchemeError::NotProcedure`] when `proc` is neither kind;
    /// whatever the handler or the body raises.
    pub fn apply_procedure(&self, proc: &Value, args: &[Value], k: &Cont) -> EvalResult {
        match proc {
            Value::Primitive { f, .. } => k(self, f(args)?),
            Value::Closure(closure) => {
                let name = closure.name.as_deref().unwrap_or("#[compound-procedure]");
                let frame = extend_environment(
                    name,
                    &closure.params,
                    closure.rest.as_ref(),
                    args,
                    &closure.env,
                )?;
                self.eval_sequence(&closure.body, &frame, k)
            }
            other => Err(SchemeError::NotProcedure(other.clone())),
        }
    }

    /// The book's `analyze-sequence` and `sequentially`: each form but
    /// the last evaluates against a continuation that runs the rest, so
    /// a choice inside an early form resumes into the later forms.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on an empty sequence; whatever a
    /// form's evaluation raises.
    pub fn eval_sequence(&self, forms: &[Value], env: &Rc<Env>, k: &Cont) -> EvalResult {
        let Some((head, tail)) = forms.split_first() else {
            return Err(SchemeError::TypeMismatch(
                "Empty sequence: ANALYZE".to_owned(),
            ));
        };
        if tail.is_empty() {
            return self.eval_with(head, env, Rc::clone(k));
        }
        let rest: Cont = {
            let tail = tail.to_vec();
            let env = Rc::clone(env);
            let k = Rc::clone(k);
            cont(move |amb, _| amb.eval_sequence(&tail, &env, &k))
        };
        self.eval_with(head, env, rest)
    }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

    fn discard_problem(&self) {
        self.stack.borrow_mut().clear();
        self.trail.borrow_mut().clear();
    }

    fn count_failure(&self) {
        self.failures.set(self.failures.get() + 1);
    }

    fn next_job(&self) -> Option<Job> {
        let mut stack = self.stack.borrow_mut();
        let frame = stack.last_mut()?;
        if let Some((expr, env)) = frame.alts.pop_front() {
            return Some(Job::Alternative {
                expr,
                env,
                k: Rc::clone(&frame.k),
                trail_len: frame.trail_len,
            });
        }
        let frame = stack.pop()?;
        Some(Job::Exhausted(Box::new(frame)))
    }

    /// Rolls the trail back to `limit`: each entry newer than the limit
    /// restores the value its assignment overwrote. The entry exists
    /// only because the binding held `old` when the assignment read it,
    /// so a restore cannot walk off the chain.
    fn undo_to(&self, limit: usize) {
        let mut trail = self.trail.borrow_mut();
        while trail.len() > limit {
            let Some(undo) = trail.pop() else {
                break;
            };
            undo.env
                .set(&undo.name, undo.old)
                .expect("trail entry restores its own binding");
        }
    }

    /// The `amb`/`ramb` choice point: pushes the frame of pending
    /// alternatives -- `ramb` after shuffling the order the seeded
    /// xorshift draws -- and drives it.
    fn search(&self, exp: &Value, env: &Rc<Env>, k: Cont, order: Search) -> EvalResult {
        let choices = operand_items(exp)?;
        let mut alts: Vec<_> = choices
            .iter()
            .map(|choice| (choice.clone(), Rc::clone(env)))
            .collect();
        if order == Search::Shuffled {
            let mut rng = self.rng.borrow_mut();
            for i in (1..alts.len()).rev() {
                let bound = u64::try_from(i + 1).expect("an alternative count fits u64");
                let draw = rng.random(bound);
                let j = usize::try_from(draw).expect("the draw lands below the count");
                alts.swap(i, j);
            }
        }
        self.push_frame(alts, k);
        self.drive()
    }

    /// The book's `analyze-if`: the predicate evaluates against a
    /// continuation that picks the branch, so a choice inside the
    /// predicate resumes into the branch it chose.
    fn eval_if(&self, exp: &Value, env: &Rc<Env>, k: &Cont) -> EvalResult {
        let (predicate, consequent, alternative) = if_parts(exp)?;
        let branch: Cont = {
            let env = Rc::clone(env);
            let k = Rc::clone(k);
            cont(move |amb, tested| {
                let chosen = if is_true(&tested) {
                    consequent.clone()
                } else {
                    alternative.clone()
                };
                amb.eval_with(&chosen, &env, Rc::clone(&k))
            })
        };
        self.eval_with(&predicate, env, branch)
    }

    /// The book's `analyze-definition`: the value expression evaluates
    /// first; the continuation binds the name and answers `ok`.
    fn eval_definition(&self, exp: &Value, env: &Rc<Env>, k: &Cont) -> EvalResult {
        let name = variable_name(&definition_variable(exp)?)?;
        let value_exp = definition_value(exp)?;
        let bind: Cont = {
            let env = Rc::clone(env);
            let k = Rc::clone(k);
            cont(move |amb, value| {
                env.define(Rc::clone(&name), value);
                k(amb, Value::sym("ok"))
            })
        };
        self.eval_with(&value_exp, env, bind)
    }

    /// The book's `analyze-assignment`: the continuation saves the old
    /// value, assigns, and -- for [`Assignment::Undoable`], the book's
    /// `*1*`/`*2*` pair -- records the undo entry backtracking rolls
    /// back. [`Assignment::Permanent`], exercise 4.51's
    /// `permanent-set!`, skips the entry and survives backtracking.
    fn eval_assignment(
        &self,
        exp: &Value,
        env: &Rc<Env>,
        k: &Cont,
        kind: Assignment,
    ) -> EvalResult {
        let name = variable_name(&assignment_variable(exp)?)?;
        let value_exp = assignment_value(exp)?;
        let assign: Cont = {
            let env = Rc::clone(env);
            let k = Rc::clone(k);
            cont(move |amb, value| {
                let old = env.lookup(&name)?;
                env.set(&name, value)?;
                if kind == Assignment::Undoable {
                    amb.record_undo(&env, &name, old);
                }
                k(amb, Value::sym("ok"))
            })
        };
        self.eval_with(&value_exp, env, assign)
    }

    /// Exercise 4.52's `if-fail`: the first expression evaluates
    /// against the continuation as usual; when its whole search runs
    /// dry -- the `Backtrack` that survives it -- the second expression
    /// evaluates against the same continuation instead.
    fn eval_if_fail(&self, exp: &Value, env: &Rc<Env>, k: &Cont) -> EvalResult {
        let operands = operand_items(exp)?;
        let [first, second] = operands.as_slice() else {
            return Err(SchemeError::WrongArity {
                procedure: "if-fail".to_owned(),
                expected: "2".to_owned(),
                got: operands.len(),
            });
        };
        match self.eval_with(first, env, Rc::clone(k)) {
            Err(SchemeError::Backtrack) => self.eval_with(second, env, Rc::clone(k)),
            other => other,
        }
    }

    /// The book's `analyze-application`, `get-args`, and
    /// `execute-application`: the operator evaluates first, then the
    /// operands walk left to right, each against a continuation that
    /// carries the operator, the arguments already obtained, and the
    /// operand expressions still to evaluate -- the data a resumed
    /// search re-enters without replaying the walk.
    fn eval_application(&self, exp: &Value, env: &Rc<Env>, k: &Cont) -> EvalResult {
        let operator = first_of(exp)?;
        let operands = operand_items(exp)?;
        let walk: Cont = {
            let operands = operands.clone();
            let env = Rc::clone(env);
            let k = Rc::clone(k);
            cont(move |amb, proc| continue_args(amb, proc, &[], &operands, &env, &k))
        };
        self.eval_with(&operator, env, walk)
    }
}

/// Walks an application's operand list left to right: each argument
/// evaluates against a continuation that carries the operator, the
/// arguments already obtained, and the operand expressions still to
/// evaluate. The state is a snapshot per step, so a resumed search can
/// re-enter one step many times without carrying an earlier step's
/// values over.
fn continue_args(
    amb: &Amb,
    proc: Value,
    done: &[Value],
    rest: &[Value],
    env: &Rc<Env>,
    k: &Cont,
) -> EvalResult {
    let Some((head, tail)) = rest.split_first() else {
        return amb.apply_procedure(&proc, done, k);
    };
    let step: Cont = {
        let done = done.to_vec();
        let tail = tail.to_vec();
        let env = Rc::clone(env);
        let k = Rc::clone(k);
        cont(move |amb, value| {
            let mut args = done.clone();
            args.push(value);
            continue_args(amb, proc.clone(), &args, &tail, &env, &k)
        })
    };
    amb.eval_with(head, env, step)
}

/// The symbol of a variable form.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the form is not a symbol.
fn variable_name(exp: &Value) -> Result<Symbol, SchemeError> {
    match exp {
        Value::Sym(name) => Ok(Rc::clone(name)),
        other => Err(SchemeError::TypeMismatch(format!(
            "not a variable name: {other}"
        ))),
    }
}

/// The book's `lookup-variable-value` for a variable form.
///
/// # Errors
/// [`SchemeError::UnboundVariable`] when no frame binds the name;
/// [`SchemeError::TypeMismatch`] when the form is not a symbol.
fn lookup_variable(exp: &Value, env: &Rc<Env>) -> EvalResult {
    let name = variable_name(exp)?;
    env.lookup(&name)
}

/// Builds the closure of a `lambda` form.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the form is malformed.
fn lambda_closure(exp: &Value, env: &Rc<Env>) -> Result<Rc<Closure>, SchemeError> {
    let items = operand_items(exp)?;
    let [params_form, body @ ..] = items.as_slice() else {
        return Err(SchemeError::TypeMismatch(
            "lambda with no parameters".to_owned(),
        ));
    };
    let (params, rest) = split_params(params_form)?;
    Ok(Rc::new(Closure {
        name: None,
        params,
        rest,
        body: body.to_vec(),
        env: Rc::clone(env),
    }))
}

// ---------------------------------------------------------------------------
// 4.1.4: the section table, the global environment, and the drivers.
// ---------------------------------------------------------------------------

/// The section's primitive table: the 4.1 table composed unchanged, the
/// same composition point the lazy section uses.
#[must_use]
pub fn amb_table(sink: &OutputSink) -> Vec<(&'static str, Handler)> {
    primitive_table(sink)
}

/// [`setup_environment`](crate::sec_4_1::setup_environment) for the
/// nondeterministic language: the composed section table plus the
/// `true`/`false` bindings, with the object language's `display` and
/// `newline` writing to the host's standard output.
#[must_use]
pub fn setup_amb_environment() -> Rc<Env> {
    setup_amb_environment_in(&OutputSink::Stdout)
}

/// [`setup_amb_environment`] with the object language's `display` and
/// `newline` writing into `sink`.
#[must_use]
pub fn setup_amb_environment_in(sink: &OutputSink) -> Rc<Env> {
    let env = Env::global();
    env.define(Rc::from("true"), Value::boolean(true));
    env.define(Rc::from("false"), Value::boolean(false));
    for (name, handler) in amb_table(sink) {
        env.define(
            Rc::from(name),
            Value::Primitive {
                name: Rc::from(name),
                f: handler,
            },
        );
    }
    env
}

/// The driver's input prompt.
pub const INPUT_PROMPT: &str = ";;; Amb-Eval input: ";

/// The driver's output prompt.
pub const OUTPUT_PROMPT: &str = ";;; Amb-Eval value: ";

/// The driver's new-problem line.
pub const NEW_PROBLEM_PROMPT: &str = ";;; Starting a new problem";

/// The driver's exhaustion line; the form of the exhausted problem
/// prints on the next line, the book's `user-print` of the input.
pub const NO_MORE_PROMPT: &str = ";;; There are no more values of";

/// The driver's no-problem line, the answer to a `try-again` when no
/// problem is in flight.
pub const NO_PROBLEM_PROMPT: &str = ";;; There is no current problem";

/// Runs `lines` the way the book's driver loop presents a session: the
/// `Amb-Eval` prompts, `;;; Starting a new problem` before each new
/// form, and a `try-again` line resuming the problem in flight. A
/// search that runs dry prints the exhaustion line and the form; a
/// `try-again` with no problem prints the no-problem line; any other
/// error prints one `Error:` line and ends the session.
#[must_use]
pub fn amb_transcript(amb: &Amb, lines: &[&str]) -> String {
    let (sink, cell) = OutputSink::buffer();
    let env = setup_amb_environment_in(&sink);
    let mut out = String::new();
    let mut pending: Option<Value> = None;
    for line in lines {
        out.push_str(INPUT_PROMPT);
        out.push_str(line);
        out.push('\n');
        if line.trim() == "try-again" {
            match amb.try_again() {
                Ok(value) => push_value(&mut out, &value),
                Err(SchemeError::Backtrack) => {
                    if let Some(form) = pending.take() {
                        push_no_more(&mut out, &form);
                    } else {
                        out.push_str(NO_PROBLEM_PROMPT);
                        out.push('\n');
                    }
                }
                Err(error) => return push_error(&out, &error),
            }
            continue;
        }
        out.push_str(NEW_PROBLEM_PROMPT);
        out.push('\n');
        let form = match sicp_runtime::read(line) {
            Ok(form) => form,
            Err(error) => return push_error(&out, &error),
        };
        match amb.run_form(&form, &env) {
            Ok(value) => {
                push_value(&mut out, &value);
                pending = Some(form);
            }
            Err(SchemeError::Backtrack) => {
                pending = None;
                push_no_more(&mut out, &form);
            }
            Err(error) => return push_error(&out, &error),
        }
    }
    drop(cell);
    out
}

fn push_value(out: &mut String, value: &Value) {
    out.push_str(OUTPUT_PROMPT);
    out.push_str(&print_value(value));
    out.push('\n');
}

fn push_no_more(out: &mut String, form: &Value) {
    out.push_str(NO_MORE_PROMPT);
    out.push('\n');
    out.push_str(&print_value(form));
    out.push('\n');
}

fn push_error(head: &str, error: &SchemeError) -> String {
    format!("{head}Error: {error}\n")
}

/// Evaluates every form of `program` in one fresh global environment,
/// each form as a new problem, and answers the values and the displayed
/// text. Definitions answer `ok`; a form whose whole search fails ends
/// the run with [`SchemeError::Backtrack`].
///
/// # Errors
/// The reader's parse errors and the first evaluation error.
pub fn run_amb(amb: &Amb, program: &str) -> Result<(Vec<Value>, String), SchemeError> {
    let (sink, cell) = OutputSink::buffer();
    let env = setup_amb_environment_in(&sink);
    let mut values = Vec::new();
    for form in sicp_runtime::read_program(program)? {
        values.push(amb.run_form(&form, &env)?);
    }
    let displayed = cell.borrow().clone();
    Ok((values, displayed))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The section's library lines, one driver input each.
    const LIBRARY_LINES: &[&str] = &[
        "(define (require p) (if (not p) (amb)))",
        "(define (an-element-of items) \
         (require (not (null? items))) \
         (amb (car items) (an-element-of (cdr items))))",
    ];

    #[test]
    fn choice_point_answers_in_book_order() {
        let amb = Amb::new(20_260_925).expect("seed");
        let session = amb_transcript(
            &amb,
            &[
                LIBRARY_LINES[0],
                LIBRARY_LINES[1],
                "(list (amb 1 2 3) (amb 'a 'b))",
                "try-again",
                "try-again",
                "try-again",
                "try-again",
                "try-again",
                "try-again",
                "try-again",
            ],
        );
        for value in ["(1 a)", "(1 b)", "(2 a)", "(2 b)", "(3 a)", "(3 b)"] {
            assert!(
                session.contains(&format!("{OUTPUT_PROMPT}{value}\n")),
                "missing {value} in: {session}"
            );
        }
        assert!(
            session.contains(&format!(
                "{NO_MORE_PROMPT}\n(list (amb 1 2 3) (amb (quote a) (quote b)))\n"
            )),
            "session: {session}"
        );
        // Exhaustion ends the problem, so one more try-again reports
        // exactly what the book's driver reports.
        assert!(
            session.ends_with(&format!("{NO_PROBLEM_PROMPT}\n")),
            "session: {session}"
        );
    }

    #[test]
    fn driver_sample_interaction_matches_the_book() {
        let amb = Amb::new(20_260_925).expect("seed");
        let env = setup_amb_environment();
        let library = "
            (define (require p) (if (not p) (amb)))
            (define (an-element-of items)
              (require (not (null? items)))
              (amb (car items) (an-element-of (cdr items))))
            (define (prime? n)
              (define (smallest-divisor test)
                (if (> (* test test) n) n
                    (if (= (remainder n test) 0) test
                        (smallest-divisor (+ test 1)))))
              (= (smallest-divisor 2) n))
            (define (prime-sum-pair list1 list2)
              (let ((a (an-element-of list1)) (b (an-element-of list2)))
                (require (prime? (+ a b)))
                (list a b)))
        ";
        amb.run_program(library, &env).expect("library");
        let first = amb
            .run("(prime-sum-pair '(1 3 5 8) '(20 35 110))", &env)
            .expect("first answer");
        assert_eq!(print_value(&first), "(3 20)");
        assert_eq!(print_value(&amb.try_again().expect("second")), "(3 110)");
        assert_eq!(print_value(&amb.try_again().expect("third")), "(8 35)");
        assert!(matches!(amb.try_again(), Err(SchemeError::Backtrack)));
        // A new problem discards the exhausted search and answers afresh.
        let next = amb
            .run("(prime-sum-pair '(19 27 30) '(11 36 58))", &env)
            .expect("new problem");
        assert_eq!(print_value(&next), "(30 11)");
    }

    #[test]
    fn assignment_undoes_on_backtrack_and_permanent_survives() {
        let env = setup_amb_environment();
        let undoing = "
            (define (require p) (if (not p) (amb)))
            (define (an-element-of items)
              (require (not (null? items)))
              (amb (car items) (an-element-of (cdr items))))
            (define count 0)
            (let ((x (an-element-of '(1 2))))
              (set! count (+ count 1))
              (require (= x 1))
              x)
        ";
        let amb = Amb::new(20_260_925).expect("seed");
        let (values, _) = run_amb(&amb, undoing).expect("set! run");
        assert_eq!(print_value(values.last().expect("value")), "1");
        // The same search resumed in one environment: unwinding the
        // failed branch rolls every set! on the path back, so the count
        // keeps the value it had before the problem ran. A surviving
        // increment would read 3 (or 2 after the failed branch's
        // rollback alone).
        let amb2 = Amb::new(20_260_925).expect("seed");
        amb2.run_program(undoing, &env).expect("definitions");
        let first = amb2
            .run(
                "(let ((x (an-element-of '(1 2)))) \
                  (set! count (+ count 1)) (require (= x 1)) x)",
                &env,
            )
            .expect("first");
        assert_eq!(print_value(&first), "1");
        assert!(matches!(amb2.try_again(), Err(SchemeError::Backtrack)));
        assert_eq!(print_value(&amb2.run("count", &env).expect("probe")), "1");
        // permanent-set! skips the trail, so the failed branch's
        // increment survives the same unwind.
        amb2.run_program("(define count2 0)", &env).expect("reset");
        let kept = "
            (let ((x (an-element-of '(1 2))))
              (permanent-set! count2 (+ count2 1))
              (require (= x 1))
              x)
        ";
        let first = amb2.run(kept, &env).expect("first");
        assert_eq!(print_value(&first), "1");
        assert!(matches!(amb2.try_again(), Err(SchemeError::Backtrack)));
        assert_eq!(print_value(&amb2.run("count2", &env).expect("probe")), "2");
    }

    #[test]
    fn if_fail_catches_a_dry_search() {
        let library = "
            (define (require p) (if (not p) (amb)))
            (define (even? n) (= (remainder n 2) 0))
            (define (an-element-of items)
              (require (not (null? items)))
              (amb (car items) (an-element-of (cdr items))))
        ";
        let amb = Amb::new(20_260_925).expect("seed");
        let env = setup_amb_environment();
        amb.run_program(library, &env).expect("library");
        let odd = amb
            .run(
                "(if-fail (let ((x (an-element-of '(1 3 5)))) \
                 (require (even? x)) x) 'all-odd)",
                &env,
            )
            .expect("all-odd");
        assert_eq!(print_value(&odd), "all-odd");
        let even = amb
            .run(
                "(if-fail (let ((x (an-element-of '(1 3 5 8)))) \
                 (require (even? x)) x) 'all-odd)",
                &env,
            )
            .expect("eight");
        assert_eq!(print_value(&even), "8");
    }

    #[test]
    fn ramb_replays_from_the_same_seed() {
        let program = "
            (define (parse-word word-list)
              (list (car word-list)
                    (ramb (car (cdr word-list))
                          (car (cdr (cdr word-list)))
                          (car (cdr (cdr (cdr word-list)))))))
            (parse-word '(article the a some))
        ";
        let left = Amb::new(20_260_925).expect("seed");
        let right = Amb::new(20_260_925).expect("seed");
        let (left_values, _) = run_amb(&left, program).expect("left");
        let (right_values, _) = run_amb(&right, program).expect("right");
        assert_eq!(left_values, right_values);
        let other = Amb::new(99).expect("seed");
        let (other_values, _) = run_amb(&other, program).expect("other");
        assert_ne!(left_values, other_values);
    }
}
