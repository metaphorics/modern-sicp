// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.2

//! Section 4.2: the lazy evaluator on the 4.1 substrate. The section's
//! language is Scheme except that compound procedures are non-strict in
//! each argument: applying one delays its operands into thunks, and a
//! thunk's expression evaluates only when its value is demanded -- by a
//! strict primitive, an `if` predicate, an operator position, or the
//! driver loop before printing. Thunks are [`Value::Thunk`] cells shared
//! by `Rc`, and memoization is the cell's one-time fill, so the book's
//! `thunk`/`evaluated-thunk` pair collapses into the two states of one
//! cell ([`ThunkState`]).
//!
//! The seam from 4.1 is the extension mechanism: [`Lazy`] implements the
//! one [`Evaluator::step`] hook, reroutes the two clauses the section
//! changes -- the application clause and the `if` predicate -- through
//! [`lazy_step`], and falls back to `base_step` for the clause chain the
//! section leaves alone (`eval_sequence` among them, which exercise 4.30
//! debates). The object language's primitives stay strict; quoted pairs
//! stay ordinary data. The exercise variants build on the same hook:
//! the recomputing force of 4.29, Cy's forcing sequence of 4.30, the
//! declared parameters of 4.31, the lifted quotes of 4.33, and the
//! printable lazy pairs of 4.34 all implement the [`LazyEval`]
//! discipline.
//!
//! # The section table
//!
//! [`lazy_table`] composes the 4.1 primitive table unchanged: the
//! book's interactions need `/` and variadic mixed int/float arithmetic
//! (`try`'s `(/ 1 0)`, `scale-list`'s `*`, `solve`'s float `dt`), and
//! the 4.1 handlers already fold variadic operands exact-while-exact
//! and float the moment one is inexact. A lazy-specific entry would
//! extend the composed vector here, and a later object-language define
//! of the same name overwrites the binding, which is how the 4.2.3
//! session replaces `cons`, `car`, and `cdr` with the procedural pair.
//!
//! # Deep recursion
//!
//! The tail-position trampoline of 4.1 carries over: the branches of an
//! `if` and the last expression of an applied body return as
//! [`Step::Tail`], so `list-ref` over the section's lazy `integers` and
//! the 1000-step `solve` run in constant host stack. Deep lazy
//! recursion does not overflow where the book's Scheme would not; the
//! 256 MiB worker stack of
//! [`with_eval_stack`](crate::sec_4_1::with_eval_stack) remains the
//! postponement for non-tail recursion.

use std::cell::RefCell;
use std::rc::Rc;

use sicp_runtime::{
    Closure, ConsCell, Env, Handler, SchemeError, ThunkState, Value, cons_cell, print_value,
};

use crate::sec_4_1::{
    EvalResult, Evaluator, OutputSink, Step, StepResult, extend_environment, first_of, if_parts,
    is_application, is_assignment, is_begin, is_cond, is_definition, is_if, is_lambda, is_let,
    is_quoted, is_true, operand_items,
};

/// The lazy layer's discipline, shared by the section evaluator and the
/// exercise variants: how a value in hand is forced, and how operands
/// bind under a compound procedure.
pub trait LazyEval: Evaluator {
    /// Forces a value in hand: the section's memoized thunks compute
    /// once and serve the stored value; the recomputing wrappers of the
    /// no-memo probes (4.29, 4.31) evaluate again at every demand.
    ///
    /// # Errors
    /// Whatever the thunk's expression raises.
    fn force_value(&self, value: Value) -> EvalResult;

    /// Answers whether operand `position` of `proc` is delayed when the
    /// compound procedure is applied, and if so whether forcing
    /// memoizes. `None` binds the operand's evaluated value now -- the
    /// upward-compatible strict default of exercise 4.31.
    fn delay_operand(&self, proc: &Rc<Closure>, position: usize) -> Option<bool>;

    /// The book's `actual-value`: evaluates and then forces, so a
    /// delayed value never crosses a demand site.
    ///
    /// # Errors
    /// Whatever the evaluation or the forcing raises.
    fn actual_value(&self, exp: &Value, env: &Rc<Env>) -> EvalResult {
        let value = self.eval(exp, env)?;
        self.force_value(value)
    }
}

/// The section's two changed clauses: the application clause (force the
/// operator, delay the operands of a compound procedure) and the `if`
/// (force the predicate). Everything else falls back to the base clause
/// chain, so a lazy variant reroutes only what the section reroutes.
///
/// # Errors
/// Whatever the expression's evaluation raises.
pub fn lazy_step(ev: &impl LazyEval, exp: &Value, env: &Rc<Env>) -> StepResult {
    if is_if(exp) {
        return forced_if(ev, exp, env);
    }
    if is_application(exp) && !is_special_form(exp) {
        return lazy_application(ev, exp, env);
    }
    ev.base_step(exp, env)
}

/// Whether the pair is one of the base grammar's special forms, which
/// the base clause chain must see before any application clause does.
#[must_use]
pub fn is_special_form(exp: &Value) -> bool {
    is_quoted(exp)
        || is_assignment(exp)
        || is_definition(exp)
        || is_lambda(exp)
        || is_begin(exp)
        || is_cond(exp)
        || is_let(exp)
}

/// The book's `eval-if`: the predicate is forced with `actual-value`
/// before the truth test, and the chosen branch runs through the
/// driver, so it stays a tail position.
///
/// # Errors
/// Whatever the predicate or the chosen branch raises.
fn forced_if(ev: &impl LazyEval, exp: &Value, env: &Rc<Env>) -> StepResult {
    let (predicate, consequent, alternative) = if_parts(exp)?;
    let tested = ev.actual_value(&predicate, env)?;
    let branch = if is_true(&tested) {
        consequent
    } else {
        alternative
    };
    Ok(Step::Tail(branch, Rc::clone(env)))
}

/// The book's changed application clause: the operator is forced (so a
/// procedure value can itself arrive delayed), a compound procedure's
/// operands are delayed, and a strict primitive's operands are forced.
///
/// # Errors
/// Whatever the application raises.
fn lazy_application(ev: &impl LazyEval, exp: &Value, env: &Rc<Env>) -> StepResult {
    let operator = first_of(exp)?;
    let operands = operand_items(exp)?;
    let proc = ev.actual_value(&operator, env)?;
    match &proc {
        Value::Closure(closure) => apply_delayed(ev, closure, &operands, env),
        other => {
            let args = list_of_arg_values(ev, &operands, env)?;
            ev.apply_procedure(other, &args).map(Step::Done)
        }
    }
}

/// Applies a compound procedure to delayed operands: each operand binds
/// per the evaluator's [`LazyEval::delay_operand`] answer, the frame
/// extends the captured environment, and the body runs through
/// [`Evaluator::step_sequence`], so the last expression stays a tail.
///
/// # Errors
/// Whatever the binding or the body raises.
fn apply_delayed(
    ev: &impl LazyEval,
    closure: &Rc<Closure>,
    operands: &[Value],
    env: &Rc<Env>,
) -> StepResult {
    let mut args = Vec::with_capacity(operands.len());
    for (position, operand) in operands.iter().enumerate() {
        args.push(match ev.delay_operand(closure, position) {
            Some(true) => delay_it(operand.clone(), env),
            Some(false) => delay_recomputing(operand.clone(), env),
            None => ev.actual_value(operand, env)?,
        });
    }
    let frame = extend_environment(
        closure_name(closure),
        &closure.params,
        closure.rest.as_ref(),
        &args,
        &closure.env,
    )?;
    ev.step_sequence(&closure.body, &frame)
}

/// The book's `list-of-arg-values`: forces every operand for the strict
/// primitives.
///
/// # Errors
/// Whatever the first failing operand raises.
pub fn list_of_arg_values(
    ev: &impl LazyEval,
    exps: &[Value],
    env: &Rc<Env>,
) -> Result<Vec<Value>, SchemeError> {
    let mut values = Vec::with_capacity(exps.len());
    for exp in exps {
        values.push(ev.actual_value(exp, env)?);
    }
    Ok(values)
}

/// The book's `delay-it`: packages an expression with its environment
/// as the section's memoized thunk, whose first forcing fills the cell
/// and every later forcing serves the stored value.
#[must_use]
pub fn delay_it(expr: Value, env: &Rc<Env>) -> Value {
    ThunkState::delay(expr, env)
}

/// The recomputing suspension of the no-memo probes (4.29, 4.31):
/// packages the expression with its environment under a wrapper whose
/// every forcing evaluates the expression again.
#[must_use]
pub fn delay_recomputing(expr: Value, env: &Rc<Env>) -> Value {
    Value::tagged("lazy-thunk", ThunkState::delay(expr, env))
}

/// Whether `value` is the recomputing wrapper [`delay_recomputing`]
/// builds.
#[must_use]
pub fn is_recomputing_thunk(value: &Value) -> bool {
    matches!(value, Value::Tagged { tag, .. } if &**tag == "lazy-thunk")
}

/// The payload of a recomputing wrapper; any other value answers
/// itself.
#[must_use]
pub fn wrapper_payload(value: &Value) -> &Value {
    match value {
        Value::Tagged { data, .. } => data,
        other => other,
    }
}

/// The memoized `force-it` of the section: a thunk's expression
/// evaluates once, the cell keeps the value, and later forcings of the
/// same cell -- or of any thunk the forcing produced -- return the
/// stored value without recomputation. A failed force leaves the cell
/// delayed, so the next demand tries again.
///
/// # Errors
/// Whatever the thunk's expression raises on its one evaluation.
pub fn force_memo(ev: &impl Evaluator, value: Value) -> EvalResult {
    let mut current = value;
    while let Value::Thunk(_) = current {
        current = ThunkState::force(&current, |expr, env| ev.eval(expr, env))?;
    }
    Ok(current)
}

/// The recomputing force of the no-memo probes: a delayed cell is left
/// delayed, so the next demand evaluates the expression again, and a
/// chain forces level by level until a non-thunk is in hand.
///
/// # Errors
/// Whatever the thunk's expression raises.
pub fn force_recomputing(ev: &impl Evaluator, value: Value) -> EvalResult {
    let mut current = value;
    while let Some(cell) = recompute_target(&current) {
        current = recompute_cell(ev, cell)?;
    }
    Ok(current)
}

/// The delayed cell a recomputing force evaluates next: the cell inside
/// a wrapper, the cell of a plain thunk, or `None` for a forced or
/// ordinary value.
fn recompute_target(value: &Value) -> Option<&Rc<RefCell<ThunkState>>> {
    let inner = if is_recomputing_thunk(value) {
        wrapper_payload(value)
    } else {
        value
    };
    match inner {
        Value::Thunk(cell) => Some(cell),
        _ => None,
    }
}

/// Evaluates one delayed cell without memoizing it: the state stays
/// `Delayed`, and an already `Forced` cell answers its value.
///
/// # Errors
/// Whatever the delayed expression raises.
pub fn recompute_cell(ev: &impl Evaluator, cell: &Rc<RefCell<ThunkState>>) -> EvalResult {
    let pending = match &*cell.borrow() {
        ThunkState::Forced(value) => return Ok(value.clone()),
        ThunkState::Delayed { expr, env } => (expr.clone(), Rc::clone(env)),
    };
    ev.eval(&pending.0, &pending.1)
}

/// The section's evaluator: the 4.1 clause chain with the application
/// clause and the `if` predicate rerouted for laziness, memoized
/// thunks, and strict primitives.
#[derive(Debug, Default)]
pub struct Lazy;

impl LazyEval for Lazy {
    fn force_value(&self, value: Value) -> EvalResult {
        force_memo(self, value)
    }

    fn delay_operand(&self, proc: &Rc<Closure>, _position: usize) -> Option<bool> {
        // Every parameter of a compound procedure is non-strict, with
        // memoized forcing.
        matches!(proc.as_ref(), Closure { .. }).then_some(true)
    }
}

impl Evaluator for Lazy {
    fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
        lazy_step(self, exp, env)
    }
}

fn closure_name(closure: &Closure) -> &str {
    closure.name.as_deref().unwrap_or("#[compound-procedure]")
}

// ---------------------------------------------------------------------------
// 4.2.2/4.2.3: the section table, the lazy global environment, drivers.
// ---------------------------------------------------------------------------

/// The section's primitive table: the 4.1 table composed unchanged. See
/// the module docs for why the section's extension is this composition
/// point.
#[must_use]
pub fn lazy_table(sink: &OutputSink) -> Vec<(&'static str, Handler)> {
    crate::sec_4_1::primitive_table(sink)
}

/// [`setup_environment`](crate::sec_4_1::setup_environment) for the lazy
/// language: the composed section table plus the `true`/`false`
/// bindings, with the object language's `display` and `newline` writing
/// to the host's standard output.
#[must_use]
pub fn setup_lazy_environment() -> Rc<Env> {
    setup_lazy_environment_in(&OutputSink::Stdout)
}

/// [`setup_lazy_environment`] with the object language's `display` and
/// `newline` writing into `sink`.
#[must_use]
pub fn setup_lazy_environment_in(sink: &OutputSink) -> Rc<Env> {
    let env = Env::global();
    env.define(Rc::from("true"), Value::boolean(true));
    env.define(Rc::from("false"), Value::boolean(false));
    for (name, handler) in lazy_table(sink) {
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

/// The lazy driver's input prompt.
pub const INPUT_PROMPT: &str = ";;; L-Eval input: ";

/// The lazy driver's output prompt.
pub const OUTPUT_PROMPT: &str = ";;; L-Eval value: ";

/// How a driver renders one forced value: the printer's form, or the
/// budgeted lazy-pair form of exercise 4.34.
#[derive(Clone, Copy, Debug)]
enum Render {
    /// The shared printer's value form.
    Plain,
    /// The printable driver: lazy pairs under [`LAZY_PRINT_BUDGET`].
    Budgeted,
}

impl Render {
    fn value(self, ev: &impl LazyEval, value: &Value) -> Result<String, SchemeError> {
        match self {
            Self::Plain => print_forced(ev, value),
            Self::Budgeted => print_lazy(ev, value),
        }
    }
}

/// Runs `lines` the way the book's lazy driver loop presents a session:
/// the L-Eval prompts, [`LazyEval::actual_value`] before printing, and
/// one `Error:` line ending the transcript when a form raises. One
/// fresh lazy global environment carries the whole session.
///
/// # Panics
/// Propagates a panic from the evaluation.
#[must_use]
pub fn lazy_driver_transcript(ev: &impl LazyEval, lines: &[&str]) -> String {
    lazy_session(ev, lines, Render::Plain)
}

/// [`lazy_driver_transcript`] with the printable rendering of exercise
/// 4.34: lazy pairs print their first [`LAZY_PRINT_BUDGET`] elements.
///
/// # Panics
/// Propagates a panic from the evaluation.
#[must_use]
pub fn printable_driver_transcript(ev: &impl LazyEval, lines: &[&str]) -> String {
    lazy_session(ev, lines, Render::Budgeted)
}

fn lazy_session(ev: &impl LazyEval, lines: &[&str], render: Render) -> String {
    let (sink, cell) = OutputSink::buffer();
    let env = setup_lazy_environment_in(&sink);
    for line in lines {
        announce_input(&cell, line);
        match evaluate_line(ev, line, &env, render) {
            Ok(text) => announce_output(&cell, &text),
            Err(error) => return push_lazy_error(&cell, &error),
        }
    }
    cell.borrow().clone()
}

fn announce_input(cell: &Rc<RefCell<String>>, line: &str) {
    let mut out = cell.borrow_mut();
    out.push_str(INPUT_PROMPT);
    out.push_str(line);
    out.push('\n');
}

fn announce_output(cell: &Rc<RefCell<String>>, text: &str) {
    let mut out = cell.borrow_mut();
    out.push_str(OUTPUT_PROMPT);
    out.push_str(text);
    out.push('\n');
}

/// Reads, evaluates with `actual-value`, and renders one driver line.
///
/// # Errors
/// The reader's parse error, the evaluation error, or a rendering
/// error.
fn evaluate_line(
    ev: &impl LazyEval,
    line: &str,
    env: &Rc<Env>,
    render: Render,
) -> Result<String, SchemeError> {
    let form = sicp_runtime::read(line)?;
    let value = ev.actual_value(&form, env)?;
    render.value(ev, &value)
}

fn push_lazy_error(cell: &Rc<RefCell<String>>, error: &SchemeError) -> String {
    let mut out = cell.borrow_mut();
    out.push_str("Error: ");
    out.push_str(&error.to_string());
    out.push('\n');
    drop(out);
    cell.borrow().clone()
}

// ---------------------------------------------------------------------------
// 4.34: the printable lazy pairs and the lazy-printing budget.
// ---------------------------------------------------------------------------

/// The lazy-printing budget of exercise 4.34: a lazy list prints its
/// first ten elements, each forced once, and an unprinted tail prints
/// as the ellipsis. The rule pins what the printer will never do: force
/// past the budget, or force the tail of an unprinted element.
pub const LAZY_PRINT_BUDGET: usize = 10;

/// The tagged lazy pair of exercise 4.34 -- the book's "modify the
/// representation of lazy pairs so that the evaluator can identify
/// them": a runtime pair whose two slots are the delayed operands,
/// under the tag the printer and the lazy `car`/`cdr` recognize.
#[must_use]
pub fn lazy_pair(car: Value, cdr: Value) -> Value {
    Value::tagged("lazy-pair", Value::Pair(cons_cell(car, cdr)))
}

/// The two-slot pair inside a tagged lazy pair, or `None` for any other
/// value.
#[must_use]
pub fn lazy_pair_slots(value: &Value) -> Option<&ConsCell> {
    match value {
        Value::Tagged { tag, data } if &**tag == "lazy-pair" => match &**data {
            Value::Pair(cell) => Some(cell),
            _ => None,
        },
        _ => None,
    }
}

/// Whether `exp` is an application whose operator names the `cons`
/// primitive currently installed in `env`: the syntactic guard the
/// printable variant uses so its non-strict `cons` never re-evaluates a
/// non-`cons` application.
#[must_use]
pub fn is_lazy_cons_call(exp: &Value, env: &Rc<Env>) -> bool {
    let Ok(operator) = first_of(exp) else {
        return false;
    };
    matches!(&operator, Value::Sym(name) if &**name == "cons")
        && matches!(
            env.lookup("cons"),
            Ok(Value::Primitive { ref name, .. }) if &**name == "cons"
        )
}

/// Builds the printable pair of a `(cons a b)` application: both
/// operands are delayed, and neither is forced.
///
/// # Errors
/// [`SchemeError::WrongArity`] when the application does not have
/// exactly two operands.
pub fn lazy_cons_call(exp: &Value, env: &Rc<Env>) -> EvalResult {
    let operands = operand_items(exp)?;
    let [head_exp, tail_exp] = &operands[..] else {
        return Err(SchemeError::WrongArity {
            procedure: "cons".to_owned(),
            expected: "2".to_owned(),
            got: operands.len(),
        });
    };
    Ok(lazy_pair(
        delay_it(head_exp.clone(), env),
        delay_it(tail_exp.clone(), env),
    ))
}

/// The plain driver's print rule: a propagated value is forced before
/// printing, and so is every thunk inside the pair shape the printer
/// walks -- the book's "if a delayed value is propagated back to the
/// read-eval-print loop, it will be forced before being printed", read
/// over the data the printer renders. Infinite lazy lists are exercise
/// 4.34's budget; an ordinary pair's shape is finite by construction,
/// because the strict `cons` forces its slots.
///
/// # Errors
/// Whatever forcing a printed element raises.
pub fn print_forced(ev: &impl LazyEval, value: &Value) -> Result<String, SchemeError> {
    render_forced(ev, value)
}

/// Renders one value with every thunk in the pair shape forced.
///
/// # Errors
/// Whatever forcing a printed element raises.
fn render_forced(ev: &impl LazyEval, value: &Value) -> Result<String, SchemeError> {
    let mut parts: Vec<String> = Vec::new();
    let mut cursor = value.clone();
    loop {
        let forced = ev.force_value(cursor.clone())?;
        match &forced {
            Value::Pair(pair) => {
                let element = ev.force_value(pair.car.borrow().clone())?;
                parts.push(render_forced(ev, &element)?);
                cursor = pair.cdr.borrow().clone();
            }
            Value::Nil => return Ok(format!("({})", parts.join(" "))),
            other => {
                // `other` is already forced: an atom renders as itself,
                // and a dotted tail renders inside the opened pair.
                let tail = print_value(other);
                if parts.is_empty() {
                    return Ok(tail);
                }
                return Ok(format!("({} . {tail})", parts.join(" ")));
            }
        }
    }
}

/// Renders one forced top-level value for the printable driver: lazy
/// pairs render their prefix under [`LAZY_PRINT_BUDGET`], every other
/// value per the printer.
///
/// # Errors
/// Whatever forcing a printed element raises.
pub fn print_lazy(ev: &impl LazyEval, value: &Value) -> Result<String, SchemeError> {
    let forced = ev.force_value(value.clone())?;
    render_lazy(ev, &forced, LAZY_PRINT_BUDGET)
}

/// Renders one value under `budget`: a tagged lazy pair walks its
/// spine, forcing each printed element and the tails that remain;
/// every other value renders per the printer. An element that is
/// itself a lazy pair gets the full budget again, so an infinite tree
/// of lazy pairs still prints finitely.
///
/// # Errors
/// Whatever forcing a printed element raises.
pub fn render_lazy_public(
    ev: &impl LazyEval,
    value: &Value,
    budget: usize,
) -> Result<String, SchemeError> {
    render_lazy(ev, value, budget)
}

fn render_lazy(ev: &impl LazyEval, value: &Value, budget: usize) -> Result<String, SchemeError> {
    let Some(cell) = lazy_pair_slots(value) else {
        return Ok(print_value(value));
    };
    let element = ev.force_value(cell.car.borrow().clone())?;
    let head = render_lazy(ev, &element, budget)?;
    if budget <= 1 {
        // The budget is spent: the tail stays unforced and prints as
        // the ellipsis.
        return Ok(format!("({head} ...)"));
    }
    let tail = ev.force_value(cell.cdr.borrow().clone())?;
    Ok(format!("({head}{}", render_tail(ev, &tail, budget)?))
}

/// Renders the tail of a lazy pair: another lazy pair continues the
/// spine (its opening parenthesis is spliced away), the empty list
/// closes it, an ordinary pair renders inside the same parentheses, and
/// anything else prints dotted.
///
/// # Errors
/// Whatever forcing a printed element raises.
fn render_tail(ev: &impl LazyEval, tail: &Value, budget: usize) -> Result<String, SchemeError> {
    if lazy_pair_slots(tail).is_some() {
        let rest = render_lazy(ev, tail, budget - 1)?;
        // The continuation loses its own opening parenthesis: the
        // spine shares the pair's parentheses.
        let continuation = &rest[1..];
        return Ok(format!(" {continuation}"));
    }
    if tail.is_nil() {
        return Ok(")".to_owned());
    }
    let printed = print_value(tail);
    if tail.is_pair() {
        // An ordinary tail renders inside the same parentheses.
        let inside = &printed[1..printed.len() - 1];
        return Ok(format!(" {inside})"));
    }
    Ok(format!(" . {printed})"))
}

/// The lifted quote of exercise 4.33: a quotation of a non-empty
/// proper list rewrites into the `cons` chain that builds the same
/// elements as lazy pairs; `None` leaves any other datum (atoms, the
/// empty list, dotted tails) ordinary.
///
/// # Errors
/// Never fails; the dotted-tail case answers `None`.
pub fn lifted_quote(datum: &Value) -> Result<Option<Value>, SchemeError> {
    let Ok(items) = datum.list_items() else {
        return Ok(None);
    };
    if items.is_empty() {
        return Ok(None);
    }
    let mut form = Value::list(vec![Value::sym("quote"), Value::Nil]);
    for item in items.into_iter().rev() {
        form = Value::list(vec![
            Value::sym("cons"),
            Value::list(vec![Value::sym("quote"), item]),
            form,
        ]);
    }
    Ok(Some(form))
}

#[cfg(test)]
mod tests {
    use super::{Lazy, lazy_driver_transcript, printable_driver_transcript};
    use crate::eval_support::{LazyPrintable, run_lazy};
    use sicp_runtime::{SchemeError, print_value};

    fn lazy_values(program: &str) -> Result<Vec<String>, SchemeError> {
        // run_lazy forces each form the way the driver prints; eval
        // alone answers thunks for values nobody demanded.
        let (values, _) = run_lazy(&Lazy, program)?;
        Ok(values.iter().map(print_value).collect())
    }

    #[test]
    fn lazy_eval_defers_armed_operands() {
        // `(/ 1 0)` is never demanded: `try` returns 1 without it.
        assert_eq!(
            lazy_values("(define (try a b) (if (= a 0) 1 b))\n(try 0 (/ 1 0))"),
            Ok(vec!["ok".to_owned(), "1".to_owned()])
        );
    }

    #[test]
    fn strict_primitives_still_force() {
        assert_eq!(lazy_values("(+ 1 (* 2 3))"), Ok(vec!["7".to_owned()]));
        let error = run_lazy(&Lazy, "(car '())").expect_err("car of () raises");
        assert!(error.to_string().contains("car"));
    }

    #[test]
    fn memoized_thunk_computes_once() {
        let lines = lazy_values(
            "(define count 0)\n\
             (define (id x) (set! count (+ count 1)) x)\n\
             (define w (id (id 10)))\n\
             count\nw\ncount\nw\ncount",
        )
        .expect("runs");
        // The define runs id's set! once; the inner call stays a thunk
        // until `w` is displayed, and the memoized re-display adds
        // nothing.
        assert_eq!(&lines[3..], &["1", "10", "2", "10", "2"]);
    }

    #[test]
    fn tail_positions_stay_flat_through_thunks() -> Result<(), SchemeError> {
        let program = "\
(define (cons x y) (lambda (m) (m x y)))\n\
(define (car z) (z (lambda (p q) p)))\n\
(define (cdr z) (z (lambda (p q) q)))\n\
(define (list-ref items n) (if (= n 0) (car items) (list-ref (cdr items) (- n 1))))\n\
(define (map proc items) \
(if (null? items) '() (cons (proc (car items)) (map proc (cdr items)))))\n\
(define (scale-list items factor) (map (lambda (x) (* x factor)) items))\n\
(define (add-lists list1 list2) \
(cond ((null? list1) list2) ((null? list2) list1) \
(else (cons (+ (car list1) (car list2)) (add-lists (cdr list1) (cdr list2))))))\n\
(define ones (cons 1 ones))\n\
(define integers (cons 1 (add-lists ones integers)))\n\
(list-ref integers 17)";
        // run_lazy answers one value per form; the last is the probe.
        assert_eq!(lazy_values(program)?.last().map(String::as_str), Some("18"));
        Ok(())
    }

    #[test]
    fn driver_transcript_shows_the_l_eval_prompts() {
        let text = lazy_driver_transcript(
            &Lazy,
            &["(define (try a b) (if (= a 0) 1 b))", "(try 0 (/ 1 0))"],
        );
        assert_eq!(
            text,
            ";;; L-Eval input: (define (try a b) (if (= a 0) 1 b))\n\
             ;;; L-Eval value: ok\n\
             ;;; L-Eval input: (try 0 (/ 1 0))\n\
             ;;; L-Eval value: 1\n"
        );
    }

    #[test]
    fn printable_driver_budgets_infinite_lazy_lists() {
        let text = printable_driver_transcript(
            &LazyPrintable,
            &[
                "(define ones (cons 1 ones))",
                "ones",
                "(car ones)",
                "(cons (cons 1 '()) (cons 2 '()))",
            ],
        );
        let values: Vec<&str> = text
            .lines()
            .filter_map(|line| line.strip_prefix(";;; L-Eval value: "))
            .collect();
        // The define, the budgeted prefix of `ones` (ten elements, then
        // the ellipsis), the demand on it, and the nested ordinary pair.
        assert_eq!(values.first(), Some(&"ok"));
        assert_eq!(values.get(1), Some(&"(1 1 1 1 1 1 1 1 1 1 ...)"));
        assert_eq!(values.get(2), Some(&"1"));
        assert_eq!(values.last(), Some(&"((1) 2)"));
    }

    #[test]
    fn printable_driver_stops_on_a_forcing_error() {
        let text = printable_driver_transcript(
            &LazyPrintable,
            &["(cons 1 (cons 2 '()))", "'(a b)", "\"hi\"", "(car '())"],
        );
        let values: Vec<&str> = text
            .lines()
            .filter_map(|line| line.strip_prefix(";;; L-Eval value: "))
            .collect();
        assert_eq!(values.first(), Some(&"(1 2)"), "text was: {text}");
        assert!(text.contains(";;; L-Eval value: (a b)\n"));
        assert!(text.contains(";;; L-Eval value: \"hi\"\n"));
        assert!(text.ends_with("Error: type mismatch: car of a non-pair: ()\n"));
    }

    #[test]
    fn buffer_sink_captures_display_output() {
        let text = lazy_driver_transcript(
            &Lazy,
            &[
                "(define (unless condition usual-value exceptional-value) \
                 (if condition exceptional-value usual-value))",
                "(unless (= 0 0) (/ 1 0) (begin (display \"exception: returning 0\") 0))",
            ],
        );
        assert!(
            // The object program's `display` writes no newline, so the
            // driver's value prompt follows the displayed text directly.
            text.contains("exception: returning 0;;; L-Eval value: 0\n"),
            "transcript was: {text}"
        );
    }
}
