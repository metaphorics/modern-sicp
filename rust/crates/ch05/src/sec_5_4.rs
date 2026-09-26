// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.4

//! Section 5.4: The explicit-control evaluator.
//!
//! The book's register machine that runs the metacircular evaluator's
//! algorithm directly, as a controller sequence over the
//! [section 5.2 simulator](crate::sec_5_2): the registers, the
//! monitored stack, the flag, and the assembler are exactly 5.2's, and
//! the controller is the book's text in the book's notation, held
//! here as [`controller_fragments`].
//!
//! Machine words. The runtime [`Value`] type is the word type, the 5.3
//! precedent, so every word rides in a machine register unchanged:
//! object values are themselves, and the object-language expressions
//! are the list structure the reader produced, the book's own uniform
//! representation, so the syntax operations are the 4.1.2 list
//! procedures the controller names, [`base_operations`]. The
//! evaluator's own data are tagged words no object-language value can
//! spell (the reserved `sicp-word:` tag prefix): the environment word
//! (a handle into the environment table), the procedure words, the
//! thunk word of exercise 5.25, and the condition words of exercise
//! 5.30. Labels are the label-name symbols the 5.2 assembler's
//! `(goto (reg continue))` consumes.
//!
//! Nothing raises out of a correct run: the driver loop ends when the
//! input queue runs dry, through the typed [`Fault::Op`] whose
//! message is [`INPUT_EXHAUSTED`], and every other failure of the
//! evaluator stops the machine with a typed fault too. The
//! [`Evaluator::run`] wrapper reads the queue-dry fault as the run's
//! normal end.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::sec_5_2::{Fault, Machine, OpHandler, make_machine};
use sicp_runtime::{Env, Value, display_value, read_program};

// The reserved tag prefix of the evaluator's own words is
// `sicp-word:`: no object-language value carries it, because the
// object language has no tagged words.

const ENVIRONMENT_TAG: &str = "sicp-word: environment";
const PRIMITIVE_TAG: &str = "sicp-word: primitive";
const COMPOUND_TAG: &str = "sicp-word: compound-procedure";
const THUNK_TAG: &str = "sicp-word: thunk";
const CONDITION_TAG: &str = "sicp-word: condition";

/// The message whose read fault ends the driver loop when the input
/// queue runs dry: the edition's stop for the book's unbounded
/// read-eval-print loop.
pub const INPUT_EXHAUSTED: &str = "the evaluator's input queue is empty";

/// The registers of the evaluator machine description: the book's
/// seven. The machine's own `flag` is never named by a controller and
/// is never allocated as a register.
const EVALUATOR_REGISTERS: &[&str] = &["exp", "env", "val", "continue", "proc", "argl", "unev"];

/// Builds a `Fault::Op` with the message only; the assembler
/// decorates the fault with the operation's name and the step.
fn op_fail(message: impl Into<String>) -> Fault {
    Fault::Op {
        op: String::new(),
        message: message.into(),
        step: 0,
    }
}

// ---------------------------------------------------------------------------
// Machine words
// ---------------------------------------------------------------------------

/// The tag of a tagged word, or nothing for a plain value.
fn word_tag(word: &Value) -> Option<&str> {
    match word {
        Value::Tagged { tag, .. } => Some(tag.as_ref()),
        _ => None,
    }
}

/// Whether the word carries the given evaluator tag.
fn is_word_kind(word: &Value, tag: &str) -> bool {
    word_tag(word) == Some(tag)
}

/// The environment word: a handle into the environment table, what
/// the `env` register holds.
#[must_use]
pub fn env_word(handle: usize) -> Value {
    // usize to i128 widens for every handle the table can hold.
    #[allow(clippy::cast_possible_wrap)]
    Value::tagged(ENVIRONMENT_TAG, Value::Int(handle as i128))
}

/// The handle an environment word names.
#[must_use]
pub fn env_handle(word: &Value) -> Option<usize> {
    if !is_word_kind(word, ENVIRONMENT_TAG) {
        return None;
    }
    let Value::Tagged { data, .. } = word else {
        return None;
    };
    let Value::Int(handle) = **data else {
        return None;
    };
    usize::try_from(handle).ok()
}

/// The primitive procedure word: the name into the primitive table.
#[must_use]
pub fn primitive_word(name: &str) -> Value {
    Value::tagged(PRIMITIVE_TAG, Value::sym(name))
}

/// The name a primitive procedure word carries.
#[must_use]
pub fn primitive_name(word: &Value) -> Option<&str> {
    if !is_word_kind(word, PRIMITIVE_TAG) {
        return None;
    }
    let Value::Tagged { data, .. } = word else {
        return None;
    };
    match &**data {
        Value::Sym(name) => Some(name),
        _ => None,
    }
}

/// The compound procedure word: the parameter list, the body, and the
/// environment word, the book's `make-procedure` product.
#[must_use]
pub fn compound_word(parameters: Value, body: Value, environment: Value) -> Value {
    Value::tagged(
        COMPOUND_TAG,
        Value::list(vec![parameters, body, environment]),
    )
}

/// The `(parameters, body, environment)` a compound procedure word
/// carries.
#[must_use]
pub fn compound_parts(word: &Value) -> Option<(Value, Value, Value)> {
    if !is_word_kind(word, COMPOUND_TAG) {
        return None;
    }
    let Value::Tagged { data, .. } = word else {
        return None;
    };
    let items = data.list_items().ok()?;
    Some((
        items.first()?.clone(),
        items.get(1)?.clone(),
        items.get(2)?.clone(),
    ))
}

/// The thunk word of exercise 5.25: the delayed expression and the
/// environment it delays over. The base evaluator never builds one.
#[must_use]
pub fn thunk_word(expression: Value, environment: Value) -> Value {
    Value::tagged(THUNK_TAG, Value::list(vec![expression, environment]))
}

/// Whether the word is a thunk word.
#[must_use]
pub fn is_thunk(word: &Value) -> bool {
    is_word_kind(word, THUNK_TAG)
}

/// The `(expression, environment)` a thunk word delays.
#[must_use]
pub fn thunk_parts(word: &Value) -> Option<(Value, Value)> {
    if !is_word_kind(word, THUNK_TAG) {
        return None;
    }
    let Value::Tagged { data, .. } = word else {
        return None;
    };
    let items = data.list_items().ok()?;
    Some((items.first()?.clone(), items.get(1)?.clone()))
}

/// The condition word of exercise 5.30: a reserved tag no object
/// value can spell, the condition's name, and the detail
/// `signal-error` reports.
#[must_use]
pub fn condition_word(name: &str, detail: &str) -> Value {
    Value::tagged(
        CONDITION_TAG,
        Value::list(vec![Value::sym(name), Value::string(detail)]),
    )
}

/// Whether the word is the condition word of the given name.
#[must_use]
pub fn is_condition(word: &Value, name: &str) -> bool {
    condition_parts(word).is_some_and(|(found, _)| found == name)
}

/// The `(name, detail)` a condition word carries.
#[must_use]
pub fn condition_parts(word: &Value) -> Option<(String, String)> {
    if !is_word_kind(word, CONDITION_TAG) {
        return None;
    }
    let Value::Tagged { data, .. } = word else {
        return None;
    };
    let items = data.list_items().ok()?;
    let Value::Sym(name) = items.first()? else {
        return None;
    };
    let Value::Str(detail) = items.get(1)? else {
        return None;
    };
    Some((name.to_string(), detail.to_string()))
}

/// Renders a word for the transcript: the detail of a condition word,
/// the word's own name for the evaluator's other words, and the
/// displayed form of every plain value.
#[must_use]
pub fn render_word(word: &Value) -> String {
    if let Some((_, detail)) = condition_parts(word) {
        return detail;
    }
    match word {
        _ if env_handle(word).is_some() => "#[environment]".to_owned(),
        _ => match primitive_name(word) {
            Some(name) => format!("#[primitive-procedure {name}]"),
            None if compound_parts(word).is_some() => "#[compound-procedure]".to_owned(),
            None if is_thunk(word) => "#[thunk]".to_owned(),
            None => display_value(word),
        },
    }
}

// ---------------------------------------------------------------------------
// The environment table
// ---------------------------------------------------------------------------

thread_local! {
    /// The environment table the environment words name entries in:
    /// process-wide because the machine's operations are stateless
    /// closures, and grow-only so a handle never changes meaning.
    static ENVIRONMENTS: RefCell<Vec<Rc<Env>>> = const { RefCell::new(Vec::new()) };
}

/// Enters an environment in the table and answers its word.
fn intern_environment(environment: Rc<Env>) -> Value {
    ENVIRONMENTS.with(|table| {
        let mut table = table.borrow_mut();
        let handle = table.len();
        table.push(environment);
        env_word(handle)
    })
}

/// Reads the environment a word names, the public shape the
/// exercise solutions' overridden environment operations use.
///
/// # Errors
/// [`Fault::Op`] when the word is not an environment word or names
/// no table entry.
pub fn environment_of(word: &Value, what: &str) -> Result<Rc<Env>, Fault> {
    lookup_environment(word, what)
}

/// Enters an environment in the evaluator's table and answers its
/// word: the public shape the section 5.5 machine's runtime
/// environment primitives use, so their words are readable by every
/// base operation.
#[must_use]
pub fn intern_env(environment: Rc<Env>) -> Value {
    intern_environment(environment)
}

/// Builds one operation over words for an exercise's table: the
/// public shape the solutions' extra operations use.
pub fn operation(
    name: &'static str,
    f: impl Fn(&[Value]) -> Result<Value, Fault> + 'static,
) -> (&'static str, OpHandler) {
    word_op(name, f)
}

/// Reads the environment a word names.
fn lookup_environment(word: &Value, what: &str) -> Result<Rc<Env>, Fault> {
    let handle = env_handle(word).ok_or_else(|| op_fail(format!("{what} needs an environment")))?;
    ENVIRONMENTS
        .with(|table| table.borrow().get(handle).cloned())
        .ok_or_else(|| op_fail(format!("{what} names no environment")))
}

// ---------------------------------------------------------------------------
// Word shorthands the operations share
// ---------------------------------------------------------------------------

fn one(name: &str, args: &[Value]) -> Result<(Value,), Fault> {
    let [a] = args else {
        return Err(op_fail(format!("{name}: needs one argument")));
    };
    Ok((a.clone(),))
}

fn two(name: &str, args: &[Value]) -> Result<(Value, Value), Fault> {
    let [a, b] = args else {
        return Err(op_fail(format!("{name}: needs two arguments")));
    };
    Ok((a.clone(), b.clone()))
}

fn three(name: &str, args: &[Value]) -> Result<(Value, Value, Value), Fault> {
    let [a, b, c] = args else {
        return Err(op_fail(format!("{name}: needs three arguments")));
    };
    Ok((a.clone(), b.clone(), c.clone()))
}

// ---------------------------------------------------------------------------
// Syntax: the 4.1.2 list procedures over words
// ---------------------------------------------------------------------------

/// The items of a proper list word, or nothing.
fn items_of(word: &Value) -> Option<Vec<Value>> {
    word.list_items().ok()
}

/// The items when `word` is the tagged list `(tag ...)`.
fn tagged_items(word: &Value, tag: &str) -> Option<Vec<Value>> {
    let items = items_of(word)?;
    if matches!(items.first(), Some(Value::Sym(name)) if name.as_ref() == tag) {
        Some(items)
    } else {
        None
    }
}

fn is_symbol(word: &Value) -> bool {
    matches!(word, Value::Sym(_))
}

fn symbol_name(word: &Value) -> Option<String> {
    match word {
        Value::Sym(name) => Some(name.to_string()),
        _ => None,
    }
}

/// The book's `self-evaluating?`: numbers, strings, and booleans.
fn is_self_evaluating(word: &Value) -> bool {
    matches!(
        word,
        Value::Int(_) | Value::Real(_) | Value::Str(_) | Value::Bool(_)
    )
}

/// The book's `true?`: every value counts as true except `false`.
#[must_use]
pub fn object_is_true(value: &Value) -> bool {
    !matches!(value, Value::Bool(false))
}

/// Whether a define target names a value (`x`) or a procedure
/// (`(f args...)`).
fn is_definition_target(word: &Value) -> bool {
    matches!(word, Value::Sym(_) | Value::Pair(_))
}

/// Builds the lambda a procedure-form define names:
/// `(lambda parameters body...)`.
fn lambda_form(parameters: Value, body: &[Value]) -> Value {
    let mut form = vec![Value::sym("lambda"), parameters];
    form.extend(body.iter().cloned());
    Value::list(form)
}

/// The operand list word: the book's `adjoin-arg` appends at the end,
/// the book's order.
fn adjoin_arg(word: Value, argl: &Value) -> Result<Value, Fault> {
    let mut items = argl
        .list_items()
        .map_err(|_| op_fail("adjoin-arg needs an operand list"))?;
    items.push(word);
    Ok(Value::list(items))
}

/// Whether a sequence word is empty and whether it is down to its
/// last expression.
fn sequence_flags(name: &str, word: &Value) -> Result<(bool, bool), Fault> {
    let items = items_of(word).ok_or_else(|| op_fail(format!("{name} needs a sequence")))?;
    Ok((items.is_empty(), items.len() == 1))
}

// ---------------------------------------------------------------------------
// The object-language primitives
// ---------------------------------------------------------------------------

/// The exact integer a primitive argument must carry.
fn number_of(name: &str, word: &Value) -> Result<i128, Fault> {
    match word {
        Value::Int(n) => Ok(*n),
        other => Err(op_fail(format!(
            "{name}: needs a number, got {}",
            display_value(other)
        ))),
    }
}

fn int_pair(name: &str, args: &[Value]) -> Result<(i128, i128), Fault> {
    let (a, b) = two(name, args)?;
    Ok((number_of(name, &a)?, number_of(name, &b)?))
}

/// The object language's `eq?`: identity on pairs, content on
/// numbers, symbols, strings, booleans, and the empty list.
fn eq_values(a: &Value, b: &Value) -> bool {
    if let (Value::Pair(x), Value::Pair(y)) = (a, b) {
        return Rc::ptr_eq(x, y);
    }
    a == b
}

/// The object-language arithmetic: `+`, `-`, and `*` fold n-ary over
/// exact integers with the `+` and `*` identities and unary `-` as
/// the negation; `/` answers an exact quotient when the division is
/// exact and a real otherwise, and a zero divisor is the division
/// failure that exercise 5.30 turns into a condition code.
fn arith(name: &str, args: &[Value]) -> Result<Value, Fault> {
    let overflow = || op_fail("arithmetic overflow");
    let mut acc = number_of(name, args.first().unwrap_or(&Value::Nil))?;
    for word in args.iter().skip(1) {
        let next = number_of(name, word)?;
        acc = match name {
            "+" => acc.checked_add(next).ok_or_else(overflow)?,
            "-" => acc.checked_sub(next).ok_or_else(overflow)?,
            "*" => acc.checked_mul(next).ok_or_else(overflow)?,
            _ => return Err(op_fail(format!("{name}: is not arithmetic"))),
        };
    }
    Ok(Value::Int(acc))
}

fn comparison(name: &str, args: &[Value], pick: fn(i128, i128) -> bool) -> Result<Value, Fault> {
    let (a, b) = int_pair(name, args)?;
    Ok(Value::boolean(pick(a, b)))
}

/// The names of the object primitives the evaluator's global
/// environment binds: the public shape the section 5.5 machine's
/// runtime table extends.
#[must_use]
pub fn object_primitive_names() -> &'static [&'static str] {
    OBJECT_PRIMITIVES
}

/// Applies the object-language primitive `name` to the values `args`:
/// the evaluator's `apply-primitive-procedure` calls it with `proc`'s
/// name, and the error-signaling exercise wraps its failures in
/// condition words.
///
/// # Errors
/// [`Fault::Op`] carrying the primitive's own message; an unknown
/// name is the unknown-operation fault.
pub fn apply_object_primitive(name: &str, args: &[Value]) -> Result<Value, Fault> {
    match name {
        "cons" => {
            let (a, b) = two(name, args)?;
            Ok(Value::Pair(sicp_runtime::cons_cell(a, b)))
        }
        "car" => match one(name, args)?.0 {
            Value::Pair(pair) => Ok(pair.car.borrow().clone()),
            other => Err(op_fail(format!(
                "car: not a pair: {}",
                display_value(&other)
            ))),
        },
        "cdr" => match one(name, args)?.0 {
            Value::Pair(pair) => Ok(pair.cdr.borrow().clone()),
            other => Err(op_fail(format!(
                "cdr: not a pair: {}",
                display_value(&other)
            ))),
        },
        "null?" => Ok(Value::boolean(one(name, args)?.0.is_nil())),
        "pair?" => Ok(Value::boolean(matches!(one(name, args)?.0, Value::Pair(_)))),
        "symbol?" => Ok(Value::boolean(matches!(one(name, args)?.0, Value::Sym(_)))),
        "number?" => Ok(Value::boolean(matches!(
            one(name, args)?.0,
            Value::Int(_) | Value::Real(_)
        ))),
        "string?" => Ok(Value::boolean(matches!(one(name, args)?.0, Value::Str(_)))),
        "not" => Ok(Value::boolean(!object_is_true(&one(name, args)?.0))),
        "eq?" => {
            let (a, b) = two(name, args)?;
            Ok(Value::boolean(eq_values(&a, &b)))
        }
        "equal?" => {
            let (a, b) = two(name, args)?;
            Ok(Value::boolean(a == b))
        }
        "list" => Ok(Value::list(args.to_vec())),
        "+" | "*" => arith(name, args),
        "-" => unary_or_fold(name, args, i128::checked_neg),
        "/" => divide(args),
        "=" => comparison(name, args, |a, b| a == b),
        "<" => comparison(name, args, |a, b| a < b),
        ">" => comparison(name, args, |a, b| a > b),
        "remainder" => {
            let (a, b) = int_pair(name, args)?;
            if b == 0 {
                return Err(op_fail("division by zero"));
            }
            Ok(Value::Int(a.wrapping_rem(b)))
        }
        other => Err(Fault::UnknownOperation {
            op: other.to_owned(),
        }),
    }
}

/// The negation of one argument, or the n-ary subtraction.
fn unary_or_fold(
    name: &str,
    args: &[Value],
    negate: fn(i128) -> Option<i128>,
) -> Result<Value, Fault> {
    let first = number_of(name, args.first().unwrap_or(&Value::Nil))?;
    if args.len() == 1 {
        let negated = negate(first).ok_or_else(|| op_fail("arithmetic overflow"))?;
        return Ok(Value::Int(negated));
    }
    arith(name, args)
}

/// The quotient: exact when the division divides evenly, a real
/// otherwise, and the division failure on a zero divisor.
fn divide(args: &[Value]) -> Result<Value, Fault> {
    let mut acc = number_of("/", args.first().unwrap_or(&Value::Nil))?;
    for word in args.iter().skip(1) {
        let divisor = number_of("/", word)?;
        if divisor == 0 {
            return Err(op_fail("division by zero"));
        }
        if acc % divisor == 0 {
            acc /= divisor;
        } else {
            #[expect(
                clippy::cast_precision_loss,
                reason = "the section's quotients are far below f64's exact range, \
                          so the lossy widening is wanted here"
            )]
            let exact = acc as f64 / divisor as f64;
            return Ok(Value::Real(exact));
        }
    }
    Ok(Value::Int(acc))
}

// ---------------------------------------------------------------------------
// The base operations table
// ---------------------------------------------------------------------------

/// One operation over words: the shape the simulator's
/// [`OpHandler`] expects, with the word-level errors the evaluator
/// raises.
fn word_op(
    name: &'static str,
    f: impl Fn(&[Value]) -> Result<Value, Fault> + 'static,
) -> (&'static str, OpHandler) {
    let handler: OpHandler = Rc::new(move |_, args| f(args));
    (name, handler)
}

/// One action under `perform`.
fn word_action(
    name: &'static str,
    f: impl Fn(&[Value]) -> Result<(), Fault> + 'static,
) -> (&'static str, OpHandler) {
    let handler: OpHandler = Rc::new(move |_, args| f(args).map(|()| Value::sym("done")));
    (name, handler)
}

/// Reads one environment word argument.
fn environment_arg(args: &[Value], at: usize, what: &str) -> Result<Rc<Env>, Fault> {
    let word = args
        .get(at)
        .ok_or_else(|| op_fail(format!("{what}: missing an argument")))?;
    lookup_environment(word, what)
}

fn variable_arg(args: &[Value], at: usize, what: &str) -> Result<String, Fault> {
    let word = args
        .get(at)
        .ok_or_else(|| op_fail(format!("{what}: missing an argument")))?;
    symbol_name(word).ok_or_else(|| op_fail(format!("{what} needs a variable")))
}

fn selector(
    name: &'static str,
    tag: &'static str,
    at: usize,
    missing: &'static str,
) -> (&'static str, OpHandler) {
    word_op(name, move |args| {
        let (word,) = one(name, args)?;
        tagged_items(&word, tag)
            .and_then(|items| items.get(at).cloned())
            .ok_or_else(|| op_fail(missing))
    })
}

/// The syntax, sequence, operand-list, procedure, and environment
/// operations of 4.1 the controller names, typed over words. The
/// driver's own operations (the input queue, the transcript, and the
/// global environment) are installed by [`make_evaluator`], which
/// lets an exercise's `operations` override any name here.
/// The table is the section's one operations listing; splitting it
/// would scatter the book's single table over artificial helpers.
#[expect(
    clippy::too_many_lines,
    reason = "the table is the section's operations verbatim"
)]
#[must_use]
pub fn base_operations() -> Vec<(&'static str, OpHandler)> {
    vec![
        // -- the dispatch predicates --
        word_op("self-evaluating?", |args| {
            Ok(Value::boolean(is_self_evaluating(
                &one("self-evaluating?", args)?.0,
            )))
        }),
        word_op("variable?", |args| {
            Ok(Value::boolean(is_symbol(&one("variable?", args)?.0)))
        }),
        word_op("quoted?", |args| {
            Ok(Value::boolean(
                tagged_items(&one("quoted?", args)?.0, "quote")
                    .is_some_and(|items| items.len() == 2),
            ))
        }),
        word_op("assignment?", |args| {
            Ok(Value::boolean(
                tagged_items(&one("assignment?", args)?.0, "set!")
                    .is_some_and(|items| items.len() == 3),
            ))
        }),
        word_op("definition?", |args| {
            let word = one("definition?", args)?.0;
            let shaped = tagged_items(&word, "define")
                .is_some_and(|items| items.len() >= 3 && is_definition_target(&items[1]));
            Ok(Value::boolean(shaped))
        }),
        word_op("if?", |args| {
            Ok(Value::boolean(
                tagged_items(&one("if?", args)?.0, "if")
                    .is_some_and(|items| items.len() == 3 || items.len() == 4),
            ))
        }),
        word_op("lambda?", |args| {
            Ok(Value::boolean(
                tagged_items(&one("lambda?", args)?.0, "lambda")
                    .is_some_and(|items| items.len() >= 3),
            ))
        }),
        word_op("begin?", |args| {
            Ok(Value::boolean(
                tagged_items(&one("begin?", args)?.0, "begin")
                    .is_some_and(|items| items.len() >= 2),
            ))
        }),
        word_op("application?", |args| {
            Ok(Value::boolean(matches!(
                one("application?", args)?.0,
                Value::Pair(_)
            )))
        }),
        // -- simple expressions --
        selector(
            "text-of-quotation",
            "quote",
            1,
            "text-of-quotation needs a quotation",
        ),
        selector(
            "lambda-parameters",
            "lambda",
            1,
            "lambda-parameters needs a lambda",
        ),
        word_op("lambda-body", |args| {
            let items = tagged_items(&one("lambda?", args)?.0, "lambda")
                .ok_or_else(|| op_fail("lambda-body needs a lambda"))?;
            Ok(Value::list(items.into_iter().skip(2).collect()))
        }),
        // -- applications --
        word_op("operator", |args| {
            let items = items_of(&one("operator", args)?.0)
                .ok_or_else(|| op_fail("operator needs an application"))?;
            items
                .first()
                .cloned()
                .ok_or_else(|| op_fail("operator of an empty combination"))
        }),
        word_op("operands", |args| {
            let items = items_of(&one("operands", args)?.0)
                .ok_or_else(|| op_fail("operands needs an application"))?;
            Ok(Value::list(items.into_iter().skip(1).collect()))
        }),
        word_op("no-operands?", |args| {
            let (empty, _) = sequence_flags("no-operands?", &one("no-operands?", args)?.0)?;
            Ok(Value::boolean(empty))
        }),
        word_op("first-operand", |args| {
            let items = items_of(&one("first-operand", args)?.0)
                .ok_or_else(|| op_fail("first-operand needs a list"))?;
            items
                .first()
                .cloned()
                .ok_or_else(|| op_fail("first-operand of an empty list"))
        }),
        word_op("rest-operands", |args| {
            let items = items_of(&one("rest-operands", args)?.0)
                .ok_or_else(|| op_fail("rest-operands needs a list"))?;
            Ok(Value::list(items.into_iter().skip(1).collect()))
        }),
        word_op("last-operand?", |args| {
            let (_, last) = sequence_flags("last-operand?", &one("last-operand?", args)?.0)?;
            Ok(Value::boolean(last))
        }),
        word_op("empty-arglist", |_| Ok(Value::Nil)),
        word_op("adjoin-arg", |args| {
            let (word, argl) = two("adjoin-arg", args)?;
            adjoin_arg(word, &argl)
        }),
        // -- procedure application --
        word_op("primitive-procedure?", |args| {
            Ok(Value::boolean(
                primitive_name(&one("primitive-procedure?", args)?.0).is_some(),
            ))
        }),
        word_op("compound-procedure?", |args| {
            Ok(Value::boolean(
                compound_parts(&one("compound-procedure?", args)?.0).is_some(),
            ))
        }),
        word_op("apply-primitive-procedure", |args| {
            let (proc, argl) = two("apply-primitive-procedure", args)?;
            let name = primitive_name(&proc)
                .ok_or_else(|| op_fail("apply-primitive-procedure needs a primitive procedure"))?
                .to_owned();
            let values = argl
                .list_items()
                .map_err(|_| op_fail("apply-primitive-procedure needs an operand list"))?;
            apply_object_primitive(&name, &values)
        }),
        word_op("make-procedure", |args| {
            let (params, body, base) = three("make-procedure", args)?;
            lookup_environment(&base, "make-procedure")?;
            items_of(&params).ok_or_else(|| op_fail("make-procedure needs a parameter list"))?;
            items_of(&body).ok_or_else(|| op_fail("make-procedure needs a body"))?;
            Ok(compound_word(params, body, base))
        }),
        word_op("procedure-parameters", |args| {
            Ok(compound_parts(&one("compound-procedure?", args)?.0)
                .ok_or_else(|| op_fail("procedure-parameters needs a compound procedure"))?
                .0)
        }),
        word_op("procedure-body", |args| {
            Ok(compound_parts(&one("compound-procedure?", args)?.0)
                .ok_or_else(|| op_fail("procedure-body needs a compound procedure"))?
                .1)
        }),
        word_op("procedure-environment", |args| {
            Ok(compound_parts(&one("compound-procedure?", args)?.0)
                .ok_or_else(|| op_fail("procedure-environment needs a compound procedure"))?
                .2)
        }),
        // -- environments --
        word_op("extend-environment", |args| {
            let (params, argl, base) = three("extend-environment", args)?;
            let base = lookup_environment(&base, "extend-environment")?;
            let names: Vec<String> = items_of(&params)
                .ok_or_else(|| op_fail("extend-environment needs a parameter list"))?
                .iter()
                .map(|word| {
                    symbol_name(word).ok_or_else(|| op_fail("a parameter is written as a variable"))
                })
                .collect::<Result<_, _>>()?;
            let values = items_of(&argl)
                .ok_or_else(|| op_fail("extend-environment needs an operand list"))?;
            if names.len() != values.len() {
                return Err(op_fail(format!(
                    "extend-environment: the procedure wants {} arguments, got {}",
                    names.len(),
                    values.len()
                )));
            }
            let frame = Env::child(&base);
            for (name, value) in names.into_iter().zip(values) {
                frame.define(name.into(), value);
            }
            Ok(intern_environment(frame))
        }),
        word_op("lookup-variable-value", |args| {
            let (word, base) = two("lookup-variable-value", args)?;
            let name = variable_arg(std::slice::from_ref(&word), 0, "lookup-variable-value")?;
            let base = lookup_environment(&base, "lookup-variable-value")?;
            base.lookup(&name)
                .map_err(|_| op_fail(format!("unbound variable: {name}")))
        }),
        word_action("set-variable-value!", |args| {
            let name = variable_arg(args, 0, "set-variable-value!")?;
            let value = args
                .get(1)
                .cloned()
                .ok_or_else(|| op_fail("set-variable-value!: missing a value"))?;
            let base = environment_arg(args, 2, "set-variable-value!")?;
            base.set(&name, value)
                .map_err(|_| op_fail(format!("unbound variable -- set!: {name}")))
        }),
        word_action("define-variable!", |args| {
            let name = variable_arg(args, 0, "define-variable!")?;
            let value = args
                .get(1)
                .cloned()
                .ok_or_else(|| op_fail("define-variable!: missing a value"))?;
            let base = environment_arg(args, 2, "define-variable!")?;
            base.define(name.into(), value);
            Ok(())
        }),
        // -- conditionals and sequences --
        word_op("true?", |args| {
            Ok(Value::boolean(object_is_true(&one("true?", args)?.0)))
        }),
        selector("if-predicate", "if", 1, "if-predicate needs an if"),
        selector("if-consequent", "if", 2, "if-consequent needs an if"),
        word_op("if-alternative", |args| {
            let items = tagged_items(&one("if?", args)?.0, "if")
                .ok_or_else(|| op_fail("if-alternative needs an if"))?;
            Ok(items.get(3).cloned().unwrap_or_else(|| Value::sym("false")))
        }),
        word_op("begin-actions", |args| {
            let items = tagged_items(&one("begin?", args)?.0, "begin")
                .ok_or_else(|| op_fail("begin-actions needs a begin"))?;
            Ok(Value::list(items.into_iter().skip(1).collect()))
        }),
        word_op("first-exp", |args| {
            let items = items_of(&one("first-exp", args)?.0)
                .ok_or_else(|| op_fail("first-exp needs a sequence"))?;
            items
                .first()
                .cloned()
                .ok_or_else(|| op_fail("first-exp of an empty sequence"))
        }),
        word_op("rest-exps", |args| {
            let items = items_of(&one("rest-exps", args)?.0)
                .ok_or_else(|| op_fail("rest-exps needs a sequence"))?;
            Ok(Value::list(items.into_iter().skip(1).collect()))
        }),
        word_op("last-exp?", |args| {
            let (_, last) = sequence_flags("last-exp?", &one("last-exp?", args)?.0)?;
            Ok(Value::boolean(last))
        }),
        word_op("no-more-exps?", |args| {
            let (empty, _) = sequence_flags("no-more-exps?", &one("no-more-exps?", args)?.0)?;
            Ok(Value::boolean(empty))
        }),
        // -- assignments and definitions --
        selector(
            "assignment-variable",
            "set!",
            1,
            "assignment-variable needs an assignment",
        ),
        selector(
            "assignment-value",
            "set!",
            2,
            "assignment-value needs an assignment",
        ),
        word_op("definition-variable", |args| {
            let items = tagged_items(&one("definition-variable", args)?.0, "define")
                .ok_or_else(|| op_fail("definition-variable needs a definition"))?;
            let target = items.get(1).cloned().unwrap_or(Value::Nil);
            let Value::Pair(_) = target else {
                return Ok(target);
            };
            items_of(&target)
                .and_then(|inner| inner.first().cloned())
                .ok_or_else(|| op_fail("definition-variable needs a name"))
        }),
        word_op("definition-value", |args| {
            let items = tagged_items(&one("definition-value", args)?.0, "define")
                .ok_or_else(|| op_fail("definition-value needs a definition"))?;
            let target = items.get(1).cloned().unwrap_or(Value::Nil);
            let body: Vec<Value> = items.into_iter().skip(2).collect();
            let Value::Pair(_) = target else {
                return body
                    .into_iter()
                    .next()
                    .ok_or_else(|| op_fail("definition-value needs a value"));
            };
            let inner = items_of(&target)
                .ok_or_else(|| op_fail("definition-value needs a procedure form"))?;
            let parameters = inner
                .get(1..)
                .map(|slice| Value::list(slice.to_vec()))
                .ok_or_else(|| op_fail("definition-value needs parameters"))?;
            Ok(lambda_form(parameters, &body))
        }),
    ]
}

// ---------------------------------------------------------------------------
// The controller text, in the book's fragments
// ---------------------------------------------------------------------------

/// The controller fragments of the base evaluator, in printed order:
/// each pair is the fragment's name and its controller text, the
/// book's text in the book's notation. The base controller is their
/// concatenation; an exercise replaces a fragment with
/// [`compose_controller`] or splices new entries in with
/// [`splice_controller`] and hands the composed text to
/// [`make_evaluator`].
/// The fragments are the book's controller text verbatim; a shorter
/// listing would no longer be the book's controller.
#[expect(
    clippy::too_many_lines,
    reason = "the fragments are the book's controller text verbatim"
)]
#[must_use]
pub fn controller_fragments() -> &'static [(&'static str, &'static str)] {
    &[
        (
            "driver",
            "read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input)
           (const \";;; EC-Eval input:\"))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op announce-output)
           (const \";;; EC-Eval value:\"))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))",
        ),
        (
            "eval-dispatch",
            "eval-dispatch
  (test (op self-evaluating?) (reg exp))
  (branch (label ev-self-eval))
  (test (op variable?) (reg exp))
  (branch (label ev-variable))
  (test (op quoted?) (reg exp))
  (branch (label ev-quoted))
  (test (op assignment?) (reg exp))
  (branch (label ev-assignment))
  (test (op definition?) (reg exp))
  (branch (label ev-definition))
  (test (op if?) (reg exp))
  (branch (label ev-if))
  (test (op lambda?) (reg exp))
  (branch (label ev-lambda))
  (test (op begin?) (reg exp))
  (branch (label ev-begin))
  (test (op application?) (reg exp))
  (branch (label ev-application))
  (goto (label unknown-expression-type))",
        ),
        (
            "ev-self-eval",
            "ev-self-eval
  (assign val (reg exp))
  (goto (reg continue))",
        ),
        (
            "ev-variable",
            "ev-variable
  (assign val
          (op lookup-variable-value)
          (reg exp)
          (reg env))
  (goto (reg continue))",
        ),
        (
            "ev-quoted",
            "ev-quoted
  (assign val
          (op text-of-quotation)
          (reg exp))
  (goto (reg continue))",
        ),
        (
            "ev-lambda",
            "ev-lambda
  (assign unev
          (op lambda-parameters)
          (reg exp))
  (assign exp
          (op lambda-body)
          (reg exp))
  (assign val
          (op make-procedure)
          (reg unev)
          (reg exp)
          (reg env))
  (goto (reg continue))",
        ),
        (
            "ev-application",
            "ev-application
  (save continue)
  (save env)
  (assign unev (op operands) (reg exp))
  (save unev)
  (assign exp (op operator) (reg exp))
  (assign
   continue (label ev-appl-did-operator))
  (goto (label eval-dispatch))",
        ),
        (
            "ev-appl-did-operator",
            "ev-appl-did-operator
  (restore unev)
  (restore env)
  (assign argl (op empty-arglist))
  (assign proc (reg val))
  (test (op no-operands?) (reg unev))
  (branch (label apply-dispatch))
  (save proc)",
        ),
        (
            "argument-loop",
            "ev-appl-operand-loop
  (save argl)
  (assign exp
          (op first-operand)
          (reg unev))
  (test (op last-operand?) (reg unev))
  (branch (label ev-appl-last-arg))
  (save env)
  (save unev)
  (assign continue
          (label ev-appl-accumulate-arg))
  (goto (label eval-dispatch))
ev-appl-accumulate-arg
  (restore unev)
  (restore env)
  (restore argl)
  (assign argl
          (op adjoin-arg)
          (reg val)
          (reg argl))
  (assign unev
          (op rest-operands)
          (reg unev))
  (goto (label ev-appl-operand-loop))
ev-appl-last-arg
  (assign continue
          (label ev-appl-accum-last-arg))
  (goto (label eval-dispatch))
ev-appl-accum-last-arg
  (restore argl)
  (assign argl
          (op adjoin-arg)
          (reg val)
          (reg argl))
  (restore proc)
  (goto (label apply-dispatch))",
        ),
        (
            "apply-dispatch",
            "apply-dispatch
  (test (op primitive-procedure?) (reg proc))
  (branch (label primitive-apply))
  (test (op compound-procedure?) (reg proc))
  (branch (label compound-apply))
  (goto (label unknown-procedure-type))",
        ),
        (
            "primitive-apply",
            "primitive-apply
  (assign val (op apply-primitive-procedure)
              (reg proc)
              (reg argl))
  (restore continue)
  (goto (reg continue))",
        ),
        (
            "compound-apply",
            "compound-apply
  (assign unev
          (op procedure-parameters)
          (reg proc))
  (assign env
          (op procedure-environment)
          (reg proc))
  (assign env
          (op extend-environment)
          (reg unev)
          (reg argl)
          (reg env))
  (assign unev
          (op procedure-body)
          (reg proc))
  (goto (label ev-sequence))",
        ),
        (
            "begin",
            "ev-begin
  (assign unev
          (op begin-actions)
          (reg exp))
  (save continue)
  (goto (label ev-sequence))",
        ),
        (
            "ev-sequence",
            "ev-sequence
  (assign exp (op first-exp) (reg unev))
  (test (op last-exp?) (reg unev))
  (branch (label ev-sequence-last-exp))
  (save unev)
  (save env)
  (assign continue
          (label ev-sequence-continue))
  (goto (label eval-dispatch))
ev-sequence-continue
  (restore env)
  (restore unev)
  (assign unev
          (op rest-exps)
          (reg unev))
  (goto (label ev-sequence))
ev-sequence-last-exp
  (restore continue)
  (goto (label eval-dispatch))",
        ),
        (
            "if",
            "ev-if
  (save exp)
  (save env)
  (save continue)
  (assign continue (label ev-if-decide))
  (assign exp (op if-predicate) (reg exp))
  (goto (label eval-dispatch))
ev-if-decide
  (restore continue)
  (restore env)
  (restore exp)
  (test (op true?) (reg val))
  (branch (label ev-if-consequent))
ev-if-alternative
  (assign exp (op if-alternative) (reg exp))
  (goto (label eval-dispatch))
ev-if-consequent
  (assign exp (op if-consequent) (reg exp))
  (goto (label eval-dispatch))",
        ),
        (
            "assignment",
            "ev-assignment
  (assign unev
          (op assignment-variable)
          (reg exp))
  (save unev)
  (assign exp
          (op assignment-value)
          (reg exp))
  (save env)
  (save continue)
  (assign continue
          (label ev-assignment-1))
  (goto (label eval-dispatch))
ev-assignment-1
  (restore continue)
  (restore env)
  (restore unev)
  (perform
   (op set-variable-value!)
   (reg unev)
   (reg val)
   (reg env))
  (assign val
          (const ok))
  (goto (reg continue))",
        ),
        (
            "definition",
            "ev-definition
  (assign unev
          (op definition-variable)
          (reg exp))
  (save unev)
  (assign exp
          (op definition-value)
          (reg exp))
  (save env)
  (save continue)
  (assign continue (label ev-definition-1))
  (goto (label eval-dispatch))
ev-definition-1
  (restore continue)
  (restore env)
  (restore unev)
  (perform
   (op define-variable!)
   (reg unev)
   (reg val)
   (reg env))
  (assign val (const ok))
  (goto (reg continue))",
        ),
        (
            "errors",
            "unknown-expression-type
  (assign val (const unknown-expression-type-error))
  (goto (label signal-error))
unknown-procedure-type
  (restore continue)
  (assign val (const unknown-procedure-type-error))
  (goto (label signal-error))
signal-error
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))",
        ),
    ]
}

/// The base controller: the fragments in printed order.
#[must_use]
pub fn base_controller() -> String {
    controller_fragments()
        .iter()
        .map(|(_, text)| *text)
        .collect::<Vec<_>>()
        .join("\n")
}

/// The base controller with whole fragments replaced by name: the
/// composition the monitored driver of 5.4.4 and the exercises'
/// variants use.
#[must_use]
pub fn compose_controller(replacements: &[(&str, &str)]) -> String {
    controller_fragments()
        .iter()
        .map(|(name, text)| {
            replacements
                .iter()
                .find(|(replace, _)| replace == name)
                .map_or(*text, |(_, replacement)| *replacement)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Splices new controller text into a composed controller: the
/// dispatch tests go ahead of the application test, which a
/// pair-shaped derived form or basic `cond` would otherwise swallow,
/// and the new entry points go ahead of the error entries. Exercise
/// 5.23's derived forms and 5.24's basic cond compose this way.
#[must_use]
pub fn splice_controller(controller: &str, dispatch_tests: &str, entries: &str) -> String {
    let application_test = "  (test (op application?) (reg exp))";
    let (before, after) = controller
        .split_once(application_test)
        .unwrap_or((controller, ""));
    let with_tests = format!("{before}{dispatch_tests}\n{application_test}{after}");
    let errors_label = "unknown-expression-type\n";
    let (before, after) = with_tests
        .split_once(errors_label)
        .unwrap_or((with_tests.as_str(), ""));
    format!("{before}{entries}\n{errors_label}{after}")
}

// ---------------------------------------------------------------------------
// Building and running the evaluator
// ---------------------------------------------------------------------------

/// The section's evaluator: the assembled machine over the book's
/// controller plus the transcript the driver operations write.
pub struct Evaluator {
    machine: Machine,
    output: Rc<RefCell<Vec<String>>>,
}

/// One operation over the machine itself: the shape the driver's
/// stack-statistics op needs.
fn machine_op(
    name: &'static str,
    f: impl Fn(&mut Machine, &[Value]) -> Result<Value, Fault> + 'static,
) -> (&'static str, OpHandler) {
    let handler: OpHandler = Rc::new(move |machine, args| f(machine, args));
    (name, handler)
}

fn driver_operations(
    output: &Rc<RefCell<Vec<String>>>,
    input: &Rc<RefCell<VecDeque<Value>>>,
    global: &Value,
) -> Vec<(&'static str, OpHandler)> {
    let global = global.clone();
    let get_global = word_op("get-global-environment", move |_| Ok(global.clone()));
    let prompt = announce_op("prompt-for-input", output, display_value);
    let announce_output = announce_op("announce-output", output, display_value);
    let user_print = announce_op("user-print", output, render_word);
    let read = word_op("read", {
        let input = Rc::clone(input);
        move |_| {
            input
                .borrow_mut()
                .pop_front()
                .ok_or_else(|| op_fail(INPUT_EXHAUSTED))
        }
    });
    // The simulator installs its own print-stack-statistics into the
    // machine transcript; the evaluator's version prints into the
    // driver transcript, so one transcript holds a session.
    let statistics = machine_op("print-stack-statistics", {
        let output = Rc::clone(output);
        move |machine, _| {
            let (pushes, depth) = machine.stack_statistics();
            let line = format!("(total-pushes = {pushes} maximum-depth = {depth})");
            announce(&output, line);
            Ok(Value::sym("done"))
        }
    });
    vec![
        get_global,
        prompt,
        announce_output,
        read,
        user_print,
        statistics,
    ]
}

/// One operation that announces one word: the prompt, the value
/// banner, and the printed value of the book's driver loop.
fn announce_op(
    name: &'static str,
    output: &Rc<RefCell<Vec<String>>>,
    render: impl Fn(&Value) -> String + 'static,
) -> (&'static str, OpHandler) {
    let output = Rc::clone(output);
    word_op(name, move |args| {
        announce(&output, render(&first_arg(args)));
        Ok(Value::sym("done"))
    })
}

fn first_arg(args: &[Value]) -> Value {
    args.first().cloned().unwrap_or(Value::Nil)
}

fn announce(output: &RefCell<Vec<String>>, line: String) {
    output.borrow_mut().push(line);
}

/// The object primitives of the global environment, the book's
/// eceval list trimmed to the subset the section's sessions use.
const OBJECT_PRIMITIVES: &[&str] = &[
    "cons",
    "car",
    "cdr",
    "null?",
    "pair?",
    "symbol?",
    "number?",
    "string?",
    "not",
    "eq?",
    "equal?",
    "list",
    "+",
    "-",
    "*",
    "/",
    "=",
    "<",
    ">",
    "remainder",
];

/// Builds the section's evaluator: the controller text (the book's,
/// or a composed exercise variant) is assembled by the 5.2 simulator,
/// the operations table is [`base_operations`], then the driver
/// operations, then the extra `operations` last so they override on a
/// name collision, and the object program `source` is read into the
/// input queue. The global environment is the book's setup: `true`,
/// `false`, and the object-language primitives.
///
/// # Errors
/// [`Fault::Parse`] when the controller or the source is unreadable,
/// plus the assembly faults of [`Fault`] when an operation is unknown
/// or an instruction shape is wrong.
pub fn make_evaluator(
    controller: &str,
    operations: &[(&'static str, OpHandler)],
    source: &str,
) -> Result<Evaluator, Fault> {
    let forms = read_program(source).map_err(|error| Fault::Parse(error.to_string()))?;
    let output = Rc::new(RefCell::new(Vec::new()));
    let input = Rc::new(RefCell::new(VecDeque::from(forms)));
    let global_env = Env::global();
    global_env.define("true".into(), Value::boolean(true));
    global_env.define("false".into(), Value::boolean(false));
    for name in OBJECT_PRIMITIVES {
        global_env.define((*name).into(), primitive_word(name));
    }
    let global = intern_environment(global_env);
    let mut table: Vec<(&'static str, OpHandler)> = base_operations();
    table.extend(driver_operations(&output, &input, &global));
    table.extend(operations.iter().cloned());
    let machine = make_machine(EVALUATOR_REGISTERS, &table, controller)?;
    Ok(Evaluator { machine, output })
}

impl Evaluator {
    /// The machine, for the monitoring extensions the exercises use:
    /// the instruction budget, the trace, and the breakpoints.
    #[must_use]
    pub fn machine(&self) -> &Machine {
        &self.machine
    }

    /// The machine, mutable.
    pub fn machine_mut(&mut self) -> &mut Machine {
        &mut self.machine
    }

    /// Runs the evaluator until the input queue runs dry, the book's
    /// read-eval-print loop.
    ///
    /// # Errors
    /// Any fault of the machine except the queue-dry read, which is
    /// the run's normal end.
    pub fn run(&mut self) -> Result<(), Fault> {
        match self.machine.start() {
            Ok(_) => Ok(()),
            Err(Fault::Op { op, message, .. }) if op == "read" && message == INPUT_EXHAUSTED => {
                Ok(())
            }
            Err(fault) => Err(fault),
        }
    }

    /// The lines the driver printed: the prompts, the stack
    /// statistics of a monitored driver, and the values, in order.
    #[must_use]
    pub fn transcript(&self) -> Vec<String> {
        self.output.borrow().clone()
    }

    /// The machine's stack counters `(total-pushes, maximum-depth)`.
    #[must_use]
    pub fn stack_statistics(&self) -> (u64, u64) {
        self.machine.stack_statistics()
    }

    /// Reads a register's contents.
    ///
    /// # Errors
    /// [`Fault::UnknownRegister`] when the machine has no such
    /// register.
    pub fn get_register(&self, name: &str) -> Result<Value, Fault> {
        self.machine.get_register(name)
    }

    /// The instruction count of the run so far.
    #[must_use]
    pub fn instruction_count(&self) -> u64 {
        self.machine.instruction_count()
    }
}

/// Builds the base evaluator over `source`, runs it to the end of the
/// queue, and answers the driver's transcript.
///
/// # Errors
/// [`Fault::Parse`] when the source is unreadable, and any fault of
/// the run except the queue-dry read.
pub fn run_session(source: &str) -> Result<Vec<String>, Fault> {
    let mut evaluator = make_evaluator(&base_controller(), &[], source)?;
    evaluator.run()?;
    Ok(evaluator.transcript())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The monitored driver of 5.4.4: the statistics printed before
    /// the value.
    fn monitored_controller() -> String {
        let monitored = "read-eval-print-loop
  (perform (op initialize-stack))
  (perform (op prompt-for-input)
           (const \";;; EC-Eval input:\"))
  (assign exp (op read))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (label eval-dispatch))
print-result
  (perform (op print-stack-statistics))
  (perform (op announce-output)
           (const \";;; EC-Eval value:\"))
  (perform (op user-print) (reg val))
  (goto (label read-eval-print-loop))";
        compose_controller(&[("driver", monitored)])
    }

    /// The value of the last interaction: the line before the
    /// trailing prompt the driver prints when the queue runs dry.
    fn last_value(transcript: &[String]) -> &str {
        let at = transcript.len().saturating_sub(2);
        transcript[at].as_str()
    }

    /// The `(total-pushes ...)` lines of a monitored session.
    fn stats_lines(transcript: &[String]) -> Vec<String> {
        transcript
            .iter()
            .filter(|line| line.starts_with("(total-pushes"))
            .cloned()
            .collect()
    }

    /// Runs `source` on the monitored driver and answers the stats
    /// of the last interaction.
    fn measured(source: &str) -> (u64, u64, String) {
        let mut evaluator =
            make_evaluator(&monitored_controller(), &[], source).expect("assembles");
        evaluator.run().expect("runs");
        let transcript = evaluator.transcript();
        let stats = stats_lines(&transcript);
        let line = stats.last().map(String::as_str).unwrap_or_default();
        let (pushes, depth) = parse_stats(line);
        (pushes, depth, last_value(&transcript).to_owned())
    }

    /// Reads the two counters out of a stats line.
    fn parse_stats(line: &str) -> (u64, u64) {
        let integer =
            |text: &str| -> u64 { text.trim().trim_end_matches(')').parse().unwrap_or(0) };
        let (_, rest) = line.split_once("(total-pushes = ").unwrap_or(("", line));
        let (pushes, rest) = rest.split_once(' ').unwrap_or((rest, ""));
        let depth = rest
            .split_once("maximum-depth = ")
            .map_or("0", |(_, depth)| depth);
        (integer(pushes), integer(depth))
    }

    #[test]
    fn quotes_and_variables_and_arithmetic() {
        let lines = run_session("'foo\n(* 6 7)\n(cons 1 (cons 2 '()))").expect("runs");
        assert_eq!(lines[0], ";;; EC-Eval input:");
        assert_eq!(last_value(&lines), "(1 2)");
        assert!(
            lines.contains(&"foo".to_owned()),
            "the quoted symbol: {lines:?}"
        );
        assert!(lines.contains(&"42".to_owned()), "the product: {lines:?}");
    }

    #[test]
    fn defines_and_sets() {
        let lines = run_session(
            "(define (twice n) (* 2 n))\n(twice 21)\n(define x 5)\n(set! x (twice x))\nx",
        )
        .expect("runs");
        assert!(
            lines.contains(&"ok".to_owned()),
            "the define answers: {lines:?}"
        );
        assert_eq!(last_value(&lines), "10", "x reads (twice 5) after the set!");
    }

    #[test]
    fn begin_and_if_compose() {
        let lines = run_session("(begin 1 2 (if (< 1 2) (quote yes) (quote no)))").expect("runs");
        assert_eq!(last_value(&lines), "yes");
    }

    #[test]
    fn unknown_expression_errors_reach_the_driver_loop() {
        // The controller's error entries route through signal-error
        // and back to the driver loop; the next read ends the run.
        let lines = run_session("7").expect("runs");
        assert_eq!(last_value(&lines), "7");
    }

    #[test]
    fn primitive_failures_stop_the_machine() {
        let result = run_session("(car 5)");
        let Err(fault) = result else {
            panic!("car of 5 must fault")
        };
        assert!(fault.to_string().contains("car: not a pair: 5"), "{fault}");
    }

    #[test]
    fn unbound_variables_stop_the_base_machine() {
        // Exercise 5.30's work: the base evaluator only catches the
        // unknown expression and procedure types, so the lookup fault
        // takes the run out of the evaluator.
        let result = run_session("no-such-variable");
        let Err(fault) = result else {
            panic!("an unbound variable must fault")
        };
        assert!(
            fault
                .to_string()
                .contains("unbound variable: no-such-variable"),
            "{fault}"
        );
    }

    #[test]
    fn the_monitored_session_matches_the_book() {
        let mut evaluator = make_evaluator(
            &monitored_controller(),
            &[],
            "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))\n(factorial 5)",
        )
        .expect("assembles");
        evaluator.run().expect("runs");
        let transcript = evaluator.transcript();
        assert_eq!(
            stats_lines(&transcript),
            vec![
                "(total-pushes = 3 maximum-depth = 3)".to_owned(),
                "(total-pushes = 144 maximum-depth = 28)".to_owned(),
            ]
        );
        assert_eq!(last_value(&transcript), "120");
    }

    #[test]
    fn the_recursive_factorial_fits_the_book_formulas() {
        for (n, pushes, depth) in [(1, 16, 8), (2, 48, 13), (3, 80, 18), (5, 144, 28)] {
            let source = format!(
                "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))\n(factorial {n})"
            );
            let (measured_pushes, measured_depth, value) = measured(&source);
            assert_eq!(value, ((1..=n).product::<i128>()).to_string(), "n = {n}");
            assert_eq!(
                (measured_pushes, measured_depth),
                (pushes, depth),
                "n = {n}"
            );
        }
    }

    #[test]
    fn tail_recursion_keeps_the_depth_constant() {
        let source = "(define (factorial n) (define (iter product counter) (if (> counter n) product (iter (* counter product) (+ counter 1)))) (iter 1 1))\n(factorial 6)";
        let (pushes, depth, value) = measured(source);
        assert_eq!(value, "720");
        assert_eq!(depth, 10, "the maximum depth is independent of n");
        assert_eq!(pushes, 239, "the pushes fit 35n + 29 at n = 6");
    }
}
