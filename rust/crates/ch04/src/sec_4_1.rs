// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1: the metacircular evaluator, written in Rust against the
//! shared runtime. The evaluator runs the book's Scheme subset with
//! [`Value`] forms as the syntax: the reader
//! (`sicp_runtime::read_program`) produces the forms, dispatch is one
//! exhaustive `match` (the book's `eval` cond chain of 4.1.1), and
//! every failure travels through [`SchemeError`].
//!
//! The book's functions port under their Rust names: `eval` is
//! [`Evaluator::base_dispatch`] plus the free [`eval`]; `apply` is
//! [`Evaluator::apply_procedure`]; `list-of-values` is
//! [`Evaluator::list_of_values`]; `eval-if`, `eval-sequence`,
//! `eval-assignment`, and `eval-definition` are the matching trait
//! methods; `true?` is [`is_true`]; `cond->if` is [`cond_to_if`]; the
//! four environment operations of 4.1.3
//! (`lookup-variable-value`, `extend-environment`,
//! `set-variable-value!`, `define-variable!`) wrap the crate's
//! [`Env`] frames; `setup-environment` and the driver loop of 4.1.4 are
//! [`setup_environment`] and [`driver_transcript`]; the analyzed
//! evaluator of 4.1.7 is [`Analyzer`] with [`AnalyzerBase`].
//!
//! # The extension seam
//!
//! Exercises 4.2 to 4.13 and 4.16 to 4.23 add or replace syntax
//! clauses. The seam is one mechanism: the [`Evaluator`] (and
//! [`Analyzer`]) trait holds one required hook, `step`, and every other
//! clause as a provided method that recurses through `self.eval` and
//! `self.step`, so a new struct that checks its own forms first and
//! falls back to `self.base_step` catches the whole recursion at every
//! nesting depth without editing this file. Plain base behavior stays
//! reachable in one call: [`eval`] is the base evaluator, and nothing
//! here routes through an extension. A ten-line extension:
//!
//! ```rust
//! use std::rc::Rc;
//! use ch04::sec_4_1::{Base, Evaluator, Step, StepResult, is_tagged_list};
//! use sicp_runtime::{Env, SchemeError, Value};
//!
//! struct WithLet;
//!
//! impl Evaluator for WithLet {
//!     fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
//!         if is_tagged_list(exp, "let") {
//!             let rewritten = let_to_combination(exp);
//!             Ok(Step::Tail(rewritten, Rc::clone(env))) // fires at every depth
//!         } else {
//!             self.base_step(exp, env)
//!         }
//!     }
//! }
//! # fn let_to_combination(exp: &Value) -> Value { exp.clone() }
//! ```
//!
//! # Deep recursion
//!
//! The driver loop of [`Evaluator::eval`] unwinds the book's tail
//! positions -- the branches of an `if`, the last expression of a body,
//! the body of an applied procedure -- in constant host stack, so the
//! corpus's 100,000-call `count-down` runs like it does on a Scheme
//! host with tail calls. What still grows the Rust stack is non-tail
//! recursion, and [`run_program`] postpones that overflow by running
//! each program on a worker thread with a 256 MiB stack
//! ([`EVAL_STACK_BYTES`]). The bound is a postponement, not an answer:
//! the chapter 5 register machine, whose explicit stack is
//! tail-recursive by construction, is the real answer.

use std::cell::RefCell;
use std::fmt::Write as _;
use std::rc::Rc;

use sicp_runtime::{Closure, Env, Handler, SchemeError, Symbol, Value, cons_cell, print_value};

/// One evaluation's answer.
pub type EvalResult = Result<Value, SchemeError>;

/// One operand-list evaluation's answer.
pub type EvalList = Result<Vec<Value>, SchemeError>;

/// One analyzed expression: the execution procedure of 4.1.7, an
/// environment to result closure with the dispatch already decided.
pub type Exec = Rc<dyn Fn(&Rc<Env>) -> EvalResult>;

/// The worker stack that runs whole programs: 256 MiB, the bound the
/// deep-recursion corpus program relies on.
pub const EVAL_STACK_BYTES: usize = 256 * 1024 * 1024;

/// The marker value of a not-yet-assigned scanned-out definition
/// (4.1.6); the environment stores it as a real binding, never as a
/// default or an absent entry.
#[must_use]
pub fn unassigned() -> Value {
    Value::sym("*unassigned*")
}

/// Holds for every value except the false object, the book's `true?`.
#[must_use]
pub fn is_true(v: &Value) -> bool {
    !matches!(v, Value::Bool(false))
}

// ---------------------------------------------------------------------------
// Syntax: the predicates and selectors of 4.1.2 over `Value` forms.
// ---------------------------------------------------------------------------

/// Whether `exp` is a list tagged with `tag`, the book's `tagged-list?`.
#[must_use]
pub fn is_tagged_list(exp: &Value, tag: &str) -> bool {
    let Value::Pair(cell) = exp else {
        return false;
    };
    matches!(&*cell.car.borrow(), Value::Sym(s) if &**s == tag)
}

/// Whether `exp` evaluates to itself: numbers, booleans, and strings.
#[must_use]
pub fn is_self_evaluating(exp: &Value) -> bool {
    matches!(
        exp,
        Value::Int(_) | Value::Real(_) | Value::Bool(_) | Value::Str(_)
    )
}

/// Whether `exp` is a variable: a symbol.
#[must_use]
pub fn is_variable(exp: &Value) -> bool {
    matches!(exp, Value::Sym(_))
}

/// Whether `exp` is `(quote datum)`.
#[must_use]
pub fn is_quoted(exp: &Value) -> bool {
    is_tagged_list(exp, "quote")
}

/// Whether `exp` is `(set! name value)`.
#[must_use]
pub fn is_assignment(exp: &Value) -> bool {
    is_tagged_list(exp, "set!")
}

/// Whether `exp` is a `define` of a variable or a procedure.
#[must_use]
pub fn is_definition(exp: &Value) -> bool {
    is_tagged_list(exp, "define")
}

/// Whether `exp` is a `lambda` form.
#[must_use]
pub fn is_lambda(exp: &Value) -> bool {
    is_tagged_list(exp, "lambda")
}

/// Whether `exp` is an `if` form.
#[must_use]
pub fn is_if(exp: &Value) -> bool {
    is_tagged_list(exp, "if")
}

/// Whether `exp` is a `begin` form.
#[must_use]
pub fn is_begin(exp: &Value) -> bool {
    is_tagged_list(exp, "begin")
}

/// Whether `exp` is a `cond` form.
#[must_use]
pub fn is_cond(exp: &Value) -> bool {
    is_tagged_list(exp, "cond")
}

/// Whether `exp` is a `let` form, the grammar's derived form; the named
/// variant of exercise 4.8 is not part of the base language.
#[must_use]
pub fn is_let(exp: &Value) -> bool {
    is_tagged_list(exp, "let")
}

/// Rewrites one plain `let` into the application of a `lambda`, the
/// book's `let->combination` of exercise 4.6.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] on a malformed let.
pub fn let_to_combination(exp: &Value) -> EvalResult {
    let items = exp.list_items()?;
    let bindings = items
        .get(1)
        .cloned()
        .ok_or_else(|| SchemeError::TypeMismatch(format!("malformed let: {exp}")))?;
    let body: Vec<Value> = items.into_iter().skip(2).collect();
    let mut names: Vec<Symbol> = Vec::new();
    let mut inits = Vec::new();
    for binding in bindings.list_items()? {
        let pair = binding.list_items()?;
        let (Some(name), Some(init)) = (pair.first(), pair.get(1)) else {
            return Err(SchemeError::TypeMismatch(format!(
                "malformed let binding: {binding}"
            )));
        };
        let Value::Sym(name) = name else {
            return Err(SchemeError::TypeMismatch(format!("not a let name: {name}")));
        };
        names.push(name.clone());
        inits.push(init.clone());
    }
    let lambda = make_lambda(&names, None, &body);
    Ok(Value::Pair(cons_cell(lambda, Value::list(inits))))
}

/// Whether `exp` is a procedure application: any pair left over after
/// the special forms have been checked.
#[must_use]
pub fn is_application(exp: &Value) -> bool {
    exp.is_pair()
}

/// The items of a proper list, the book's list selectors over syntax.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `exp` is a dotted or non-list form.
pub fn list_items(exp: &Value) -> EvalList {
    exp.list_items()
}

/// The first item of a list form.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `exp` is not a pair.
pub fn first_of(exp: &Value) -> EvalResult {
    sicp_runtime::car(exp)
}

/// The rest items of a list form.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `exp` is not a pair.
pub fn rest_of(exp: &Value) -> EvalResult {
    sicp_runtime::cdr(exp)
}

/// The items of a combination's operand list, the book's `operands`.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `exp` is not a proper application.
pub fn operand_items(exp: &Value) -> EvalList {
    let cdr = sicp_runtime::cdr(exp)?;
    cdr.list_items()
}

/// The text of a quotation.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the quote has no datum.
pub fn text_of_quotation(exp: &Value) -> EvalResult {
    second_of(exp)
}

/// The name an assignment assigns.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the form is malformed.
pub fn assignment_variable(exp: &Value) -> EvalResult {
    second_of(exp)
}

/// The expression an assignment evaluates.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the form is malformed.
pub fn assignment_value(exp: &Value) -> EvalResult {
    third_of(exp)
}

/// The name a definition binds: the variable form's target, or the
/// procedure form's name.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the form is malformed.
pub fn definition_variable(exp: &Value) -> EvalResult {
    let second = second_of(exp)?;
    if is_variable(&second) {
        Ok(second)
    } else {
        first_of(&second)
    }
}

/// The value a definition binds: the variable form's expression, or the
/// procedure form rebuilt as a `lambda`, the book's `definition-value`.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the form is malformed.
pub fn definition_value(exp: &Value) -> EvalResult {
    let second = second_of(exp)?;
    if is_variable(&second) {
        return third_of(exp);
    }
    let (params, rest) = split_params(&rest_of(&second)?)?;
    let body = {
        let items = exp.list_items()?;
        items.into_iter().skip(2).collect::<Vec<_>>()
    };
    Ok(make_lambda(&params, rest.as_ref(), &body))
}

/// The parameter names of a `lambda` form.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the form is malformed.
pub fn lambda_parameters(exp: &Value) -> Result<(Vec<Symbol>, Option<Symbol>), SchemeError> {
    split_params(&second_of(exp)?)
}

/// The body forms of a `lambda` form.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the form is malformed.
pub fn lambda_body(exp: &Value) -> EvalList {
    let items = exp.list_items()?;
    Ok(items.into_iter().skip(2).collect())
}

/// Builds a `lambda` form out of parameters and a body, the book's
/// `make-lambda`.
#[must_use]
pub fn make_lambda(params: &[Symbol], rest: Option<&Symbol>, body: &[Value]) -> Value {
    let mut param_list = Value::Nil;
    for param in params.iter().rev() {
        param_list = Value::Pair(cons_cell(Value::Sym(param.clone()), param_list));
    }
    if let Some(rest) = rest {
        param_list = Value::Pair(cons_cell(Value::sym("."), param_list));
        param_list = Value::Pair(cons_cell(Value::Sym(rest.clone()), param_list));
    }
    let mut form = Value::list(body.to_vec());
    form = Value::Pair(cons_cell(param_list, form));
    Value::Pair(cons_cell(Value::sym("lambda"), form))
}

/// Splits a parameter list `(a b c)` or `(a b . rest)` into its names
/// and the optional rest name.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the list is malformed.
pub fn split_params(form: &Value) -> Result<(Vec<Symbol>, Option<Symbol>), SchemeError> {
    let mut params = Vec::new();
    let mut cursor = form.clone();
    loop {
        match cursor {
            Value::Nil => return Ok((params, None)),
            Value::Sym(name) => return Ok((params, Some(name))),
            Value::Pair(cell) => {
                let car = cell.car.borrow().clone();
                let Value::Sym(name) = car else {
                    return Err(SchemeError::TypeMismatch(format!(
                        "not a parameter name: {car}"
                    )));
                };
                params.push(name);
                cursor = cell.cdr.borrow().clone();
            }
            other => {
                return Err(SchemeError::TypeMismatch(format!(
                    "not a parameter list: {other}"
                )));
            }
        }
    }
}

fn second_of(exp: &Value) -> EvalResult {
    let items = exp.list_items()?;
    items
        .into_iter()
        .nth(1)
        .ok_or_else(|| SchemeError::TypeMismatch(format!("missing second part: {exp}")))
}

fn third_of(exp: &Value) -> EvalResult {
    let items = exp.list_items()?;
    items
        .into_iter()
        .nth(2)
        .ok_or_else(|| SchemeError::TypeMismatch(format!("missing third part: {exp}")))
}

/// The predicate, consequent, and optional alternative of an `if`, the
/// missing alternative filled with the false object.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the form is malformed.
pub fn if_parts(exp: &Value) -> Result<(Value, Value, Value), SchemeError> {
    let items = exp.list_items()?;
    let predicate = items.get(1).cloned().unwrap_or(Value::Nil);
    let consequent = items.get(2).cloned().unwrap_or(Value::Nil);
    let alternative = items.get(3).cloned().unwrap_or(Value::Bool(false));
    Ok((predicate, consequent, alternative))
}

/// The clause list of a `cond`.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `exp` is not a cond.
pub fn cond_clauses(exp: &Value) -> EvalList {
    operand_items(exp)
}

/// Whether a clause is the `else` clause.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when `clause` is not a list.
pub fn is_else_clause(clause: &Value) -> Result<bool, SchemeError> {
    let predicate = first_of(clause)?;
    Ok(matches!(&predicate, Value::Sym(s) if &**s == "else"))
}

/// Packs a body into one expression, a `begin` when more than one
/// expression remains, the book's `sequence->exp`.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] when the body is empty.
pub fn sequence_to_exp(seq: &[Value]) -> EvalResult {
    match seq {
        [] => Err(SchemeError::TypeMismatch(
            "empty sequence: SEQUENCE->EXP".to_owned(),
        )),
        [one] => Ok(one.clone()),
        many => Ok(make_begin(many)),
    }
}

/// Builds a `begin` form, the book's `make-begin`.
#[must_use]
pub fn make_begin(seq: &[Value]) -> Value {
    Value::Pair(cons_cell(Value::sym("begin"), Value::list(seq.to_vec())))
}

/// Rewrites one `cond` into a nest of `if` expressions, the book's
/// `cond->if`; a cond with no true predicate and no `else` answers the
/// false object, the choice the section fixes for the omitted case.
///
/// # Errors
/// [`SchemeError::TypeMismatch`] on a malformed or `else`-not-last cond.
pub fn cond_to_if(exp: &Value) -> EvalResult {
    expand_clauses(&cond_clauses(exp)?)
}

fn expand_clauses(clauses: &[Value]) -> EvalResult {
    let Some((first, rest)) = clauses.split_first() else {
        return Ok(Value::Bool(false));
    };
    if is_else_clause(first)? {
        if rest.is_empty() {
            let actions = clause_actions(first)?;
            return sequence_to_exp(&actions);
        }
        return Err(SchemeError::TypeMismatch(
            "ELSE clause isn't last: COND->IF".to_owned(),
        ));
    }
    let actions = clause_actions(first)?;
    let consequent = sequence_to_exp(&actions)?;
    let alternative = expand_clauses(rest)?;
    let predicate = first_of(first)?;
    Ok(make_if(&predicate, &consequent, &alternative))
}

fn clause_actions(clause: &Value) -> EvalList {
    let items = clause.list_items()?;
    Ok(items.into_iter().skip(1).collect())
}

/// Builds an `if` form, the book's `make-if`.
#[must_use]
pub fn make_if(predicate: &Value, consequent: &Value, alternative: &Value) -> Value {
    Value::list(vec![
        Value::sym("if"),
        predicate.clone(),
        consequent.clone(),
        alternative.clone(),
    ])
}

// ---------------------------------------------------------------------------
// 4.1.3: the four environment operations over the crate's frames.
// ---------------------------------------------------------------------------

/// The book's `lookup-variable-value`: the nearest binding of `name`.
///
/// # Errors
/// [`SchemeError::UnboundVariable`] when no frame binds `name`.
pub fn lookup_variable_value(name: &str, env: &Rc<Env>) -> EvalResult {
    env.lookup(name)
}

/// The book's `extend-environment`: one new frame binding each name to
/// the value at the same position, in front of `base`.
///
/// # Errors
/// [`SchemeError::WrongArity`] when the parameter count and the
/// argument count disagree, the book's "Too many arguments supplied".
pub fn extend_environment(
    procedure: &str,
    params: &[Symbol],
    rest: Option<&Symbol>,
    args: &[Value],
    base: &Rc<Env>,
) -> Result<Rc<Env>, SchemeError> {
    let expected = match rest {
        Some(_) => format!("at least {}", params.len()),
        None => params.len().to_string(),
    };
    if args.len() < params.len() || (rest.is_none() && args.len() > params.len()) {
        return Err(SchemeError::WrongArity {
            procedure: procedure.to_owned(),
            expected,
            got: args.len(),
        });
    }
    let frame = Env::child(base);
    for (param, arg) in params.iter().zip(args) {
        frame.define(param.clone(), arg.clone());
    }
    if let Some(rest) = rest {
        frame.define(rest.clone(), Value::list(args[params.len()..].to_vec()));
    }
    Ok(frame)
}

/// The book's `set-variable-value!`: rebinds the nearest existing
/// binding of `name`.
///
/// # Errors
/// [`SchemeError::UnboundVariable`] when no frame binds `name`.
pub fn set_variable_value_(name: &str, value: Value, env: &Rc<Env>) -> Result<(), SchemeError> {
    env.set(name, value)
}

/// The book's `define-variable!`: binds `name` in the newest frame and
/// answers the symbol `ok`.
///
/// # Errors
/// Never fails; the answer channel matches the other operations.
pub fn define_variable_(name: &str, value: Value, env: &Rc<Env>) -> EvalResult {
    env.define(Rc::from(name), value);
    Ok(Value::sym("ok"))
}

// ---------------------------------------------------------------------------
// 4.1.1: the evaluator seam and the base evaluator.
// ---------------------------------------------------------------------------

/// One dispatch step's outcome: a complete value, or a tail call the
/// driver loop unwinds in constant host stack.
pub enum Step {
    /// The evaluation is complete.
    Done(Value),
    /// Continue with another expression in another environment: the
    /// book's tail positions, the branches of `if`, the last expression
    /// of a body, and the body of an applied procedure.
    Tail(Value, Rc<Env>),
}

/// One step's answer.
pub type StepResult = Result<Step, SchemeError>;

/// The evaluator's extension seam: one required hook, [`Evaluator::step`],
/// plus the book's clause procedures of 4.1.1 as provided methods that
/// recurse through `self.eval` and `self.step`, so a new struct that
/// checks its own forms first and falls back to `self.base_step` catches
/// the whole recursion at every nesting depth without editing this
/// file. See the module docs for the ten-line example.
pub trait Evaluator {
    /// Dispatches one expression one step; the hook every recursion
    /// crosses, so an extension's clause fires at every nesting depth.
    ///
    /// # Errors
    /// Whatever the expression's step raises.
    fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult;

    /// Drives steps to a value: a `Tail` outcome loops back through
    /// [`Evaluator::step`], which keeps the book's tail positions --
    /// the branches of an `if`, the last expression of a body, the body
    /// of an applied procedure -- in constant host stack, the way a
    /// Scheme host's tail calls behave. Non-tail recursion still grows
    /// the stack, which [`run_program`]'s worker bound postpones.
    ///
    /// # Errors
    /// Whatever any step raises.
    fn eval(&self, exp: &Value, env: &Rc<Env>) -> EvalResult {
        let mut cursor = exp.clone();
        let mut place = Rc::clone(env);
        loop {
            match self.step(&cursor, &place)? {
                Step::Done(value) => return Ok(value),
                Step::Tail(next, next_env) => {
                    cursor = next;
                    place = next_env;
                }
            }
        }
    }

    /// The book's `eval` cond chain of 4.1.1: one arm per syntactic
    /// type, `cond` reduced through [`cond_to_if`], and the leftover
    /// pairs treated as applications. Extensions fall back to this.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on an unknown expression type;
    /// otherwise whatever the step raises.
    fn base_step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
        if is_self_evaluating(exp) {
            return Ok(Step::Done(exp.clone()));
        }
        if is_variable(exp) {
            let Value::Sym(name) = exp else {
                return Err(SchemeError::TypeMismatch("not a variable".to_owned()));
            };
            return Ok(Step::Done(lookup_variable_value(name, env)?));
        }
        if is_quoted(exp) {
            return Ok(Step::Done(text_of_quotation(exp)?));
        }
        if is_assignment(exp) {
            return Ok(Step::Done(self.eval_assignment(exp, env)?));
        }
        if is_definition(exp) {
            return Ok(Step::Done(self.eval_definition(exp, env)?));
        }
        if is_if(exp) {
            let (predicate, consequent, alternative) = if_parts(exp)?;
            let tested = self.eval(&predicate, env)?;
            return Ok(Step::Tail(
                if is_true(&tested) {
                    consequent
                } else {
                    alternative
                },
                Rc::clone(env),
            ));
        }
        if is_lambda(exp) {
            let (params, rest) = lambda_parameters(exp)?;
            let body = lambda_body(exp)?;
            return Ok(Step::Done(Value::Closure(Rc::new(Closure {
                name: None,
                params,
                rest,
                body,
                env: Rc::clone(env),
            }))));
        }
        if is_begin(exp) {
            return self.step_sequence(&operand_items(exp)?, env);
        }
        if is_cond(exp) {
            return Ok(Step::Tail(cond_to_if(exp)?, Rc::clone(env)));
        }
        if is_let(exp) {
            return Ok(Step::Tail(let_to_combination(exp)?, Rc::clone(env)));
        }
        if is_application(exp) {
            let operator = first_of(exp)?;
            let operands = operand_items(exp)?;
            let proc = self.eval(&operator, env)?;
            let args = self.list_of_values(&operands, env)?;
            return self.tail_apply(&proc, &args);
        }
        Err(SchemeError::TypeMismatch(format!(
            "Unknown expression type: EVAL {exp}"
        )))
    }

    /// Evaluates all but the last expression for effect and hands the
    /// last back as a tail: the body of a `begin` or of an applied
    /// procedure.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on an empty sequence; otherwise
    /// whatever the evaluation raises.
    fn step_sequence(&self, exps: &[Value], env: &Rc<Env>) -> StepResult {
        let Some((last, head)) = exps.split_last() else {
            return Err(SchemeError::TypeMismatch(
                "empty sequence: EVAL-SEQUENCE".to_owned(),
            ));
        };
        for exp in head {
            self.eval(exp, env)?;
        }
        Ok(Step::Tail(last.clone(), Rc::clone(env)))
    }

    /// The book's `list-of-values`: evaluates the operands from left to
    /// right by construction.
    ///
    /// # Errors
    /// Whatever the first failing operand raises.
    fn list_of_values(&self, exps: &[Value], env: &Rc<Env>) -> EvalList {
        let mut values = Vec::with_capacity(exps.len());
        for exp in exps {
            values.push(self.eval(exp, env)?);
        }
        Ok(values)
    }

    /// The book's `eval-sequence`: evaluates a body or `begin` in order
    /// and answers the last value; the last evaluation runs through the
    /// driver, so a body ending in a tail call stays flat.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on an empty sequence; otherwise
    /// whatever the evaluation raises.
    fn eval_sequence(&self, exps: &[Value], env: &Rc<Env>) -> EvalResult {
        let Some((last, head)) = exps.split_last() else {
            return Err(SchemeError::TypeMismatch(
                "empty sequence: EVAL-SEQUENCE".to_owned(),
            ));
        };
        for exp in head {
            self.eval(exp, env)?;
        }
        self.eval(last, env)
    }

    /// The book's `eval-if`: the predicate is evaluated in the object
    /// language and translated with [`is_true`] before the branch; the
    /// chosen branch runs through the driver, so it is a tail position.
    ///
    /// # Errors
    /// Whatever the chosen branch raises.
    fn eval_if(&self, exp: &Value, env: &Rc<Env>) -> EvalResult {
        let (predicate, consequent, alternative) = if_parts(exp)?;
        let tested = self.eval(&predicate, env)?;
        self.eval(
            if is_true(&tested) {
                &consequent
            } else {
                &alternative
            },
            env,
        )
    }

    /// The book's `eval-assignment`: evaluates the value, rebinds the
    /// variable, and answers the symbol `ok`.
    ///
    /// # Errors
    /// [`SchemeError::UnboundVariable`] on an unbound target; otherwise
    /// whatever the value expression raises.
    fn eval_assignment(&self, exp: &Value, env: &Rc<Env>) -> EvalResult {
        let name = assignment_variable(exp)?;
        let value = self.eval(&assignment_value(exp)?, env)?;
        let Value::Sym(name) = name else {
            return Err(SchemeError::TypeMismatch(format!(
                "not an assignment target: {name}"
            )));
        };
        set_variable_value_(&name, value, env)?;
        Ok(Value::sym("ok"))
    }

    /// The book's `eval-definition`: binds the variable or the named
    /// procedure in the newest frame and answers the symbol `ok`. The
    /// sequential treatment of 4.1.6 applies: each define extends the
    /// frame one name at a time, which makes internal procedures
    /// mutually recursive as long as the definitions come first.
    ///
    /// # Errors
    /// Whatever the value expression raises.
    fn eval_definition(&self, exp: &Value, env: &Rc<Env>) -> EvalResult {
        let name = definition_variable(exp)?;
        let Value::Sym(name) = name else {
            return Err(SchemeError::TypeMismatch(format!(
                "not a definition target: {name}"
            )));
        };
        let second = second_of(exp)?;
        let value = if is_variable(&second) {
            self.eval(&third_of(exp)?, env)?
        } else {
            let (params, rest) = split_params(&rest_of(&second)?)?;
            let body = {
                let items = exp.list_items()?;
                items.into_iter().skip(2).collect::<Vec<_>>()
            };
            self.make_named_procedure(&name, &params, rest.as_ref(), &body, env)?
        };
        define_variable_(&name, value, env)
    }

    /// Builds the procedure value of a named definition; the base makes
    /// a [`Closure`] carrying the name, which the printer prints as
    /// `#[compound-procedure name]`.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on a malformed parameter list.
    fn make_named_procedure(
        &self,
        name: &str,
        params: &[Symbol],
        rest: Option<&Symbol>,
        body: &[Value],
        env: &Rc<Env>,
    ) -> EvalResult {
        Ok(Value::Closure(Rc::new(Closure {
            name: Some(Rc::from(name)),
            params: params.to_vec(),
            rest: rest.cloned(),
            body: body.to_vec(),
            env: Rc::clone(env),
        })))
    }

    /// The application's tail half: the procedure and its arguments are
    /// in hand, so a primitive answers directly and a compound
    /// procedure evaluates all but the last expression of its body and
    /// hands the last back as a tail. The object language's `apply` and
    /// `map` must place procedure *values* into the operator position,
    /// which a bare handler cannot do, so they are applied here where
    /// the evaluator is at hand.
    ///
    /// # Errors
    /// [`SchemeError::NotProcedure`] when `proc` is not a procedure;
    /// otherwise whatever the application raises.
    fn tail_apply(&self, proc: &Value, args: &[Value]) -> StepResult {
        match proc {
            Value::Primitive { name, .. } if &**name == "apply" => {
                let [target, list, ..] = args else {
                    return Err(arity("apply", 2, args.len()));
                };
                let items = list.list_items()?;
                self.apply_procedure(target, &items).map(Step::Done)
            }
            Value::Primitive { name, .. } if &**name == "map" => {
                self.apply_procedure(proc, args).map(Step::Done)
            }
            Value::Primitive { f, .. } => f(args).map(Step::Done),
            Value::Closure(c) => {
                let frame =
                    extend_environment(closure_name(c), &c.params, c.rest.as_ref(), args, &c.env)?;
                self.step_sequence(&c.body, &frame)
            }
            other => Err(SchemeError::NotProcedure(other.clone())),
        }
    }

    /// The book's `apply`, forced to a value: a primitive runs its
    /// handler; a compound procedure extends its captured environment
    /// with one frame binding the parameters to the arguments and
    /// evaluates its body there through the driver. The forced form
    /// serves the `apply` and `map` primitives and extension code that
    /// applies a procedure mid-step.
    ///
    /// # Errors
    /// [`SchemeError::NotProcedure`] when `proc` is not a procedure;
    /// otherwise whatever the application raises.
    fn apply_procedure(&self, proc: &Value, args: &[Value]) -> EvalResult {
        match proc {
            Value::Primitive { name, .. } if &**name == "apply" => {
                let [target, list, ..] = args else {
                    return Err(arity("apply", 2, args.len()));
                };
                let items = list.list_items()?;
                self.apply_procedure(target, &items)
            }
            Value::Primitive { name, .. } if &**name == "map" => {
                let [f, lists @ ..] = args else {
                    return Err(arity("map", 2, args.len()));
                };
                self.map_over(f, lists)
            }
            Value::Primitive { f, .. } => f(args),
            Value::Closure(c) => {
                let frame =
                    extend_environment(closure_name(c), &c.params, c.rest.as_ref(), args, &c.env)?;
                self.eval_sequence(&c.body, &frame)
            }
            other => Err(SchemeError::NotProcedure(other.clone())),
        }
    }

    /// Applies `f` across one or more lists, stopping at the shortest,
    /// the book's n-ary `map`.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] when an argument is not a proper
    /// list; otherwise whatever `f` raises.
    fn map_over(&self, f: &Value, lists: &[Value]) -> EvalResult {
        let columns: Vec<Vec<Value>> = lists
            .iter()
            .map(sicp_runtime::Value::list_items)
            .collect::<Result<_, _>>()?;
        let depth = columns.iter().map(Vec::len).min().unwrap_or(0);
        let mut out = Vec::with_capacity(depth);
        for index in 0..depth {
            let row: Vec<Value> = columns.iter().map(|col| col[index].clone()).collect();
            out.push(self.apply_procedure(f, &row)?);
        }
        Ok(Value::list(out))
    }
}

fn closure_name(c: &Closure) -> &str {
    c.name.as_deref().unwrap_or("#[compound-procedure]")
}

/// The base evaluator: the section as the book presents it, with no
/// extension in the way.
#[derive(Debug, Default)]
pub struct Base;

impl Evaluator for Base {
    fn step(&self, exp: &Value, env: &Rc<Env>) -> StepResult {
        self.base_step(exp, env)
    }
}

/// Evaluates one expression with the base evaluator: plain base
/// behavior in one call, the book's `(eval exp env)`.
///
/// # Errors
/// Whatever the expression's evaluation raises.
pub fn eval(exp: &Value, env: &Rc<Env>) -> EvalResult {
    Base.eval(exp, env)
}

// ---------------------------------------------------------------------------
// 4.1.4: the primitive table, the global environment, and the drivers.
// ---------------------------------------------------------------------------

/// Where the object language's `display` and `newline` write.
#[derive(Clone, Debug)]
pub enum OutputSink {
    /// The host's standard output.
    Stdout,
    /// A captured buffer, for tests and transcripts.
    Buffer(Rc<RefCell<String>>),
}

impl OutputSink {
    /// A buffer sink paired with the cell it fills.
    #[must_use]
    pub fn buffer() -> (Self, Rc<RefCell<String>>) {
        let cell = Rc::new(RefCell::new(String::new()));
        (Self::Buffer(Rc::clone(&cell)), cell)
    }

    /// Writes one piece of text.
    pub fn write_str(&self, text: &str) {
        match self {
            Self::Stdout => print!("{text}"),
            Self::Buffer(cell) => cell.borrow_mut().push_str(text),
        }
    }
}

/// The book's sample primitives of 4.1.4 plus the additions
/// `metacircular.scm` reads its own setup through: the destructive pair
/// operations, the exactness conversion, `length`, `map`, and the
/// `c(a|d)+r` compositions. `apply` and `map` carry sentinel handlers:
/// [`Evaluator::apply_procedure`] intercepts them because they must
/// apply procedure values, which a bare handler cannot.
#[must_use]
pub fn primitive_table(sink: &OutputSink) -> Vec<(&'static str, Handler)> {
    let mut table = core_table();
    table.extend(pair_table());
    table.extend(arith_table());
    table.extend(output_table(sink));
    table
}

/// One primitive handler under `name`.
fn primitive(
    name: &'static str,
    f: impl Fn(&[Value]) -> EvalResult + 'static,
) -> (&'static str, Handler) {
    (name, Rc::new(f))
}

/// One argument, or the arity error naming `name`.
fn one_arg<'a>(name: &str, args: &'a [Value]) -> Result<&'a Value, SchemeError> {
    match args {
        [v] => Ok(v),
        _ => Err(arity(name, 1, args.len())),
    }
}

/// Two arguments, or the arity error naming `name`.
fn two_args<'a>(name: &str, args: &'a [Value]) -> Result<(&'a Value, &'a Value), SchemeError> {
    match args {
        [a, b] => Ok((a, b)),
        _ => Err(arity(name, 2, args.len())),
    }
}

/// The integer pair of a two-argument integer operation.
fn int_args(name: &str, args: &[Value]) -> Result<(i128, i128), SchemeError> {
    let (a, b) = two_args(name, args)?;
    Ok((int_of(name, a)?, int_of(name, b)?))
}

/// The data, list, and predicate primitives of grammar.md's core set,
/// plus `error`, `length`, and the `apply`/`map` sentinels whose real
/// application [`Evaluator::apply_procedure`] intercepts.
fn core_table() -> Vec<(&'static str, Handler)> {
    vec![
        primitive("car", |args| sicp_runtime::car(one_arg("car", args)?)),
        primitive("cdr", |args| sicp_runtime::cdr(one_arg("cdr", args)?)),
        primitive("cons", |args| {
            let (a, b) = two_args("cons", args)?;
            Ok(Value::Pair(cons_cell(a.clone(), b.clone())))
        }),
        primitive("list", |args| Ok(Value::list(args.to_vec()))),
        primitive("null?", |args| {
            Ok(Value::boolean(one_arg("null?", args)?.is_nil()))
        }),
        primitive("pair?", |args| {
            Ok(Value::boolean(one_arg("pair?", args)?.is_pair()))
        }),
        primitive("eq?", |args| {
            let (a, b) = two_args("eq?", args)?;
            Ok(Value::boolean(eq_values(a, b)))
        }),
        primitive("equal?", |args| {
            let (a, b) = two_args("equal?", args)?;
            Ok(Value::boolean(a == b))
        }),
        primitive("number?", |args| {
            let v = one_arg("number?", args)?;
            Ok(Value::boolean(matches!(v, Value::Int(_) | Value::Real(_))))
        }),
        primitive("symbol?", |args| {
            Ok(Value::boolean(matches!(
                one_arg("symbol?", args)?,
                Value::Sym(_)
            )))
        }),
        primitive("string?", |args| {
            Ok(Value::boolean(matches!(
                one_arg("string?", args)?,
                Value::Str(_)
            )))
        }),
        primitive("not", |args| {
            Ok(Value::boolean(!is_true(one_arg("not", args)?)))
        }),
        primitive("assoc", |args| match args {
            [key, entries] => {
                for entry in entries.list_items()? {
                    let items = entry.list_items()?;
                    let Some(first) = items.first() else {
                        continue;
                    };
                    if first == key {
                        return Ok(entry);
                    }
                }
                Ok(Value::boolean(false))
            }
            _ => Err(arity("assoc", 2, args.len())),
        }),
        primitive("length", |args| {
            // usize to i128 is lossless: no program list approaches the
            // narrower width.
            Ok(Value::Int(
                one_arg("length", args)?.list_items()?.len() as i128
            ))
        }),
        primitive("error", |args| match args.split_first() {
            None => Err(SchemeError::UserRaised {
                message: "error".to_owned(),
                irritants: Vec::new(),
            }),
            Some((first, irritants)) => Err(SchemeError::UserRaised {
                message: sicp_runtime::display_value(first),
                irritants: irritants.to_vec(),
            }),
        }),
        primitive("apply", |_| {
            Err(SchemeError::TypeMismatch(
                "apply is applied by the evaluator's apply_procedure".to_owned(),
            ))
        }),
        primitive("map", |_| {
            Err(SchemeError::TypeMismatch(
                "map is applied by the evaluator's apply_procedure".to_owned(),
            ))
        }),
    ]
}

/// The destructive pair operations `metacircular.scm` builds its frames
/// out of, the exactness conversion its `/` reads through, and the
/// `c(a|d)+r` compositions its selectors use.
fn pair_table() -> Vec<(&'static str, Handler)> {
    let mut table = vec![
        primitive("set-car!", |args| {
            let (p, v) = two_args("set-car!", args)?;
            sicp_runtime::set_car(&as_pair("set-car!", p)?, v.clone());
            Ok(Value::sym("ok"))
        }),
        primitive("set-cdr!", |args| {
            let (p, v) = two_args("set-cdr!", args)?;
            sicp_runtime::set_cdr(&as_pair("set-cdr!", p)?, v.clone());
            Ok(Value::sym("ok"))
        }),
        primitive("exact->inexact", |args| {
            // The book's exactness conversion is intentionally inexact:
            // an integer past f64's 2^53 mantissa loses precision by
            // the rule the corpus's programs rely on.
            let v = one_arg("exact->inexact", args)?;
            Ok(Value::real(number_of("exact->inexact", v)?))
        }),
    ];
    for name in CXR_NAMES {
        table.push((name, cxr_handler(name)));
    }
    table
}

/// Every `c(a|d)+r` composition the corpus programs read through.
const CXR_NAMES: &[&str] = &[
    "caar", "cadr", "cdar", "cddr", "caaar", "caadr", "cadar", "caddr", "cdaar", "cdadr", "cddar",
    "cdddr", "cadddr",
];

/// Builds one `c(a|d)+r` primitive: the letters of its own name, read
/// right to left, pick the outer operation first.
fn cxr_handler(name: &'static str) -> Handler {
    Rc::new(move |args: &[Value]| {
        let [v, ..] = args else {
            return Err(arity(name, 1, args.len()));
        };
        let mut cursor = v.clone();
        for letter in name[1..name.len() - 1].chars().rev() {
            cursor = match letter {
                'a' => sicp_runtime::car(&cursor)?,
                'd' => sicp_runtime::cdr(&cursor)?,
                other => {
                    return Err(SchemeError::TypeMismatch(format!(
                        "bad composition letter {other} in {name}"
                    )));
                }
            };
        }
        Ok(cursor)
    })
}

/// The arithmetic and comparison primitives: exact integers while every
/// operand is exact, floats the moment one is inexact, and `/` always
/// inexact per grammar.md.
fn arith_table() -> Vec<(&'static str, Handler)> {
    vec![
        primitive("+", |args| arith_fold("+", args, Arith::Add)),
        primitive("-", |args| arith_fold("-", args, Arith::Sub)),
        primitive("*", |args| arith_fold("*", args, Arith::Mul)),
        primitive("/", divide),
        primitive("=", |args| compare("=", args, numeric_eq)),
        primitive("<", |args| compare("<", args, |a, b| a < b)),
        primitive(">", |args| compare(">", args, |a, b| a > b)),
        primitive("<=", |args| compare("<=", args, |a, b| a <= b)),
        primitive(">=", |args| compare(">=", args, |a, b| a >= b)),
        primitive("remainder", |args| {
            let (a, b) = int_args("remainder", args)?;
            let r = a.checked_rem(b).ok_or(remainder_error(b))?;
            Ok(Value::Int(r))
        }),
        primitive("quotient", |args| {
            let (a, b) = int_args("quotient", args)?;
            let q = a.checked_div(b).ok_or(remainder_error(b))?;
            Ok(Value::Int(q))
        }),
        primitive("abs", |args| {
            let v = one_arg("abs", args)?;
            match v {
                Value::Int(n) => Ok(Value::Int(n.checked_abs().ok_or(SchemeError::Overflow)?)),
                Value::Real(x) => Ok(Value::real(x.abs())),
                other => Err(SchemeError::TypeMismatch(format!(
                    "abs: not a number: {other}"
                ))),
            }
        }),
    ]
}

/// The output primitives; `display` and `newline` write through `sink`.
fn output_table(sink: &OutputSink) -> Vec<(&'static str, Handler)> {
    let display_sink = sink.clone();
    let newline_sink = sink.clone();
    vec![
        primitive("display", move |args| {
            let v = one_arg("display", args)?;
            display_sink.write_str(&sicp_runtime::display_value(v));
            Ok(Value::sym("ok"))
        }),
        primitive("newline", move |args| {
            match args {
                [] => {}
                _ => return Err(arity("newline", 0, args.len())),
            }
            newline_sink.write_str("\n");
            Ok(Value::sym("ok"))
        }),
    ]
}

/// The exact integer of a value, or a type error naming `name`.
fn int_of(name: &str, v: &Value) -> Result<i128, SchemeError> {
    match v {
        Value::Int(n) => Ok(*n),
        other => Err(SchemeError::TypeMismatch(format!(
            "{name}: not an integer: {other}"
        ))),
    }
}

/// The numeric content of a value, integers and floats mixed; an exact
/// integer outside f64's mantissa loses precision by the same
/// exactness rule the book's inexact arithmetic applies.
#[expect(
    clippy::cast_precision_loss,
    reason = "the book's exactness conversion is the stated semantics of mixed arithmetic"
)]
fn number_of(name: &str, v: &Value) -> Result<f64, SchemeError> {
    match v {
        Value::Int(n) => Ok(*n as f64),
        Value::Real(x) => Ok(*x),
        other => Err(SchemeError::TypeMismatch(format!(
            "{name}: not a number: {other}"
        ))),
    }
}

/// The three foldable arithmetic operations.
#[derive(Clone, Copy)]
enum Arith {
    /// Addition, seeded with 0.
    Add,
    /// Subtraction, seeded with the first operand.
    Sub,
    /// Multiplication, seeded with 1.
    Mul,
}

impl Arith {
    /// Steps one pair.
    fn exact(self, acc: i128, n: i128) -> Option<i128> {
        match self {
            Self::Add => acc.checked_add(n),
            Self::Sub => acc.checked_sub(n),
            Self::Mul => acc.checked_mul(n),
        }
    }

    /// Steps one pair in float arithmetic.
    fn real(self, acc: f64, x: f64) -> f64 {
        match self {
            Self::Add => acc + x,
            Self::Sub => acc - x,
            Self::Mul => acc * x,
        }
    }

    /// The seed the fold starts from.
    fn unit(self) -> i128 {
        match self {
            Self::Add | Self::Sub => 0,
            Self::Mul => 1,
        }
    }
}

/// Folds one arithmetic operation left over the arguments: exact while
/// every operand is exact, float the moment one is inexact. Addition
/// and multiplication seed with their unit; subtraction folds from the
/// first operand, so `(- x)` negates and `(- a b c)` chains the
/// difference.
fn arith_fold(name: &str, args: &[Value], op: Arith) -> EvalResult {
    if args.is_empty() {
        return Err(arity(name, 1, 0));
    }
    if let (Arith::Sub, [only]) = (op, args) {
        return negate(name, only);
    }
    let any_real = args.iter().any(|v| matches!(v, Value::Real(_)));
    if any_real {
        let mut floats = Vec::with_capacity(args.len());
        if matches!(op, Arith::Add) {
            floats.push(0.0);
        }
        if matches!(op, Arith::Mul) {
            floats.push(1.0);
        }
        for v in args {
            floats.push(number_of(name, v)?);
        }
        let [head, rest @ ..] = &floats[..] else {
            return Err(arity(name, 1, 0));
        };
        return Ok(Value::real(
            rest.iter().fold(*head, |acc, x| op.real(acc, *x)),
        ));
    }
    let mut exacts = Vec::with_capacity(args.len());
    if matches!(op, Arith::Add | Arith::Mul) {
        exacts.push(op.unit());
    }
    for v in args {
        exacts.push(int_of(name, v)?);
    }
    let [head, rest @ ..] = &exacts[..] else {
        return Err(arity(name, 1, 0));
    };
    let mut acc = *head;
    for n in rest {
        acc = op.exact(acc, *n).ok_or(SchemeError::Overflow)?;
    }
    Ok(Value::Int(acc))
}

/// `(- x)`: the negation of one operand, checked at the integer width.
fn negate(name: &str, v: &Value) -> EvalResult {
    match v {
        Value::Real(x) => Ok(Value::real(-x)),
        other => {
            let n = int_of(name, other)?;
            Ok(Value::Int(n.checked_neg().ok_or(SchemeError::Overflow)?))
        }
    }
}

/// The book's inexact division: every intermediate is a float, and a
/// zero divisor raises the division error.
fn divide(args: &[Value]) -> EvalResult {
    if args.is_empty() {
        return Err(arity("/", 1, 0));
    }
    let mut terms = Vec::with_capacity(args.len() + 1);
    if args.len() == 1 {
        terms.push(1.0);
    }
    for v in args {
        terms.push(number_of("/", v)?);
    }
    let dividend = terms[0];
    let divisors = &terms[1..];
    if divisors.contains(&0.0) {
        return Err(SchemeError::DivisionByZero);
    }
    Ok(Value::real(
        divisors.iter().fold(dividend, |acc, x| acc / x),
    ))
}

/// A zero divisor raises the division error; the only other
/// `checked_rem`/`checked_div` failure is `i128::MIN` over `-1`.
fn remainder_error(divisor: i128) -> SchemeError {
    if divisor == 0 {
        SchemeError::DivisionByZero
    } else {
        SchemeError::Overflow
    }
}

/// The object language's numeric equality: exact comparison of the two
/// float values is the definition, not an approximation to avoid.
fn numeric_eq(a: f64, b: f64) -> bool {
    #[allow(
        clippy::float_cmp,
        reason = "the object language's = is exact numeric equality"
    )]
    let equal = a == b;
    equal
}

/// Two numeric arguments of a comparison.
fn compare(name: &str, args: &[Value], op: fn(f64, f64) -> bool) -> EvalResult {
    let (a, b) = two_args(name, args)?;
    Ok(Value::boolean(op(number_of(name, a)?, number_of(name, b)?)))
}

fn arity(name: &str, expected: usize, got: usize) -> SchemeError {
    SchemeError::WrongArity {
        procedure: name.to_owned(),
        expected: expected.to_string(),
        got,
    }
}

fn as_pair(name: &str, v: &Value) -> Result<sicp_runtime::Pair, SchemeError> {
    match v {
        Value::Pair(pair) => Ok(pair.clone()),
        other => Err(SchemeError::TypeMismatch(format!(
            "{name}: not a pair: {other}"
        ))),
    }
}

/// The object language's `eq?`: identity on pairs and procedures,
/// content on numbers, symbols, strings, booleans, and the empty list.
fn eq_values(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Pair(x), Value::Pair(y)) => sicp_runtime::eq_pair(x, y),
        (Value::Closure(x), Value::Closure(y)) => Rc::ptr_eq(x, y),
        (Value::Thunk(x), Value::Thunk(y)) => Rc::ptr_eq(x, y),
        _ => a == b,
    }
}

/// The book's `setup-environment`: a fresh global frame holding the
/// primitive procedures under their object-language names plus the
/// bindings of `true` and `false`, with `display` and `newline`
/// writing to the host's standard output.
#[must_use]
pub fn setup_environment() -> Rc<Env> {
    setup_environment_in(&OutputSink::Stdout)
}

/// [`setup_environment`] with the object language's `display` and
/// `newline` writing into `sink`.
#[must_use]
pub fn setup_environment_in(sink: &OutputSink) -> Rc<Env> {
    let env = Env::global();
    env.define(Rc::from("true"), Value::boolean(true));
    env.define(Rc::from("false"), Value::boolean(false));
    for (name, handler) in primitive_table(sink) {
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

/// Reads and evaluates every form of `text` in `env` on the calling
/// thread and answers each form's value in order, definitions answering
/// `ok`. The caller's stack carries the recursion, so deep programs go
/// through [`run_program`].
///
/// # Errors
/// The reader's parse errors and the first evaluation error.
pub fn eval_program(env: &Rc<Env>, text: &str) -> Result<Vec<Value>, SchemeError> {
    let forms = sicp_runtime::read_program(text)?;
    let mut values = Vec::with_capacity(forms.len());
    for form in &forms {
        values.push(Base.eval(form, env)?);
    }
    Ok(values)
}

/// Whether a top-level form's value line is suppressed because its
/// output is the side effect: `display` and `newline` applications.
fn is_output_form(env: &Rc<Env>, form: &Value) -> bool {
    let Value::Pair(cell) = form else {
        return false;
    };
    let Value::Sym(name) = &*cell.car.borrow() else {
        return false;
    };
    if &**name != "display" && &**name != "newline" {
        return false;
    }
    matches!(
        env.lookup(name),
        Ok(Value::Primitive { ref name, .. }) if &**name == "display" || &**name == "newline"
    )
}

/// Runs `text` as one program the way printer.md fixes the output: a
/// value line after every form except definitions, whose output is
/// nothing, and `display`/`newline` applications, whose output is the
/// side effect; a raised error prints one `Error:` line and stops the
/// program. The program runs on a worker thread with a 256 MiB stack
/// ([`EVAL_STACK_BYTES`]), fresh global environment included.
///
/// # Panics
/// Propagates a panic raised inside the evaluation, and panics when the
/// worker thread itself cannot be spawned, which only thread-creation
/// failure causes.
#[must_use]
pub fn run_program(text: &str) -> String {
    let text = text.to_owned();
    with_eval_stack(move || {
        let (sink, cell) = OutputSink::buffer();
        let env = setup_environment_in(&sink);
        let forms = match sicp_runtime::read_program(&text) {
            Ok(forms) => forms,
            Err(error) => return render_run(&cell, Some(error)),
        };
        for form in &forms {
            match Base.eval(form, &env) {
                Ok(value) if !is_definition(form) && !is_output_form(&env, form) => {
                    let mut out = cell.borrow_mut();
                    out.push_str(&print_value(&value));
                    out.push('\n');
                }
                Ok(_) => {}
                Err(error) => return render_run(&cell, Some(error)),
            }
        }
        render_run(&cell, None)
    })
}

fn render_run(cell: &Rc<RefCell<String>>, error: Option<SchemeError>) -> String {
    let mut text = cell.borrow().clone();
    if let Some(error) = error {
        write!(text, "Error: {error}").expect("a String write cannot fail");
        text.push('\n');
    }
    text
}

/// Runs `job` on a worker thread whose stack is [`EVAL_STACK_BYTES`]:
/// the bound that carries the corpus's 100,000-deep recursion, and the
/// honest postponement of the overflow the chapter 5 machine removes.
///
/// # Panics
/// Propagates a panic from `job`, and panics when the worker thread
/// cannot be spawned.
pub fn with_eval_stack<T: Send + 'static>(job: impl FnOnce() -> T + Send + 'static) -> T {
    let spawned = std::thread::Builder::new()
        .stack_size(EVAL_STACK_BYTES)
        .spawn(job);
    match spawned {
        Ok(handle) => match handle.join() {
            Ok(value) => value,
            Err(payload) => std::panic::resume_unwind(payload),
        },
        Err(error) => {
            panic!("the {EVAL_STACK_BYTES}-byte evaluation worker could not be spawned: {error}")
        }
    }
}

/// Runs `lines` the way the book's driver loop presents a session: an
/// input prompt before each form and an output prompt before each
/// value, in one fresh global environment. A form that raises ends the
/// transcript with one `Error:` line.
///
/// # Panics
/// Propagates a panic from the evaluation (see [`with_eval_stack`]).
#[must_use]
pub fn driver_transcript(lines: &[&str]) -> String {
    let (sink, cell) = OutputSink::buffer();
    let env = setup_environment_in(&sink);
    for line in lines {
        let mut out = cell.borrow_mut();
        out.push_str(";;; M-Eval input: ");
        out.push_str(line);
        out.push('\n');
        drop(out);
        let form = match sicp_runtime::read(line) {
            Ok(form) => form,
            Err(error) => return push_error(&cell, &error),
        };
        match Base.eval(&form, &env) {
            Ok(value) => {
                let mut out = cell.borrow_mut();
                out.push_str(";;; M-Eval value: ");
                out.push_str(&print_value(&value));
                out.push('\n');
            }
            Err(error) => return push_error(&cell, &error),
        }
    }
    cell.borrow().clone()
}

fn push_error(cell: &Rc<RefCell<String>>, error: &SchemeError) -> String {
    let mut out = cell.borrow_mut();
    write!(out, "Error: {error}").expect("a String write cannot fail");
    out.push('\n');
    drop(out);
    cell.borrow().clone()
}

// ---------------------------------------------------------------------------
// 4.1.7: the analyzed evaluator.
// ---------------------------------------------------------------------------

/// The analyzed evaluator's extension seam: the same one mechanism as
/// [`Evaluator`], one required hook, [`Analyzer::analyze`], with the
/// book's analysis procedures of 4.1.7 as provided methods that recurse
/// through `self.analyze`.
pub trait Analyzer {
    /// Analyzes one expression into its execution procedure; the hook
    /// every analysis recursion crosses.
    ///
    /// # Errors
    /// Whatever the analysis raises.
    fn analyze(&self, exp: &Value) -> Result<Exec, SchemeError>;

    /// The analyzed bodies, keyed by the procedure value's identity: the
    /// book stores the execution procedure inside the procedure object,
    /// and the shared `Closure` has no slot for it, so the table lives
    /// beside the evaluator, under an `Rc` the execution procedures
    /// capture.
    fn bodies(&self) -> &Rc<RefCell<std::collections::HashMap<usize, Exec>>>;

    /// The book's `analyze` cond chain: one arm per syntactic type, the
    /// analysis work done once per form.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on an unknown expression type;
    /// otherwise whatever the analysis raises.
    fn base_analyze(&self, exp: &Value) -> Result<Exec, SchemeError> {
        match exp {
            Value::Int(_) | Value::Real(_) | Value::Bool(_) | Value::Str(_) => {
                let constant = exp.clone();
                Ok(Rc::new(move |_| Ok(constant.clone())))
            }
            Value::Sym(name) => {
                let name = name.clone();
                Ok(Rc::new(move |env| lookup_variable_value(&name, env)))
            }
            _ if is_quoted(exp) => {
                let constant = text_of_quotation(exp)?;
                Ok(Rc::new(move |_| Ok(constant.clone())))
            }
            _ if is_assignment(exp) => {
                let name = assignment_variable(exp)?;
                let value = assignment_value(exp)?;
                let Value::Sym(name) = name else {
                    return Err(SchemeError::TypeMismatch(format!(
                        "not an assignment target: {name}"
                    )));
                };
                let vproc = self.analyze(&value)?;
                Ok(Rc::new(move |env| {
                    let value = vproc(env)?;
                    set_variable_value_(&name, value, env)?;
                    Ok(Value::sym("ok"))
                }))
            }
            _ if is_definition(exp) => self.analyze_definition(exp),
            _ if is_if(exp) => self.analyze_if(exp),
            _ if is_lambda(exp) => self.analyze_lambda(exp),
            _ if is_begin(exp) => {
                let actions = operand_items(exp)?;
                self.analyze_sequence(&actions)
            }
            _ if is_cond(exp) => self.analyze(&cond_to_if(exp)?),
            _ if is_application(exp) => self.analyze_application(exp),
            _ => Err(SchemeError::TypeMismatch(format!(
                "Unknown expression type: ANALYZE {exp}"
            ))),
        }
    }

    /// The book's `analyze-definition`: the value expression is
    /// analyzed once; the execution installs the definition.
    ///
    /// # Errors
    /// Whatever the analysis raises.
    fn analyze_definition(&self, exp: &Value) -> Result<Exec, SchemeError> {
        let name = definition_variable(exp)?;
        let Value::Sym(name) = name else {
            return Err(SchemeError::TypeMismatch(format!(
                "not a definition target: {name}"
            )));
        };
        let second = second_of(exp)?;
        if is_variable(&second) {
            let vproc = self.analyze(&third_of(exp)?)?;
            Ok(Rc::new(move |env| {
                let value = vproc(env)?;
                define_variable_(&name, value, env)
            }))
        } else {
            let (params, rest) = split_params(&rest_of(&second)?)?;
            let body = {
                let items = exp.list_items()?;
                items.into_iter().skip(2).collect::<Vec<_>>()
            };
            let bproc = self.analyze_sequence(&body)?;
            let bodies = Rc::clone(self.bodies());
            Ok(Rc::new(move |env| {
                let closure = Rc::new(Closure {
                    name: Some(Rc::clone(&name)),
                    params: params.clone(),
                    rest: rest.clone(),
                    body: body.clone(),
                    env: Rc::clone(env),
                });
                bodies
                    .borrow_mut()
                    .insert(body_key(&closure), Rc::clone(&bproc));
                define_variable_(&name, Value::Closure(closure), env)
            }))
        }
    }

    /// The book's `analyze-if`: the three parts analyzed once, the
    /// branch chosen at execution.
    ///
    /// # Errors
    /// Whatever the analysis raises.
    fn analyze_if(&self, exp: &Value) -> Result<Exec, SchemeError> {
        let (predicate, consequent, alternative) = if_parts(exp)?;
        let pproc = self.analyze(&predicate)?;
        let cproc = self.analyze(&consequent)?;
        let aproc = self.analyze(&alternative)?;
        Ok(Rc::new(move |env| {
            if is_true(&pproc(env)?) {
                cproc(env)
            } else {
                aproc(env)
            }
        }))
    }

    /// The book's `analyze-lambda`: the body analyzed once even though
    /// the procedure may be applied many times.
    ///
    /// # Errors
    /// Whatever the analysis raises.
    fn analyze_lambda(&self, exp: &Value) -> Result<Exec, SchemeError> {
        let (params, rest) = lambda_parameters(exp)?;
        let body = lambda_body(exp)?;
        let bproc = self.analyze_sequence(&body)?;
        let bodies = Rc::clone(self.bodies());
        Ok(Rc::new(move |env| {
            let closure = Rc::new(Closure {
                name: None,
                params: params.clone(),
                rest: rest.clone(),
                body: body.clone(),
                env: Rc::clone(env),
            });
            bodies
                .borrow_mut()
                .insert(body_key(&closure), Rc::clone(&bproc));
            Ok(Value::Closure(closure))
        }))
    }

    /// The book's `analyze-sequence`: each expression analyzed once,
    /// the execution procedures combined so the calls are built in.
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] on an empty sequence; otherwise
    /// whatever the analysis raises.
    fn analyze_sequence(&self, exps: &[Value]) -> Result<Exec, SchemeError> {
        let Some((first, rest)) = exps.split_first() else {
            return Err(SchemeError::TypeMismatch(
                "Empty sequence: ANALYZE".to_owned(),
            ));
        };
        if rest.is_empty() {
            return self.analyze(first);
        }
        let first_proc = self.analyze(first)?;
        let rest_proc = self.analyze_sequence(rest)?;
        Ok(sequentially(first_proc, rest_proc))
    }

    /// The book's `analyze-application`: operator and operands analyzed
    /// once; the execution gathers the arguments and hands them to
    /// [`Analyzer::execute_application`].
    ///
    /// # Errors
    /// Whatever the analysis raises.
    fn analyze_application(&self, exp: &Value) -> Result<Exec, SchemeError> {
        let fproc = self.analyze(&first_of(exp)?)?;
        let operands = operand_items(exp)?;
        let aprocs: Vec<Exec> = operands
            .iter()
            .map(|op| self.analyze(op))
            .collect::<Result<_, _>>()?;
        let bodies = Rc::clone(self.bodies());
        Ok(Rc::new(move |env| {
            let proc = fproc(env)?;
            let mut args = Vec::with_capacity(aprocs.len());
            for aproc in &aprocs {
                args.push(aproc(env)?);
            }
            execute_with_bodies(&bodies, &proc, &args)
        }))
    }

    /// The book's `execute-application`: the analog of `apply`, with the
    /// compound body already analyzed, so the extended environment goes
    /// straight to the registered execution procedure. The object
    /// language's `apply` and `map` route through here like the base
    /// evaluator's.
    ///
    /// # Errors
    /// [`SchemeError::NotProcedure`] when `proc` is not a procedure;
    /// otherwise whatever the application raises.
    fn execute_application(&self, proc: &Value, args: &[Value]) -> EvalResult {
        execute_with_bodies(self.bodies(), proc, args)
    }

    /// Analyzes `exp` and runs the result in `env`, the book's
    /// `(define (eval exp env) ((analyze exp) env))`.
    ///
    /// # Errors
    /// Whatever the analysis or execution raises.
    fn eval_exp(&self, exp: &Value, env: &Rc<Env>) -> EvalResult {
        let exec = self.analyze(exp)?;
        exec(env)
    }
}

/// The analyzed-body table the execution procedures share.
type Bodies = Rc<RefCell<std::collections::HashMap<usize, Exec>>>;

/// The book's `execute-application`, free of `self` so an analysis
/// closure can call it with nothing but the body table it captured:
/// primitives run their handlers, the `apply` and `map` sentinels route
/// procedure values back through here, and a compound procedure's
/// extended environment goes straight to its registered execution
/// procedure.
fn execute_with_bodies(bodies: &Bodies, proc: &Value, args: &[Value]) -> EvalResult {
    match proc {
        Value::Primitive { name, .. } if &**name == "apply" => {
            let [target, list, ..] = args else {
                return Err(arity("apply", 2, args.len()));
            };
            let items = list.list_items()?;
            execute_with_bodies(bodies, target, &items)
        }
        Value::Primitive { name, .. } if &**name == "map" => {
            let [f, lists @ ..] = args else {
                return Err(arity("map", 2, args.len()));
            };
            let columns: Vec<Vec<Value>> = lists
                .iter()
                .map(sicp_runtime::Value::list_items)
                .collect::<Result<_, _>>()?;
            let depth = columns.iter().map(Vec::len).min().unwrap_or(0);
            let mut out = Vec::with_capacity(depth);
            for index in 0..depth {
                let row: Vec<Value> = columns.iter().map(|col| col[index].clone()).collect();
                out.push(execute_with_bodies(bodies, f, &row)?);
            }
            Ok(Value::list(out))
        }
        Value::Primitive { f, .. } => f(args),
        Value::Closure(c) => {
            let frame =
                extend_environment(closure_name(c), &c.params, c.rest.as_ref(), args, &c.env)?;
            let body = bodies.borrow().get(&body_key(c)).cloned();
            match body {
                Some(exec) => exec(&frame),
                None => Err(SchemeError::TypeMismatch(
                    "the analyzed body is missing".to_owned(),
                )),
            }
        }
        other => Err(SchemeError::NotProcedure(other.clone())),
    }
}

/// The key a closure's analyzed body registers under: the pointer
/// identity of the procedure value, the shared-closure stand-in for the
/// book's slot inside the procedure object.
fn body_key(closure: &Rc<Closure>) -> usize {
    Rc::as_ptr(closure) as usize
}

/// The book's `sequentially`: two execution procedures combined into
/// one that runs the first for effect and answers the second.
fn sequentially(proc1: Exec, proc2: Exec) -> Exec {
    Rc::new(move |env| {
        proc1(env)?;
        proc2(env)
    })
}

/// The base analyzed evaluator: 4.1.7 as the book presents it.
#[derive(Default)]
pub struct AnalyzerBase {
    bodies: Rc<RefCell<std::collections::HashMap<usize, Exec>>>,
}

impl AnalyzerBase {
    /// A fresh analyzed evaluator with an empty body table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Analyzer for AnalyzerBase {
    fn analyze(&self, exp: &Value) -> Result<Exec, SchemeError> {
        self.base_analyze(exp)
    }

    fn bodies(&self) -> &Rc<RefCell<std::collections::HashMap<usize, Exec>>> {
        &self.bodies
    }
}

/// Analyzes one expression with the base analyzer, the analyzed
/// evaluator in one call.
///
/// # Errors
/// Whatever the analysis raises.
pub fn analyze(exp: &Value) -> Result<Exec, SchemeError> {
    AnalyzerBase::default().analyze(exp)
}

#[cfg(test)]
mod tests {
    use super::{
        analyze, cond_to_if, driver_transcript, eval, eval_program, run_program, setup_environment,
    };
    use std::rc::Rc;

    use sicp_runtime::{SchemeError, Value, print_value};

    fn eval_text(text: &str) -> String {
        let env = setup_environment();
        let values = eval_program(&env, text).expect("evaluates");
        values
            .iter()
            .map(print_value)
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn eval_drives_the_core_forms() {
        assert_eq!(
            eval_text("(define (square x) (* x x))\n(square 21)"),
            "ok\n441"
        );
        assert_eq!(
            eval_text("(cond ((= 1 2) 'no) ((= 1 1) 'yes) (else 'else))"),
            "yes"
        );
        assert_eq!(
            eval_text("(let ((x 3) (y 4)) (+ x y))"),
            "7",
            "the grammar's derived let is base behavior"
        );
    }

    #[test]
    fn set_bang_rebinds_and_define_creates() {
        assert_eq!(eval_text("(define x 1)\n(set! x 5)\nx"), "ok\nok\n5");
        let env = setup_environment();
        let Err(error) = eval_program(&env, "(set! nothing 1)") else {
            panic!("setting an unbound name must raise");
        };
        assert!(matches!(error, SchemeError::UnboundVariable(_)));
    }

    #[test]
    fn tail_positions_run_in_constant_host_stack() {
        let program =
            "(define (count-down n) (if (= n 0) 'done (count-down (- n 1))))\n(count-down 100000)";
        assert_eq!(
            eval_text(program),
            "ok\ndone",
            "deep tail recursion is flat"
        );
    }

    #[test]
    fn quoted_data_and_printed_values_match_the_printer() {
        assert_eq!(eval_text("'(a b c)"), "(a b c)");
        assert_eq!(eval_text("(cons 1 2)"), "(1 . 2)");
        assert_eq!(eval_text("\"hi\""), "\"hi\"");
    }

    #[test]
    fn cond_to_if_rewrites_a_cond() {
        let cond = sicp_runtime::read("(cond ((> x 0) 'pos) (else 'neg))").expect("read");
        let rewritten = cond_to_if(&cond).expect("rewrites");
        assert_eq!(
            print_value(&rewritten),
            "(if (> x 0) (quote pos) (quote neg))"
        );
    }

    #[test]
    fn run_program_renders_printer_md_output() {
        let output =
            run_program("(define (square x) (* x x))\n(square 21)\n(display 'hi)\n(newline)\n");
        assert_eq!(
            output, "441\nhi\n",
            "display writes its side effect and no value line"
        );
    }

    #[test]
    fn run_program_stops_after_one_error_line() {
        let output = run_program("(+ 1 2)\n(car 5)\n(+ 3 4)");
        assert_eq!(output, "3\nError: type mismatch: car of a non-pair: 5\n");
    }

    #[test]
    fn driver_transcript_shows_the_book_prompts() {
        let text = driver_transcript(&[
            "(define (append x y) (if (null? x) y (cons (car x) (append (cdr x) y))))",
            "(append '(a b c) '(d e f))",
        ]);
        assert_eq!(
            text,
            ";;; M-Eval input: (define (append x y) (if (null? x) y (cons (car x) (append (cdr x) y))))\n\
             ;;; M-Eval value: ok\n\
             ;;; M-Eval input: (append '(a b c) '(d e f))\n\
             ;;; M-Eval value: (a b c d e f)\n"
        );
    }

    #[test]
    fn apply_and_map_reach_compound_procedures() {
        assert_eq!(
            eval_text("(define (twice f x) (apply f (list x x)))\n(twice cons 7)"),
            "ok\n(7 . 7)"
        );
        assert_eq!(
            eval_text("(define (sq x) (* x x))\n(map sq '(1 2 3))"),
            "ok\n(1 4 9)",
            "map applies a compound procedure through the evaluator"
        );
    }

    #[test]
    fn analyze_runs_the_same_subset() {
        let env = setup_environment();
        let exp = sicp_runtime::read("(if (< 1 2) (+ 20 22) 0)").expect("read");
        let exec = analyze(&exp).expect("analyzes");
        assert_eq!(exec(&Rc::clone(&env)), Ok(Value::int(42)));
        assert_eq!(eval(&exp, &Rc::clone(&env)), Ok(Value::int(42)));
    }
}
