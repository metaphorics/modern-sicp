// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 5.5

//! Section 5.5: Compilation.
//!
//! The book's [`compile`] and its code generators: they turn
//! object-language expressions (the list structure the reader
//! produced) into instruction sequences whose statements are
//! controller lines in the book's notation. Every sequence carries
//! its needed and modified register sets, so
//! [`preserving_instruction_sequences`] never reads code. The label
//! orders and compile orders inside the generators are the ones the
//! book's own figures show (after-lambda before entry, alternative
//! before consequent before predicate, after-call before
//! compiled-branch before primitive-branch), so the emitted label
//! numbering matches the book's; the module tests check the compiled
//! factorial against Figure 5.17 and the seeded compilation of
//! exercise 5.35 against Figure 5.18.
//!
//! One spelling deviates from the book's listings, forced by the
//! section 5.2 machine language (exercise 5.9 forbids labels as
//! operation inputs): the book's `(label entry2)` operation input of
//! `make-compiled-procedure` is spelled `(const entry2)`. List
//! constants need no table here, because the 5.2 machine's `(const
//! (n))` spellings build list values directly.
//!
//! The 5.5.7 machine ([`make_compiled_evaluator`],
//! [`compile_and_go`]) runs compiled code beside interpreted code:
//! the 5.4 evaluator's controller with the compiled apply-dispatch,
//! the armed external entry, and the `arg1`/`arg2` registers of
//! exercise 5.38.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

use crate::sec_5_2::{Fault, Machine, OpHandler, make_machine};
use crate::sec_5_4::{
    INPUT_EXHAUSTED, apply_object_primitive, environment_of, intern_env, object_is_true,
    primitive_name, primitive_word, render_word,
};
use sicp_runtime::{
    Env, Pair, Value, cons_cell, display_value, print_value, read_program, set_car, set_cdr,
};

/// The reserved tag prefix of the evaluator's own words is shared with
/// 5.4; compiled procedure words carry this tag.
const COMPILED_TAG: &str = "sicp-word: compiled-procedure";

/// The message whose read fault ends a driver session, 5.4's stop.
pub const INPUT_QUEUE_EMPTY: &str = INPUT_EXHAUSTED;

fn op_fail(message: impl Into<String>) -> Fault {
    Fault::Op {
        op: String::new(),
        message: message.into(),
        step: 0,
    }
}

// ---------------------------------------------------------------------------
// Instruction sequences
// ---------------------------------------------------------------------------

/// One instruction sequence: the registers the code needs, the
/// registers it modifies, and the statements, one controller line
/// each, in the book's three-part shape.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Seq {
    /// The registers that must be initialized before the code runs.
    pub needs: Vec<String>,
    /// The registers the code's instructions modify.
    pub modifies: Vec<String>,
    /// The statements: controller lines in the book's notation.
    pub stmts: Vec<String>,
}

/// The book's `make-instruction-sequence`.
#[must_use]
pub fn make_instruction_sequence(needs: &[&str], modifies: &[&str], stmts: Vec<String>) -> Seq {
    Seq {
        needs: needs.iter().map(|name| (*name).to_owned()).collect(),
        modifies: modifies.iter().map(|name| (*name).to_owned()).collect(),
        stmts,
    }
}

/// The book's `empty-instruction-sequence`.
#[must_use]
pub fn empty_instruction_sequence() -> Seq {
    Seq::default()
}

/// The book's `list-union`, preserving the first list's order.
fn list_union(s1: &[String], s2: &[String]) -> Vec<String> {
    let mut out: Vec<String> = s1.to_vec();
    for name in s2 {
        if !out.contains(name) {
            out.push(name.clone());
        }
    }
    out
}

/// The book's `list-difference`, preserving the first list's order.
fn list_difference(s1: &[String], s2: &[String]) -> Vec<String> {
    s1.iter()
        .filter(|name| !s2.contains(name))
        .cloned()
        .collect()
}

/// The book's `append-2-sequences`: the needs of the first plus the
/// needs of the second that the first does not modify.
#[must_use]
pub fn append_2_sequences(seq1: &Seq, seq2: &Seq) -> Seq {
    Seq {
        needs: list_union(&seq1.needs, &list_difference(&seq2.needs, &seq1.modifies)),
        modifies: list_union(&seq1.modifies, &seq2.modifies),
        stmts: seq1
            .stmts
            .iter()
            .chain(seq2.stmts.iter())
            .cloned()
            .collect(),
    }
}

/// The book's `append-instruction-sequences` over a list.
#[must_use]
pub fn append_sequences(seqs: &[Seq]) -> Seq {
    let mut acc = empty_instruction_sequence();
    for seq in seqs {
        acc = append_2_sequences(&acc, seq);
    }
    acc
}

/// The book's `tack-on-instruction-sequence`: the body's register use
/// is ignored, because the body is not executed in line.
#[must_use]
pub fn tack_on_instruction_sequence(seq: &Seq, body_seq: &Seq) -> Seq {
    Seq {
        needs: seq.needs.clone(),
        modifies: seq.modifies.clone(),
        stmts: seq
            .stmts
            .iter()
            .chain(body_seq.stmts.iter())
            .cloned()
            .collect(),
    }
}

/// The book's `parallel-instruction-sequences`: the two branches after
/// a test are never executed sequentially, so the combined sequence
/// modifies what either branch modifies.
#[must_use]
pub fn parallel_instruction_sequences(seq1: &Seq, seq2: &Seq) -> Seq {
    Seq {
        needs: list_union(&seq1.needs, &seq2.needs),
        modifies: list_union(&seq1.modifies, &seq2.modifies),
        stmts: seq1
            .stmts
            .iter()
            .chain(seq2.stmts.iter())
            .cloned()
            .collect(),
    }
}

/// The book's `preserving`: appends with a `save`/`restore` around the
/// first sequence of every register the first modifies and the second
/// needs. Walking the register list in order nests the wraps, so the
/// first register of the set is saved last, the book's own order.
///
/// With [`Config::preserving_on`] off (the 5.37 comparison) every
/// register in the set is saved unconditionally.
#[must_use]
pub fn preserving_instruction_sequences(
    cfg: &Config,
    regs: &[&str],
    seq1: &Seq,
    seq2: &Seq,
) -> Seq {
    let mut current = seq1.clone();
    for reg in regs {
        let needed = seq2.needs.iter().any(|name| name == reg)
            && current.modifies.iter().any(|name| name == reg);
        let saves = if cfg.preserving_on { needed } else { true };
        if saves {
            let mut stmts = vec![format!("(save {reg})")];
            stmts.extend(current.stmts.iter().cloned());
            stmts.push(format!("(restore {reg})"));
            current = Seq {
                needs: list_union(&[(*reg).to_owned()], &current.needs),
                modifies: list_difference(&current.modifies, &[(*reg).to_owned()]),
                stmts,
            };
        }
    }
    append_2_sequences(&current, seq2)
}

// ---------------------------------------------------------------------------
// The compiler state
// ---------------------------------------------------------------------------

/// The compiler's state: the book's label counter, shared by clone so
/// a running machine's operation can compile under the same numbering.
#[derive(Clone, Default)]
pub struct State {
    counter: Cell<usize>,
    entries: Cell<usize>,
}

/// The book's fresh label counter.
#[must_use]
pub fn new_state() -> State {
    State::default()
}

/// A label counter seeded at `n`: exercise 5.35's reproduction of
/// Figure 5.18 seeds 14, the labels the book's session had already
/// generated.
#[must_use]
pub fn new_state_seeded(n: usize) -> State {
    State {
        counter: Cell::new(n),
        entries: Cell::new(0),
    }
}

/// The book's `make-label`: the name suffixed with the next counter
/// value, so successive labels read `after-lambda1`, `after-lambda15`,
/// and so on, as the book's figures show.
pub fn make_label(state: &State, name: &str) -> String {
    let next = state.counter.get() + 1;
    state.counter.set(next);
    format!("{name}{next}")
}

/// Counts one compiled block: the entry names of 5.48's recorded
/// blocks and 5.49's chained forms stay distinct.
pub fn bump_entry(state: &State) -> usize {
    let next = state.entries.get() + 1;
    state.entries.set(next);
    next
}

// ---------------------------------------------------------------------------
// Targets, linkages, compile-time environments
// ---------------------------------------------------------------------------

/// The book's linkage descriptor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Linkage {
    /// Continue at the next instruction in sequence.
    Next,
    /// Return from the procedure being compiled.
    Return,
    /// Jump to the named entry point.
    Lab(String),
}

/// The compile-time environment: the frames of parameter names,
/// newest first; the empty vector is the top level.
pub type Cenv = Vec<Vec<String>>;
/// The callback used by the compiler's variable-reference tracing option.
pub type TraceFn = Rc<dyn Fn(&Cenv, &str)>;

/// The empty compile-time environment: the top level.
#[must_use]
pub fn top_cenv() -> Cenv {
    Vec::new()
}

/// Extends the compile-time environment with one parameter frame.
#[must_use]
pub fn extend_cenv(params: &[String], frames: &Cenv) -> Cenv {
    let mut out = Vec::with_capacity(frames.len() + 1);
    out.push(params.to_vec());
    out.extend(frames.iter().cloned());
    out
}

/// The book's `find-variable` result: the lexical address as the pair
/// of frame number and displacement, or not-found past every frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LexicalAddress {
    /// The variable's `(frame displacement)` address.
    Found(usize, usize),
    /// The variable is in no frame of the compile-time environment.
    NotFound,
}

/// The book's `find-variable`: the lexical address of `name` with
/// respect to the compile-time environment, or not-found.
#[must_use]
pub fn find_variable(name: &str, frames: &Cenv) -> LexicalAddress {
    for (frame_number, names) in frames.iter().enumerate() {
        for (displacement, candidate) in names.iter().enumerate() {
            if candidate == name {
                return LexicalAddress::Found(frame_number, displacement);
            }
        }
    }
    LexicalAddress::NotFound
}

/// The configuration the section's exercises turn: lexical addressing
/// (5.40 to 5.42), scanning out internal definitions (5.43),
/// open-coded primitives (5.38 and 5.44), the operand evaluation
/// order (5.36), the preserving mechanism itself (5.37), and compiled
/// calls to interpreted procedures (5.47).
#[expect(
    clippy::struct_excessive_bools,
    reason = "The booleans are independent compiler switches for distinct section exercises."
)]
#[derive(Clone, Default)]
pub struct Config {
    /// 5.40 to 5.42: emit lexical-address accesses.
    pub lexical: bool,
    /// 5.43: scan internal definitions out of bodies.
    pub scan_out: bool,
    /// 5.38 and 5.44: open-code the named primitives.
    pub open_code: bool,
    /// 5.36: evaluate operands left to right.
    pub left_to_right: bool,
    /// 5.37: the preserving mechanism itself; off saves blindly.
    pub preserving_on: bool,
    /// 5.47: compiled code may call interpreted procedures.
    pub compound_calls: bool,
    /// 5.40: every variable reference reports the compile-time
    /// environment it was compiled against.
    pub trace: Option<TraceFn>,
}

/// The configuration of the section's main text: every extension off,
/// the preserving mechanism on.
#[must_use]
pub fn default_config() -> Config {
    Config {
        preserving_on: true,
        ..Config::default()
    }
}

/// The primitives the open-coding dispatch of 5.38 recognizes.
#[must_use]
pub fn open_coded_primitives() -> &'static [&'static str] {
    &["+", "-", "*", "<", "="]
}

// ---------------------------------------------------------------------------
// Syntax over the object language's list structure
// ---------------------------------------------------------------------------

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

fn is_tagged(word: &Value, tag: &str) -> bool {
    tagged_items(word, tag).is_some()
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

fn is_self_evaluating(word: &Value) -> bool {
    matches!(
        word,
        Value::Int(_) | Value::Real(_) | Value::Str(_) | Value::Bool(_)
    )
}

fn operand_items(word: &Value) -> Vec<Value> {
    items_of(word).unwrap_or_default()
}

/// The parameters of a lambda word as names.
fn parameter_names(word: &Value) -> Result<Vec<String>, Fault> {
    operand_items(word)
        .iter()
        .map(symbol_name)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| op_fail("compile-lambda: a parameter is not a symbol"))
}

/// Builds the lambda a procedure-form define names:
/// `(lambda parameters body...)`.
fn lambda_form(parameters: Value, body: &[Value]) -> Value {
    let mut form = vec![Value::sym("lambda"), parameters];
    form.extend(body.iter().cloned());
    Value::list(form)
}

/// Renders a constant for a `(const ...)` spelling: the value form,
/// whose strings are quoted, so the reader round-trips the datum.
fn const_spelling(word: &Value) -> String {
    print_value(word)
}

// ---------------------------------------------------------------------------
// Derived expressions
// ---------------------------------------------------------------------------

/// The book's `cond->if` (4.1.2): the clauses become nested `if`s; a
/// clause with no actions answers its test, and a cond with no else
/// falls through to false.
///
/// # Errors
///
/// Returns a fault when `exp` is not a `cond` expression.
pub fn cond_to_if(exp: &Value) -> Result<Value, Fault> {
    let items = tagged_items(exp, "cond").ok_or_else(|| op_fail("cond->if needs a cond"))?;
    let mut result = Value::Bool(false);
    for clause in items[1..].iter().rev() {
        let parts = items_of(clause).unwrap_or_default();
        let Some(test) = parts.first() else {
            continue;
        };
        let actions = parts.get(1..).unwrap_or(&[]);
        let consequent = match actions {
            [] => test.clone(),
            [single] => single.clone(),
            many => Value::list(many.to_vec()),
        };
        let is_else = matches!(test, Value::Sym(name) if name.as_ref() == "else");
        if is_else {
            result = consequent;
        } else {
            result = Value::list(vec![Value::sym("if"), test.clone(), consequent, result]);
        }
    }
    Ok(result)
}

/// The 4.1.6 `let`-to-combination transformation: `(let ((n e) ...)
/// body ...)` is the call of a lambda on the inits.
///
/// # Errors
///
/// Returns a fault when `exp` is not a `let` expression or contains a malformed binding.
pub fn let_to_combination(exp: &Value) -> Result<Value, Fault> {
    let items = tagged_items(exp, "let").ok_or_else(|| op_fail("let->combination needs a let"))?;
    let bindings = operand_items(items.get(1).unwrap_or(&Value::Nil));
    let body = items.get(2..).unwrap_or(&[]).to_vec();
    let mut params = Vec::new();
    let mut inits = Vec::new();
    for binding in &bindings {
        let pair = operand_items(binding);
        if pair.len() != 2 {
            return Err(op_fail("let->combination: bad binding"));
        }
        params.push(pair[0].clone());
        inits.push(pair[1].clone());
    }
    let lambda = lambda_form(Value::list(params), &body);
    let mut call = vec![lambda];
    call.extend(inits);
    Ok(Value::list(call))
}

// ---------------------------------------------------------------------------
// The code generators
// ---------------------------------------------------------------------------

/// The registers a compiled procedure call may disturb.
const ALL_REGS: [&str; 5] = ["env", "proc", "val", "argl", "continue"];

/// The book's `compile-linkage`.
#[must_use]
pub fn compile_linkage(linkage: &Linkage) -> Seq {
    match linkage {
        Linkage::Return => {
            make_instruction_sequence(&["continue"], &[], vec!["(goto (reg continue))".to_owned()])
        }
        Linkage::Next => empty_instruction_sequence(),
        Linkage::Lab(label) => {
            make_instruction_sequence(&[], &[], vec![format!("(goto (label {label}))")])
        }
    }
}

/// The book's `end-with-linkage`.
#[must_use]
pub fn end_with_linkage(cfg: &Config, linkage: &Linkage, seq: &Seq) -> Seq {
    preserving_instruction_sequences(cfg, &["continue"], seq, &compile_linkage(linkage))
}

/// The book's `compile`, the top-level dispatch. `cenv` is the
/// compile-time environment (empty at the top level), `target` the
/// register the code answers in, and `linkage` the descriptor for how
/// the code proceeds.
///
/// # Errors
///
/// Returns a fault when `exp` is malformed or uses an unsupported expression.
pub fn compile(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    exp: &Value,
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    if is_self_evaluating(exp) {
        return Ok(compile_self_evaluating(cfg, exp, target, linkage));
    }
    if is_tagged(exp, "quote") {
        return compile_quoted(cfg, exp, target, linkage);
    }
    if is_symbol(exp) {
        return compile_variable(cfg, cenv, exp, target, linkage);
    }
    if is_tagged(exp, "set!") {
        return compile_assignment(cfg, state, cenv, exp, target, linkage);
    }
    if is_tagged(exp, "define") {
        return compile_definition(cfg, state, cenv, exp, target, linkage);
    }
    if is_tagged(exp, "if") {
        return compile_if(cfg, state, cenv, exp, target, linkage);
    }
    if is_tagged(exp, "lambda") {
        return compile_lambda(cfg, state, cenv, exp, target, linkage);
    }
    if is_tagged(exp, "begin") {
        let items = tagged_items(exp, "begin").unwrap_or_default();
        return compile_sequence(cfg, state, cenv, &items[1..], target, linkage);
    }
    if is_tagged(exp, "cond") {
        let transformed = cond_to_if(exp)?;
        return compile(cfg, state, cenv, &transformed, target, linkage);
    }
    if is_tagged(exp, "let") {
        let transformed = let_to_combination(exp)?;
        return compile(cfg, state, cenv, &transformed, target, linkage);
    }
    if let Value::Pair(_) = exp {
        let operator = items_of(exp)
            .and_then(|items| items.first().cloned())
            .unwrap_or(Value::Nil);
        if is_open_coded(cfg, cenv, &operator) {
            return compile_open_code(cfg, state, cenv, exp, target, linkage);
        }
        return compile_application(cfg, state, cenv, exp, target, linkage);
    }
    Err(op_fail(format!(
        "Unknown expression type: COMPILE: {}",
        display_value(exp)
    )))
}

/// 5.38 and 5.44: the operator is an open-coded primitive name only
/// when the configuration open-codes, the name is one of the
/// open-coded set, and no compile-time frame binds the name.
fn is_open_coded(cfg: &Config, cenv: &Cenv, operator: &Value) -> bool {
    let Some(name) = symbol_name(operator) else {
        return false;
    };
    cfg.open_code
        && open_coded_primitives().contains(&name.as_str())
        && find_variable(&name, cenv) == LexicalAddress::NotFound
}

fn compile_self_evaluating(cfg: &Config, exp: &Value, target: &str, linkage: &Linkage) -> Seq {
    end_with_linkage(
        cfg,
        linkage,
        &make_instruction_sequence(
            &[],
            &[target],
            vec![format!("(assign {target} (const {}))", const_spelling(exp))],
        ),
    )
}

fn compile_quoted(
    cfg: &Config,
    exp: &Value,
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let items = tagged_items(exp, "quote").unwrap_or_default();
    let Some(datum) = items.get(1) else {
        return Err(op_fail("compile-quoted needs a datum"));
    };
    // The quote prefix marks the spelling as a datum, so a quoted
    // datum that is itself a `quote` form round-trips through the
    // reader unchanged.
    Ok(end_with_linkage(
        cfg,
        linkage,
        &make_instruction_sequence(
            &[],
            &[target],
            vec![format!(
                "(assign {target} (const '{}))",
                const_spelling(datum)
            )],
        ),
    ))
}

fn compile_variable(
    cfg: &Config,
    cenv: &Cenv,
    exp: &Value,
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let Some(name) = symbol_name(exp) else {
        return Err(op_fail("compile-variable needs a variable"));
    };
    if let Some(trace) = &cfg.trace {
        trace(cenv, &name);
    }
    let access = if cfg.lexical {
        match find_variable(&name, cenv) {
            LexicalAddress::Found(frame, displacement) => vec![format!(
                "(assign {target} (op lexical-address-lookup) (const {frame}) (const {displacement}) (reg env))"
            )],
            LexicalAddress::NotFound => vec![format!(
                "(assign {target} (op lookup-variable-value) (const {name}) (reg env))"
            )],
        }
    } else {
        vec![format!(
            "(assign {target} (op lookup-variable-value) (const {name}) (reg env))"
        )]
    };
    Ok(end_with_linkage(
        cfg,
        linkage,
        &make_instruction_sequence(&["env"], &[target], access),
    ))
}

fn compile_assignment(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    exp: &Value,
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let items = tagged_items(exp, "set!").unwrap_or_default();
    let Some(name) = items.get(1).and_then(symbol_name) else {
        return Err(op_fail("compile-assignment needs a variable"));
    };
    let Some(value) = items.get(2) else {
        return Err(op_fail("compile-assignment needs a value"));
    };
    let value_code = compile(cfg, state, cenv, value, "val", &Linkage::Next)?;
    let assign_stmt = if cfg.lexical {
        match find_variable(&name, cenv) {
            LexicalAddress::Found(frame, displacement) => format!(
                "(perform (op lexical-address-set!) (const {frame}) (const {displacement}) (reg val) (reg env))"
            ),
            LexicalAddress::NotFound => {
                format!("(perform (op set-variable-value!) (const {name}) (reg val) (reg env))")
            }
        }
    } else {
        format!("(perform (op set-variable-value!) (const {name}) (reg val) (reg env))")
    };
    let tail = make_instruction_sequence(
        &["env", "val"],
        &[target],
        vec![assign_stmt, format!("(assign {target} (const ok))")],
    );
    Ok(end_with_linkage(
        cfg,
        linkage,
        &preserving_instruction_sequences(cfg, &["env"], &value_code, &tail),
    ))
}

/// The `(name, value)` a define names, with a procedure-form define
/// rewritten to its lambda.
fn definition_parts(exp: &Value) -> Result<(String, Value), Fault> {
    let items = tagged_items(exp, "define").unwrap_or_default();
    let Some(target) = items.get(1) else {
        return Err(op_fail("compile-definition needs a target"));
    };
    match target {
        Value::Sym(name) => {
            let value = items
                .get(2)
                .ok_or_else(|| op_fail("compile-definition needs a value"))?;
            Ok((name.to_string(), value.clone()))
        }
        Value::Pair(_) => {
            let signature = operand_items(target);
            let name = signature
                .first()
                .and_then(symbol_name)
                .ok_or_else(|| op_fail("compile-definition needs a name"))?;
            let parameters = signature
                .get(1..)
                .map_or(Value::Nil, |rest| Value::list(rest.to_vec()));
            let body = items.get(2..).unwrap_or(&[]).to_vec();
            Ok((name, lambda_form(parameters, &body)))
        }
        other => Err(op_fail(format!(
            "compile-definition: bad target: {}",
            display_value(other)
        ))),
    }
}

fn compile_definition(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    exp: &Value,
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let (name, value) = definition_parts(exp)?;
    let value_code = compile(cfg, state, cenv, &value, "val", &Linkage::Next)?;
    let tail = make_instruction_sequence(
        &["env"],
        &[target],
        vec![
            format!("(perform (op define-variable!) (const {name}) (reg val) (reg env))"),
            format!("(assign {target} (const ok))"),
        ],
    );
    Ok(end_with_linkage(
        cfg,
        linkage,
        &preserving_instruction_sequences(cfg, &["env"], &value_code, &tail),
    ))
}

fn compile_if(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    exp: &Value,
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let items = tagged_items(exp, "if").unwrap_or_default();
    let Some(predicate) = items.get(1) else {
        return Err(op_fail("compile-if needs a predicate"));
    };
    let Some(consequent) = items.get(2) else {
        return Err(op_fail("compile-if needs a consequent"));
    };
    // The label allocations and the compilation order (alternative,
    // consequent, predicate) are the orders the book's own figures
    // show.
    let after_if = make_label(state, "after-if");
    let f_branch = make_label(state, "false-branch");
    let t_branch = make_label(state, "true-branch");
    let consequent_linkage = if *linkage == Linkage::Next {
        Linkage::Lab(after_if.clone())
    } else {
        linkage.clone()
    };
    let alternative_exp = items.get(3).cloned().unwrap_or(Value::Bool(false));
    let a_code = compile(cfg, state, cenv, &alternative_exp, target, linkage)?;
    let c_code = compile(cfg, state, cenv, consequent, target, &consequent_linkage)?;
    let p_code = compile(cfg, state, cenv, predicate, "val", &Linkage::Next)?;
    let test_code = make_instruction_sequence(
        &["val"],
        &[],
        vec![
            "(test (op false?) (reg val))".to_owned(),
            format!("(branch (label {f_branch}))"),
        ],
    );
    let true_side = append_2_sequences(
        &make_instruction_sequence(&[], &[], vec![t_branch]),
        &c_code,
    );
    let false_side = append_2_sequences(
        &make_instruction_sequence(&[], &[], vec![f_branch]),
        &a_code,
    );
    let mut branches = parallel_instruction_sequences(&true_side, &false_side);
    branches.stmts.push(after_if);
    let with_test = append_2_sequences(&test_code, &branches);
    Ok(preserving_instruction_sequences(
        cfg,
        &["env", "continue"],
        &p_code,
        &with_test,
    ))
}

/// 5.43: the internal defines of a body become a `let` of
/// `*unassigned*` bindings whose values are `set!` after it, the
/// book's scan-out; a body with no defines is returned unchanged.
#[must_use]
pub fn scan_out_defines(body: &[Value]) -> Vec<Value> {
    let has_defines = body.iter().any(|form| is_tagged(form, "define"));
    if !has_defines {
        return body.to_vec();
    }
    let mut names = Vec::new();
    let mut sets = Vec::new();
    for form in body {
        if !is_tagged(form, "define") {
            continue;
        }
        if let Ok((name, value)) = definition_parts(form) {
            names.push(name.clone());
            sets.push(Value::list(vec![
                Value::sym("set!"),
                Value::sym(&name),
                value,
            ]));
        }
    }
    let bindings = Value::list(
        names
            .iter()
            .map(|name| {
                Value::list(vec![
                    Value::sym(name),
                    Value::list(vec![Value::sym("quote"), Value::sym("*unassigned*")]),
                ])
            })
            .collect(),
    );
    let mut let_form = vec![Value::sym("let"), bindings];
    let_form.extend(sets);
    let_form.extend(
        body.iter()
            .filter(|form| !is_tagged(form, "define"))
            .cloned(),
    );
    vec![Value::list(let_form)]
}

fn compile_sequence(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    seq: &[Value],
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let Some((first, rest)) = seq.split_first() else {
        return Err(op_fail("compile-sequence needs a sequence"));
    };
    if rest.is_empty() {
        return compile(cfg, state, cenv, first, target, linkage);
    }
    let first_code = compile(cfg, state, cenv, first, target, &Linkage::Next)?;
    let rest_code = compile_sequence(cfg, state, cenv, rest, target, linkage)?;
    Ok(preserving_instruction_sequences(
        cfg,
        &["env", "continue"],
        &first_code,
        &rest_code,
    ))
}

fn compile_lambda(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    exp: &Value,
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let items = tagged_items(exp, "lambda").unwrap_or_default();
    if items.len() < 2 {
        return Err(op_fail("compile-lambda needs parameters"));
    }
    // after-lambda is allocated before entry, the order the book's
    // figures show.
    let after_lambda = make_label(state, "after-lambda");
    let proc_entry = make_label(state, "entry");
    let lambda_linkage = if *linkage == Linkage::Next {
        Linkage::Lab(after_lambda.clone())
    } else {
        linkage.clone()
    };
    let construct = end_with_linkage(
        cfg,
        &lambda_linkage,
        &make_instruction_sequence(
            &["env"],
            &[target],
            vec![format!(
                "(assign {target} (op make-compiled-procedure) (const {proc_entry}) (reg env))"
            )],
        ),
    );
    let body = compile_lambda_body(cfg, state, cenv, exp, &proc_entry)?;
    let mut combined = tack_on_instruction_sequence(&construct, &body);
    combined.stmts.push(after_lambda);
    Ok(combined)
}

fn compile_lambda_body(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    exp: &Value,
    proc_entry: &str,
) -> Result<Seq, Fault> {
    let items = tagged_items(exp, "lambda").unwrap_or_default();
    let Some(parameters) = items.get(1) else {
        return Err(op_fail("compile-lambda-body needs parameters"));
    };
    let names = parameter_names(parameters)?;
    let body = items.get(2..).unwrap_or(&[]).to_vec();
    let body = if cfg.scan_out {
        scan_out_defines(&body)
    } else {
        body
    };
    let head = make_instruction_sequence(
        &["env", "proc", "argl"],
        &["env"],
        vec![
            proc_entry.to_owned(),
            "(assign env (op compiled-procedure-env) (reg proc))".to_owned(),
            format!(
                "(assign env (op extend-environment) (const ({})) (reg argl) (reg env))",
                names.join(" ")
            ),
        ],
    );
    let body_code = compile_sequence(
        cfg,
        state,
        &extend_cenv(&names, cenv),
        &body,
        "val",
        &Linkage::Return,
    )?;
    Ok(append_2_sequences(&head, &body_code))
}

fn compile_application(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    exp: &Value,
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let items = items_of(exp).ok_or_else(|| op_fail("compile-application needs a combination"))?;
    let operator = items
        .first()
        .ok_or_else(|| op_fail("compile-application needs an operator"))?;
    let operands = items.get(1..).unwrap_or(&[]);
    let proc_code = compile(cfg, state, cenv, operator, "proc", &Linkage::Next)?;
    let mut operand_codes = Vec::with_capacity(operands.len());
    for operand in operands {
        operand_codes.push(compile(cfg, state, cenv, operand, "val", &Linkage::Next)?);
    }
    let arglist_code = construct_arglist(cfg, &operand_codes);
    let call = compile_procedure_call(cfg, state, target, linkage)?;
    let inner = preserving_instruction_sequences(cfg, &["proc", "continue"], &arglist_code, &call);
    Ok(preserving_instruction_sequences(
        cfg,
        &["env", "continue"],
        &proc_code,
        &inner,
    ))
}

/// The book's `construct-arglist`: the default evaluates the operands
/// right to left, the last operand initializing `argl` and each
/// earlier operand consing onto it. The 5.36 `left_to_right`
/// configuration evaluates first to last, adjoining each argument at
/// the end, so the argument list keeps the source order either way and
/// the instruction count is unaffected by the choice.
#[must_use]
pub fn construct_arglist(cfg: &Config, operand_codes: &[Seq]) -> Seq {
    let mut ordered: Vec<Seq> = operand_codes.to_vec();
    if !cfg.left_to_right {
        ordered.reverse();
    }
    if ordered.is_empty() {
        return make_instruction_sequence(
            &[],
            &["argl"],
            vec!["(assign argl (const ()))".to_owned()],
        );
    }
    let cons_op = if cfg.left_to_right {
        "(assign argl (op adjoin-arg) (reg val) (reg argl))"
    } else {
        "(assign argl (op cons) (reg val) (reg argl))"
    };
    let code_to_get_last_arg = append_2_sequences(
        &ordered[0],
        &make_instruction_sequence(
            &["val"],
            &["argl"],
            vec!["(assign argl (op list) (reg val))".to_owned()],
        ),
    );
    if ordered.len() == 1 {
        return code_to_get_last_arg;
    }
    let cons_step =
        make_instruction_sequence(&["val", "argl"], &["argl"], vec![cons_op.to_owned()]);
    let mut rest_code = empty_instruction_sequence();
    for operand_code in ordered[1..].iter().rev() {
        let code_for_next_arg =
            preserving_instruction_sequences(cfg, &["argl"], operand_code, &cons_step);
        rest_code = preserving_instruction_sequences(cfg, &["env"], &code_for_next_arg, &rest_code);
    }
    preserving_instruction_sequences(cfg, &["env"], &code_to_get_last_arg, &rest_code)
}

fn compile_procedure_call(
    cfg: &Config,
    state: &State,
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    // after-call is allocated before compiled-branch before
    // primitive-branch, the order the book's figures show.
    let after_call = make_label(state, "after-call");
    let compiled_branch = make_label(state, "compiled-branch");
    let primitive_branch = make_label(state, "primitive-branch");
    let compound_branch = if cfg.compound_calls {
        Some(make_label(state, "compound-branch"))
    } else {
        None
    };
    let compiled_linkage = if *linkage == Linkage::Next {
        Linkage::Lab(after_call.clone())
    } else {
        linkage.clone()
    };
    let appl_code = compile_proc_appl(state, target, &compiled_linkage)?;
    // compound-apply answers in `val`; a call compiled into another
    // register needs the copy the compiled branch's proc-return
    // performs, so the interpreted branch lands on compound-return
    // first.
    let compound_return = if compound_branch.is_some() && target != "val" {
        Some(make_label(state, "compound-return"))
    } else {
        None
    };
    let primitive_tail = if compound_branch.is_some() && *linkage == Linkage::Next {
        vec![format!("(goto (label {after_call}))")]
    } else {
        Vec::new()
    };
    let mut primitive_stmts = vec![format!(
        "(assign {target} (op apply-primitive-procedure) (reg proc) (reg argl))"
    )];
    primitive_stmts.extend(primitive_tail);
    let primitive_code = end_with_linkage(
        cfg,
        linkage,
        &make_instruction_sequence(&["proc", "argl"], &[target], primitive_stmts),
    );
    let (compound_test, compound_label, compound_return_block) = match compound_branch {
        Some(branch) => {
            compile_compound_branch(branch, target, linkage, &after_call, compound_return)
        }
        None => (
            empty_instruction_sequence(),
            empty_instruction_sequence(),
            empty_instruction_sequence(),
        ),
    };
    let dispatched = parallel_instruction_sequences(
        &append_2_sequences(
            &make_instruction_sequence(&[], &[], vec![compiled_branch]),
            &appl_code,
        ),
        &append_2_sequences(
            &make_instruction_sequence(&[], &[], vec![primitive_branch.clone()]),
            &primitive_code,
        ),
    );
    let test = make_instruction_sequence(
        &["proc"],
        &[],
        vec![
            "(test (op primitive-procedure?) (reg proc))".to_owned(),
            format!("(branch (label {primitive_branch}))"),
        ],
    );
    let after = make_instruction_sequence(&[], &[], vec![after_call]);
    Ok(append_sequences(&[
        test,
        compound_test,
        dispatched,
        compound_label,
        compound_return_block,
        after,
    ]))
}

/// 5.47: the third branch of a procedure call. It tests `proc` and,
/// for an interpreted procedure, hands the call to the evaluator's
/// compound-apply through the `unev` register, whose value is dead at
/// a call site (the machine's register set is the 5.4 machine's, so
/// the book's `compapp` register has no name here). The branch saves
/// `continue` on the stack before jumping: the interpreted
/// compound-apply reaches its body through ev-sequence, whose
/// last-expression path restores `continue` from the stack, the
/// interpreted calling convention. compound-apply answers in `val`
/// and jumps through `continue`; a call compiled into a non-`val`
/// target first lands on its compound-return label, where `val` is
/// copied into the target exactly as the compiled branch's
/// proc-return does. Returns the test, the branch body, and the
/// copy block, in emission order.
fn compile_compound_branch(
    branch: String,
    target: &str,
    linkage: &Linkage,
    after_call: &str,
    compound_return: Option<String>,
) -> (Seq, Seq, Seq) {
    let compound_test = make_instruction_sequence(
        &["proc"],
        &[],
        vec![
            "(test (op compound-procedure?) (reg proc))".to_owned(),
            format!("(branch (label {branch}))"),
        ],
    );
    let cont_label = compound_return.as_deref();
    let mut cont_setup = vec![branch];
    cont_setup.extend(match linkage {
        Linkage::Return => Vec::new(),
        Linkage::Next => vec![format!(
            "(assign continue (label {}))",
            cont_label.unwrap_or(after_call)
        )],
        Linkage::Lab(label) => vec![format!(
            "(assign continue (label {}))",
            cont_label.unwrap_or(label)
        )],
    });
    cont_setup.extend([
        "(save continue)".to_owned(),
        "(assign unev (label compound-apply))".to_owned(),
        "(goto (reg unev))".to_owned(),
    ]);
    let compound_label = make_instruction_sequence(&["proc"], &["unev", "continue"], cont_setup);
    let compound_return_block = match compound_return {
        Some(label) => compile_compound_return(target, linkage, after_call, label),
        None => empty_instruction_sequence(),
    };
    (compound_test, compound_label, compound_return_block)
}

/// The block a compound call returns to when the call was compiled
/// into a non-`val` target: `val` is copied into the target exactly
/// as the compiled branch's proc-return does, then control goes to
/// the join point. Return linkage never reaches a non-val target
/// (compile-proc-appl faults first), so the fallthrough here is
/// after-call, the Next linkage's join point.
fn compile_compound_return(
    target: &str,
    linkage: &Linkage,
    after_call: &str,
    label: String,
) -> Seq {
    let exit = match linkage {
        Linkage::Lab(destination) => destination.clone(),
        _ => after_call.to_owned(),
    };
    make_instruction_sequence(
        &["val"],
        &[target],
        vec![
            label,
            format!("(assign {target} (reg val))"),
            format!("(goto (label {exit}))"),
        ],
    )
}

fn compile_proc_appl(state: &State, target: &str, linkage: &Linkage) -> Result<Seq, Fault> {
    let all: Vec<&str> = ALL_REGS.to_vec();
    let entry = "(assign val (op compiled-procedure-entry) (reg proc))";
    let goto = "(goto (reg val))";
    match linkage {
        Linkage::Return if target == "val" => Ok(make_instruction_sequence(
            &["proc", "continue"],
            &all,
            vec![entry.to_owned(), goto.to_owned()],
        )),
        Linkage::Return => Err(op_fail("return linkage, target not val: COMPILE")),
        Linkage::Lab(label) if target == "val" => Ok(make_instruction_sequence(
            &["proc"],
            &all,
            vec![
                format!("(assign continue (label {label}))"),
                entry.to_owned(),
                goto.to_owned(),
            ],
        )),
        Linkage::Lab(label) => {
            let proc_return = make_label(state, "proc-return");
            Ok(make_instruction_sequence(
                &["proc"],
                &all,
                vec![
                    format!("(assign continue (label {proc_return}))"),
                    entry.to_owned(),
                    goto.to_owned(),
                    proc_return,
                    format!("(assign {target} (reg val))"),
                    format!("(goto (label {label}))"),
                ],
            ))
        }
        Linkage::Next => Err(op_fail(
            "compile-proc-appl: the call carries no next linkage",
        )),
    }
}

// ---------------------------------------------------------------------------
// 5.38: open-coded primitives
// ---------------------------------------------------------------------------

/// 5.38(a): the operands are evaluated into successive argument
/// registers, with the registers still to come preserved around each
/// evaluation, because an operand may itself be an open-coded call;
/// the environment is preserved with them, because a nested call
/// operand rebinds it and a later operand (a variable reference) reads
/// the caller's frame.
fn spread_arguments(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    operands: &[Value],
    targets: &[&str],
) -> Result<Seq, Fault> {
    let Some((operand, rest)) = operands.split_first() else {
        return Ok(empty_instruction_sequence());
    };
    let Some((target, rest_targets)) = targets.split_first() else {
        return Err(op_fail("spread-arguments ran out of argument registers"));
    };
    let code = compile(cfg, state, cenv, operand, target, &Linkage::Next)?;
    if rest.is_empty() {
        return Ok(code);
    }
    let rest_code = spread_arguments(cfg, state, cenv, rest, rest_targets)?;
    let mut preserve: Vec<&str> = rest_targets.to_vec();
    preserve.push("env");
    Ok(preserving_instruction_sequences(
        cfg, &preserve, &code, &rest_code,
    ))
}

fn compile_open_code(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    exp: &Value,
    target: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let items = items_of(exp).ok_or_else(|| op_fail("open coding needs a combination"))?;
    let name = items
        .first()
        .and_then(symbol_name)
        .ok_or_else(|| op_fail("open coding needs a primitive name"))?;
    let operands = items.get(1..).unwrap_or(&[]);
    if operands.len() > 2 && (name == "+" || name == "*") {
        return compile_open_code_nary(cfg, state, cenv, &name, operands, linkage);
    }
    if operands.len() != 2 {
        return Err(op_fail(format!(
            "open coding needs two operands for {name}"
        )));
    }
    let spread = spread_arguments(cfg, state, cenv, operands, &["arg1", "arg2"])?;
    let apply = make_instruction_sequence(
        &["arg1", "arg2"],
        &[target],
        vec![format!(
            "(assign {target} (op {name}) (reg arg1) (reg arg2))"
        )],
    );
    Ok(end_with_linkage(
        cfg,
        linkage,
        &append_2_sequences(&spread, &apply),
    ))
}

/// 5.38(d): more than two operands fold through one register: each
/// operand is evaluated into `arg1` and folded into `val`. The
/// environment is preserved around an evaluation whose tail reads it
/// (a later operand may be a call that rebinds `env`); `arg1` itself
/// is never preserved around its own evaluation, it is the
/// evaluation's output.
fn compile_open_code_nary(
    cfg: &Config,
    state: &State,
    cenv: &Cenv,
    name: &str,
    operands: &[Value],
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let Some(first) = operands.first() else {
        return Err(op_fail("open coding needs operands"));
    };
    let Some(second) = operands.get(1) else {
        return Err(op_fail("open coding needs operands"));
    };
    let c1 = compile(cfg, state, cenv, first, "arg1", &Linkage::Next)?;
    let c2 = compile(cfg, state, cenv, second, "arg2", &Linkage::Next)?;
    // The second operand's evaluation may clobber `arg1` internally
    // (an open-coded operand), so the first operand's result is
    // shielded across it.
    let c2_shielded = if c2.modifies.iter().any(|reg| reg == "arg1") {
        let mut stmts = vec!["(save arg1)".to_owned()];
        stmts.extend(c2.stmts.iter().cloned());
        stmts.push("(restore arg1)".to_owned());
        Seq {
            needs: list_union(&["arg1".to_owned()], &c2.needs),
            modifies: c2.modifies.clone(),
            stmts,
        }
    } else {
        c2
    };
    let fold_first = make_instruction_sequence(
        &["arg1", "arg2"],
        &["val"],
        vec![format!("(assign val (op {name}) (reg arg1) (reg arg2))")],
    );
    let open_step = make_instruction_sequence(
        &["arg1", "val"],
        &["val"],
        vec![format!("(assign val (op {name}) (reg arg1) (reg val))")],
    );
    let mut rest_code = empty_instruction_sequence();
    for operand in operands[2..].iter().rev() {
        let code = compile(cfg, state, cenv, operand, "arg1", &Linkage::Next)?;
        rest_code = preserving_instruction_sequences(
            cfg,
            &["env"],
            &code,
            &append_2_sequences(&open_step, &rest_code),
        );
    }
    // The first two operands preserve `env` the way the fold loop
    // does: an earlier operand that is a call rebinds `env`, and any
    // later operand reading a variable must see the caller's frame.
    let after_second = preserving_instruction_sequences(
        cfg,
        &["env"],
        &c2_shielded,
        &append_2_sequences(&fold_first, &rest_code),
    );
    let first_two = preserving_instruction_sequences(cfg, &["env"], &c1, &after_second);
    Ok(end_with_linkage(cfg, linkage, &first_two))
}

// ---------------------------------------------------------------------------
// Whole programs
// ---------------------------------------------------------------------------

/// Compiles the forms of one program: every form but the last
/// continues to the next, and the last carries the requested linkage;
/// the forms append with `env` and `continue` preserved.
///
/// # Errors
///
/// Returns a fault when `forms` is empty or when a form cannot be compiled.
pub fn compile_forms(
    cfg: &Config,
    state: &State,
    forms: &[Value],
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let mut acc: Option<Seq> = None;
    let last = forms.len().saturating_sub(1);
    for (index, form) in forms.iter().enumerate() {
        let form_linkage = if index == last {
            linkage
        } else {
            &Linkage::Next
        };
        let code = compile(cfg, state, &top_cenv(), form, "val", form_linkage)?;
        acc = Some(match acc {
            None => code,
            Some(previous) => {
                preserving_instruction_sequences(cfg, &["env", "continue"], &previous, &code)
            }
        });
    }
    acc.ok_or_else(|| op_fail("compile-forms needs a program"))
}

/// Reads `source` and compiles it as one program.
///
/// # Errors
/// [`Fault::Parse`] when the source is unreadable, plus the compiler's
/// faults.
pub fn compile_program(
    cfg: &Config,
    state: &State,
    source: &str,
    linkage: &Linkage,
) -> Result<Seq, Fault> {
    let forms = read_program(source).map_err(|error| Fault::Parse(error.to_string()))?;
    compile_forms(cfg, state, &forms, linkage)
}

/// The statements of a sequence, one controller line each.
#[must_use]
pub fn statements_text(seq: &Seq) -> String {
    seq.stmts.join("\n")
}

/// Compiles the forms of `source` under a fresh entry label: the shape
/// `compile_and_go`, 5.48's recorded blocks, and 5.49's chained forms
/// share. The entry label counts its own sequence, so two
/// `compile_block` calls on one state never collide.
///
/// # Errors
/// [`Fault::Parse`] when the source is unreadable, plus the compiler's
/// faults.
pub fn compile_block(cfg: &Config, state: &State, source: &str) -> Result<(String, String), Fault> {
    let seq = compile_program(cfg, state, source, &Linkage::Return)?;
    let entry = format!("compiled-entry-{}", bump_entry(state));
    Ok((entry.clone(), format!("{entry}\n{}", statements_text(&seq))))
}

// ---------------------------------------------------------------------------
// Compiled procedure words
// ---------------------------------------------------------------------------

/// The compiled procedure word: the entry label and the environment,
/// the book's `make-compiled-procedure` product.
#[must_use]
pub fn compiled_procedure_word(entry: Value, environment: Value) -> Value {
    Value::tagged(COMPILED_TAG, Value::list(vec![entry, environment]))
}

/// The `(entry, environment)` a compiled procedure word carries.
#[must_use]
pub fn compiled_procedure_parts(word: &Value) -> Option<(Value, Value)> {
    match word {
        Value::Tagged { tag, data } if tag.as_ref() == COMPILED_TAG => {
            let items = data.list_items().ok()?;
            Some((items.first()?.clone(), items.get(1)?.clone()))
        }
        _ => None,
    }
}

/// Renders a word on the compiled machine's transcript: compiled
/// procedures print as the book's footnote prints them, everything
/// else as 5.4 renders.
#[must_use]
pub fn render_compiled_word(word: &Value) -> String {
    if compiled_procedure_parts(word).is_some() {
        return "#[compiled-procedure]".to_owned();
    }
    render_word(word)
}

// ---------------------------------------------------------------------------
// The runtime primitive table
// ---------------------------------------------------------------------------

/// One runtime primitive: the shape `apply-primitive-procedure` and
/// the object-language `apply` reach.
pub type RuntimeFn = Rc<dyn Fn(&[Value]) -> Result<Value, Fault>>;

fn number_of(name: &str, word: &Value) -> Result<i128, Fault> {
    match word {
        Value::Int(n) => Ok(*n),
        other => Err(op_fail(format!(
            "{name}: needs a number, got {}",
            display_value(other)
        ))),
    }
}

fn listed(word: &Value, name: &str) -> Result<Vec<Value>, Fault> {
    word.list_items()
        .map_err(|_| op_fail(format!("{name}: needs a list")))
}

fn cadr_of(name: &str, word: &Value) -> Result<Value, Fault> {
    listed(word, name)?
        .get(1)
        .cloned()
        .ok_or_else(|| op_fail(format!("{name}: list too short")))
}

fn caddr_of(name: &str, word: &Value) -> Result<Value, Fault> {
    listed(word, name)?
        .get(2)
        .cloned()
        .ok_or_else(|| op_fail(format!("{name}: list too short")))
}

fn cadddr_of(name: &str, word: &Value) -> Result<Value, Fault> {
    listed(word, name)?
        .get(3)
        .cloned()
        .ok_or_else(|| op_fail(format!("{name}: list too short")))
}

/// The marker symbol whose presence names the empty environment in the
/// compiled metacircular's `setup-environment`.
#[must_use]
pub fn the_empty_environment_marker() -> Value {
    Value::sym("the-empty")
}

/// The runtime primitive names the machine binds in its global
/// environment: the 5.4 object set plus the names the compiled
/// metacircular calls.
#[must_use]
pub fn runtime_names() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = crate::sec_5_4::object_primitive_names().to_vec();
    names.extend([
        "cadr",
        "caddr",
        "cadddr",
        "cddr",
        "cdddr",
        "caadr",
        "cdadr",
        "quotient",
        "abs",
        "<=",
        ">=",
        "display",
        "newline",
        "error",
        "extend-environment",
        "lookup-variable-value",
        "set-variable-value!",
        "define-variable!",
        "apply-in-underlying-scheme",
    ]);
    names
}

/// Applies the runtime primitive `name` to values: the compiled
/// machine's own primitive table, the 5.4 object set plus the names
/// the compiled metacircular calls, then any `extra` entries last.
/// `display` writes into `output` when one is given.
///
/// # Errors
/// [`Fault::Op`] carrying the primitive's own message.
pub fn apply_runtime_primitive(
    output: Option<&Rc<RefCell<Vec<String>>>>,
    extra: &[(String, RuntimeFn)],
    name: &str,
    args: &[Value],
) -> Result<Value, Fault> {
    if let Some((_, f)) = extra.iter().find(|(known, _)| known == name) {
        return f(args);
    }
    let first = || args.first().cloned().unwrap_or(Value::Nil);
    match name {
        "cadr" => cadr_of("cadr", &first()),
        "caddr" => caddr_of("caddr", &first()),
        "cadddr" => cadddr_of("cadddr", &first()),
        "caadr" => listed(&cadr_of("caadr", &first())?, "caadr")?
            .first()
            .cloned()
            .ok_or_else(|| op_fail("caadr: list too short")),
        "cddr" => Ok(Value::list(
            listed(&first(), "cddr")?.into_iter().skip(2).collect(),
        )),
        "cdddr" => Ok(Value::list(
            listed(&first(), "cdddr")?.into_iter().skip(3).collect(),
        )),
        "cdadr" => Ok(Value::list(
            listed(&cadr_of("cdadr", &first())?, "cdadr")?
                .into_iter()
                .skip(1)
                .collect(),
        )),
        "quotient" => {
            let a = number_of("quotient", &first())?;
            let b = number_of("quotient", &args.get(1).cloned().unwrap_or(Value::Nil))?;
            if b == 0 {
                return Err(op_fail("division by zero"));
            }
            Ok(Value::Int(a / b))
        }
        "abs" => Ok(Value::Int(number_of("abs", &first())?.abs())),
        "<=" => Ok(Value::boolean(
            number_of("<=", &first())?
                <= number_of("<=", &args.get(1).cloned().unwrap_or(Value::Nil))?,
        )),
        ">=" => Ok(Value::boolean(
            number_of(">=", &first())?
                >= number_of(">=", &args.get(1).cloned().unwrap_or(Value::Nil))?,
        )),
        "display" => {
            let rendered = display_value(&first());
            if let Some(output) = output {
                output.borrow_mut().push(rendered);
            }
            Ok(first())
        }
        "newline" => {
            if let Some(output) = output {
                output.borrow_mut().push(String::new());
            }
            Ok(Value::sym("newline"))
        }
        "error" => Err(op_fail(format!(
            "error: {}",
            args.iter().map(display_value).collect::<Vec<_>>().join(" ")
        ))),
        "extend-environment" => runtime_extend_environment(args),
        "lookup-variable-value" => runtime_lookup(args),
        "set-variable-value!" => runtime_set(args),
        "define-variable!" => runtime_define(args),
        "apply-in-underlying-scheme" => {
            let proc = args
                .first()
                .ok_or_else(|| op_fail("apply needs a procedure"))?;
            let args_list = args.get(1).cloned().unwrap_or(Value::Nil);
            let Some(name) = primitive_name(proc) else {
                return Err(op_fail("apply-in-underlying-scheme needs a primitive"));
            };
            let values = listed(&args_list, "apply-in-underlying-scheme")?;
            apply_runtime_primitive(output, extra, name, &values)
        }
        other => apply_object_primitive(other, args),
    }
}

fn runtime_extend_environment(args: &[Value]) -> Result<Value, Fault> {
    let names = args
        .first()
        .and_then(|word| word.list_items().ok())
        .ok_or_else(|| op_fail("extend-environment needs a name list"))?;
    let values = args
        .get(1)
        .and_then(|word| word.list_items().ok())
        .ok_or_else(|| op_fail("extend-environment needs a value list"))?;
    if names.len() != values.len() {
        return Err(op_fail(format!(
            "extend-environment: wants {} arguments, got {}",
            names.len(),
            values.len()
        )));
    }
    let base: Rc<Env> = match args.get(2) {
        Some(word) if *word == the_empty_environment_marker() => Env::global(),
        Some(word) => environment_of(word, "extend-environment")?,
        None => return Err(op_fail("extend-environment needs a parent environment")),
    };
    let extended = Env::child(&base);
    for (name, value) in names.into_iter().zip(values) {
        let Some(sym) = symbol_name(&name) else {
            return Err(op_fail("extend-environment: a name is not a symbol"));
        };
        extended.define(sym.into(), value);
    }
    Ok(intern_env(extended))
}

fn runtime_lookup(args: &[Value]) -> Result<Value, Fault> {
    let Some(name) = args.first().and_then(symbol_name) else {
        return Err(op_fail("lookup-variable-value needs a variable"));
    };
    let env = environment_of(
        &args.get(1).cloned().unwrap_or(Value::Nil),
        "lookup-variable-value",
    )?;
    env.lookup(&name)
        .map_err(|error| op_fail(error.to_string()))
}

fn runtime_set(args: &[Value]) -> Result<Value, Fault> {
    let Some(name) = args.first().and_then(symbol_name) else {
        return Err(op_fail("set-variable-value! needs a variable"));
    };
    let value = args.get(1).cloned().unwrap_or(Value::Nil);
    let env = environment_of(
        &args.get(2).cloned().unwrap_or(Value::Nil),
        "set-variable-value!",
    )?;
    env.set(&name, value)
        .map_err(|error| op_fail(error.to_string()))?;
    Ok(Value::sym("ok"))
}

fn runtime_define(args: &[Value]) -> Result<Value, Fault> {
    let Some(name) = args.first().and_then(symbol_name) else {
        return Err(op_fail("define-variable! needs a variable"));
    };
    let value = args.get(1).cloned().unwrap_or(Value::Nil);
    let env = environment_of(
        &args.get(2).cloned().unwrap_or(Value::Nil),
        "define-variable!",
    )?;
    env.define(name.into(), value);
    Ok(Value::sym("ok"))
}

// ---------------------------------------------------------------------------
// The 5.5.7 machine
// ---------------------------------------------------------------------------

/// The registers of the 5.5.7 machine description: the evaluator's
/// seven plus the `arg1` and `arg2` of exercise 5.38. The machine's
/// own `flag` is never named by a controller.
pub const MACHINE_REGISTERS: &[&str] = &[
    "exp", "env", "val", "continue", "proc", "argl", "unev", "arg1", "arg2",
];

/// The apply-dispatch of 5.5.7: the compiled-procedure test before the
/// unknown-type stop, and the compiled entry that restores `continue`
/// and jumps to the compiled code.
pub const APPLY_DISPATCH_COMPILED: &str = "apply-dispatch
  (test (op primitive-procedure?) (reg proc))
  (branch (label primitive-apply))
  (test (op compound-procedure?) (reg proc))
  (branch (label compound-apply))
  (test (op compiled-procedure?) (reg proc))
  (branch (label compiled-apply))
  (goto (label unknown-procedure-type))
compiled-apply
  (restore continue)
  (assign val (op compiled-procedure-entry) (reg proc))
  (goto (reg val))";

/// The external entry: reached when the machine starts armed, it
/// points `continue` at `print-result` and jumps to the compiled code
/// in `val`.
pub const EXTERNAL_ENTRY: &str = "external-entry
  (perform (op initialize-stack))
  (assign env (op get-global-environment))
  (assign continue (label print-result))
  (goto (reg val))";

/// The 5.4 driver with the armed-entry guard in front: the test of the
/// armed operation stands in for the book's flag branch, because the
/// 5.2 machine's flag is written only by tests. The guard runs once,
/// at the top of the assembled controller.
pub const DRIVER_WITH_GUARD: &str = ";; branches if the compiled entry is armed:
  (test (op compiled-entry-armed?))
  (branch (label external-entry))
read-eval-print-loop
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
  (goto (label read-eval-print-loop))";

/// The 5.5.7 controller in named fragments, in printed order: the 5.4
/// fragments, the guarded driver, the compiled apply-dispatch, and the
/// external entry. A variant machine replaces a fragment (the
/// monitored driver of 5.45) and concatenates.
#[must_use]
pub fn eceval_fragments() -> Vec<(&'static str, String)> {
    let mut fragments: Vec<(&'static str, String)> = crate::sec_5_4::controller_fragments()
        .iter()
        .map(|(name, text)| (*name, (*text).to_owned()))
        .collect();
    for (name, text) in [
        ("driver", DRIVER_WITH_GUARD.to_owned()),
        ("apply-dispatch", APPLY_DISPATCH_COMPILED.to_owned()),
        ("external-entry", EXTERNAL_ENTRY.to_owned()),
    ] {
        match fragments.iter_mut().find(|(known, _)| *known == name) {
            Some(slot) => slot.1 = text,
            None => fragments.push((name, text)),
        }
    }
    fragments
}

/// The 5.5.7 controller: the fragments concatenated.
#[must_use]
pub fn eceval_controller() -> String {
    eceval_fragments()
        .into_iter()
        .map(|(_, text)| text)
        .collect::<Vec<_>>()
        .join("\n")
}

/// The 5.5.7 controller with the driver fragment replaced: the
/// monitored driver of 5.45 and the chained driver of 5.49 compose
/// this way.
#[must_use]
pub fn controller_replacing_driver(driver: &str) -> String {
    eceval_fragments()
        .into_iter()
        .map(|(name, text)| {
            if name == "driver" {
                driver.to_owned()
            } else {
                text
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// One operation over the machine itself: the shape the stack
/// statistics op needs.
fn machine_op(
    name: &'static str,
    f: impl Fn(&mut Machine, &[Value]) -> Result<Value, Fault> + 'static,
) -> (&'static str, OpHandler) {
    let handler: OpHandler = Rc::new(move |machine, args| f(machine, args));
    (name, handler)
}

fn word_op(
    name: &'static str,
    f: impl Fn(&[Value]) -> Result<Value, Fault> + 'static,
) -> (&'static str, OpHandler) {
    let handler: OpHandler = Rc::new(move |_, args| f(args));
    (name, handler)
}

fn one_arg(args: &[Value], what: &str) -> Result<Value, Fault> {
    args.first()
        .cloned()
        .ok_or_else(|| op_fail(format!("{what}: needs one argument")))
}

fn two_args(args: &[Value], what: &str) -> Result<(Value, Value), Fault> {
    let a = args
        .first()
        .cloned()
        .ok_or_else(|| op_fail(format!("{what}: needs two arguments")))?;
    let b = args
        .get(1)
        .cloned()
        .ok_or_else(|| op_fail(format!("{what}: needs two arguments")))?;
    Ok((a, b))
}

/// The compiled operations and the machine's own driver operations,
/// including the armed-entry latch the guard tests and the runtime
/// primitive dispatch. Install these over the 5.4 base table, whose
/// `apply-primitive-procedure` they override.
fn base_machine_operations(
    output: &Rc<RefCell<Vec<String>>>,
    input: &Rc<RefCell<VecDeque<Value>>>,
    global: &Value,
    armed: &Rc<Cell<bool>>,
    extra_runtime: &[(String, RuntimeFn)],
) -> Vec<(&'static str, OpHandler)> {
    let extra = Rc::new(extra_runtime.to_vec());
    let apply_extra = Rc::clone(&extra);
    let output_for_apply = Rc::clone(output);
    let get_global = word_op("get-global-environment", {
        let global = global.clone();
        move |_| Ok(global.clone())
    });
    let prompt = announce_op("prompt-for-input", output, display_value);
    let announce_output = announce_op("announce-output", output, display_value);
    let user_print = announce_op("user-print", output, render_compiled_word);
    let read = word_op("read", {
        let input = Rc::clone(input);
        move |_| {
            input
                .borrow_mut()
                .pop_front()
                .ok_or_else(|| op_fail(INPUT_QUEUE_EMPTY))
        }
    });
    let statistics = machine_op("print-stack-statistics", {
        let output = Rc::clone(output);
        move |machine, _| {
            let (pushes, depth) = machine.stack_statistics();
            output
                .borrow_mut()
                .push(format!("(total-pushes = {pushes} maximum-depth = {depth})"));
            Ok(Value::sym("done"))
        }
    });
    let initialize_stack = machine_op("initialize-stack", |machine, _| {
        machine.initialize_stack();
        Ok(Value::sym("done"))
    });
    let armed_latch = Rc::clone(armed);
    let armed_test = word_op("compiled-entry-armed?", move |_| {
        if armed_latch.get() {
            armed_latch.set(false);
            Ok(Value::boolean(true))
        } else {
            Ok(Value::boolean(false))
        }
    });
    let apply_primitive = word_op("apply-primitive-procedure", move |args| {
        let (proc, argl) = two_args(args, "apply-primitive-procedure")?;
        let Some(name) = primitive_name(&proc) else {
            return Err(op_fail(
                "apply-primitive-procedure needs a primitive procedure",
            ));
        };
        let values = listed(&argl, "apply-primitive-procedure")?;
        apply_runtime_primitive(Some(&output_for_apply), &apply_extra, name, &values)
    });
    vec![
        get_global,
        prompt,
        announce_output,
        read,
        user_print,
        statistics,
        initialize_stack,
        armed_test,
        word_op("false?", |args| {
            Ok(Value::boolean(!object_is_true(&one_arg(args, "false?")?)))
        }),
        word_op("list", |args| Ok(Value::list(args.to_vec()))),
        word_op("cons", |args| {
            let (a, b) = two_args(args, "cons")?;
            Ok(Value::Pair(cons_cell(a, b)))
        }),
        word_op("make-compiled-procedure", |args| {
            let (entry, env) = two_args(args, "make-compiled-procedure")?;
            Ok(compiled_procedure_word(entry, env))
        }),
        word_op("compiled-procedure-env", |args| {
            compiled_procedure_parts(&one_arg(args, "compiled-procedure-env")?)
                .map(|(_, env)| env)
                .ok_or_else(|| op_fail("compiled-procedure-env needs a compiled procedure"))
        }),
        word_op("compiled-procedure-entry", |args| {
            compiled_procedure_parts(&one_arg(args, "compiled-procedure-entry")?)
                .map(|(entry, _)| entry)
                .ok_or_else(|| op_fail("compiled-procedure-entry needs a compiled procedure"))
        }),
        word_op("compiled-procedure?", |args| {
            Ok(Value::boolean(
                compiled_procedure_parts(&one_arg(args, "compiled-procedure?")?).is_some(),
            ))
        }),
        apply_primitive,
    ]
    .into_iter()
    .chain(open_coded_operations())
    .collect()
}

/// The machine operations the open-coded compilations name directly:
/// the 5.38 argument registers feed the same object arithmetic the
/// primitive table serves.
fn open_coded_operations() -> Vec<(&'static str, OpHandler)> {
    ["+", "-", "*", "<", "="]
        .iter()
        .map(|name| word_op(name, move |args| apply_object_primitive(name, args)))
        .collect()
}

fn announce_op(
    name: &'static str,
    output: &Rc<RefCell<Vec<String>>>,
    render: impl Fn(&Value) -> String + 'static,
) -> (&'static str, OpHandler) {
    let output = Rc::clone(output);
    word_op(name, move |args| {
        let word = args.first().cloned().unwrap_or(Value::Nil);
        output.borrow_mut().push(render(&word));
        Ok(Value::sym("done"))
    })
}

/// The section's compiled evaluator: the controller (the 5.5.7 text,
/// or a composed variant) is assembled by the 5.2 simulator over the
/// 5.4 base operations, the compiled operations, and the caller's
/// `operations` last (overriding on a name collision); the global
/// environment binds `true`, `false`, and the runtime primitives; and
/// the object program `source` is read into the driver's input queue.
/// The armed latch starts false, so the plain driver path runs.
///
/// # Errors
/// [`Fault::Parse`] when the controller or the source is unreadable,
/// plus the assembly faults of [`Fault`].
pub fn make_compiled_evaluator(
    controller: Option<&str>,
    operations: &[(&'static str, OpHandler)],
    extra_runtime: &[(String, RuntimeFn)],
    source: &str,
) -> Result<CompiledEvaluator, Fault> {
    let forms = read_program(source).map_err(|error| Fault::Parse(error.to_string()))?;
    let output = Rc::new(RefCell::new(Vec::new()));
    let input = Rc::new(RefCell::new(VecDeque::from(forms)));
    let armed = Rc::new(Cell::new(false));
    let global_env = Env::global();
    global_env.define("true".into(), Value::boolean(true));
    global_env.define("false".into(), Value::boolean(false));
    for name in runtime_names() {
        global_env.define(name.into(), primitive_word(name));
    }
    for (name, _) in extra_runtime {
        global_env.define(name.as_str().into(), primitive_word(name));
    }
    let global = intern_env(global_env);
    let mut table: Vec<(&'static str, OpHandler)> = crate::sec_5_4::base_operations();
    table.extend(base_machine_operations(
        &output,
        &input,
        &global,
        &armed,
        extra_runtime,
    ));
    table.extend(operations.iter().cloned());
    let text = controller.map_or_else(eceval_controller, std::borrow::ToOwned::to_owned);
    let machine = make_machine(MACHINE_REGISTERS, &table, &text)?;
    Ok(CompiledEvaluator {
        machine,
        output,
        armed,
    })
}

/// The section's compiled evaluator over the 5.5.7 controller.
pub struct CompiledEvaluator {
    machine: Machine,
    output: Rc<RefCell<Vec<String>>>,
    armed: Rc<Cell<bool>>,
}

impl CompiledEvaluator {
    /// The machine, mutable, for the monitoring extensions.
    pub fn machine_mut(&mut self) -> &mut Machine {
        &mut self.machine
    }

    /// The machine.
    #[must_use]
    pub fn machine(&self) -> &Machine {
        &self.machine
    }

    /// Points `val` at the compiled entry and arms the external entry,
    /// the book's `compile-and-go` wiring.
    pub fn arm_entry(&mut self, entry: &str) {
        let _ = self.machine.set_register("val", Value::sym(entry));
        self.armed.set(true);
    }

    /// Arms or disarms the external entry.
    pub fn arm(&self, armed: bool) {
        self.armed.set(armed);
    }

    /// Runs the machine until the input queue runs dry, the book's
    /// read-eval-print loop.
    ///
    /// # Errors
    /// Any fault of the machine except the queue-dry read, which is
    /// the run's normal end.
    pub fn run(&mut self) -> Result<(), Fault> {
        match self.machine.start() {
            Ok(_) => Ok(()),
            Err(Fault::Op { op, message, .. }) if op == "read" && message == INPUT_QUEUE_EMPTY => {
                Ok(())
            }
            Err(fault) => Err(fault),
        }
    }

    /// The lines the driver printed: the prompts, the stack statistics
    /// of a monitored driver, and the values, in order.
    #[must_use]
    pub fn transcript(&self) -> Vec<String> {
        self.output.borrow().clone()
    }

    /// The machine's stack counters `(total-pushes, maximum-depth)`.
    #[must_use]
    pub fn stack_statistics(&self) -> (u64, u64) {
        self.machine.stack_statistics()
    }

    /// The instruction count of the run so far.
    #[must_use]
    pub fn instruction_count(&self) -> u64 {
        self.machine.instruction_count()
    }

    /// Reads a register's contents.
    ///
    /// # Errors
    /// [`Fault::UnknownRegister`] when the machine has no such
    /// register.
    pub fn get_register(&self, name: &str) -> Result<Value, Fault> {
        self.machine.get_register(name)
    }
}

/// The book's `compile-and-go`: compiles `compiled`, appends its block
/// to the 5.5.7 controller, points `val` at the entry, arms the
/// external entry, and answers the machine whose driver inputs are
/// `source`.
///
/// # Errors
/// [`Fault::Parse`] when either text is unreadable, plus the assembly
/// faults.
pub fn compile_and_go(
    cfg: &Config,
    state: &State,
    compiled: &str,
    source: &str,
) -> Result<CompiledEvaluator, Fault> {
    let (entry, block) = compile_block(cfg, state, compiled)?;
    let controller = format!("{}\n{block}", eceval_controller());
    let mut evaluator = make_compiled_evaluator(Some(&controller), &[], &[], source)?;
    evaluator.arm_entry(&entry);
    Ok(evaluator)
}

// ---------------------------------------------------------------------------
// The lexical-addressing machine of 5.39 to 5.42
// ---------------------------------------------------------------------------

/// The lexical machine's global environment: the book's 3.2 list
/// structure, a frame of `(names . values)` dotted pairs chaining by
/// `cons`, pre-bound with booleans and runtime primitives.
#[must_use]
pub fn lexical_global_environment() -> Value {
    let mut names = vec![Value::sym("true"), Value::sym("false")];
    let mut values = vec![Value::boolean(true), Value::boolean(false)];
    for name in runtime_names() {
        names.push(Value::sym(name));
        values.push(primitive_word(name));
    }
    let frame = cons_cell(Value::list(names), Value::list(values));
    Value::Pair(cons_cell(Value::Pair(frame), Value::Nil))
}

fn lexical_frames(env: &Value) -> Result<Vec<Value>, Fault> {
    env.list_items()
        .map_err(|_| op_fail("the environment is not a frame list"))
}

fn lexical_frame_at(env: &Value, frame_number: i128) -> Result<Value, Fault> {
    let frames = lexical_frames(env)?;
    let index =
        usize::try_from(frame_number).map_err(|_| op_fail("lexical address out of range"))?;
    frames
        .get(index)
        .cloned()
        .ok_or_else(|| op_fail("lexical address out of range"))
}

/// The value a frame's `displacement`th binding carries.
fn lexical_binding(frame: &Value, displacement: usize) -> Result<Value, Fault> {
    let Value::Pair(cell) = frame else {
        return Err(op_fail("the frame is not a dotted pair"));
    };
    let values = cell.cdr.borrow().clone();
    listed(&values, "the frame")?
        .into_iter()
        .nth(displacement)
        .ok_or_else(|| op_fail("lexical address out of range"))
}

/// The mutable cell holding a binding's value: the cons cell the
/// `displacement`th value of the frame rides in.
fn lexical_value_cell(frame: &Value, displacement: usize) -> Result<Pair, Fault> {
    let Value::Pair(cell) = frame else {
        return Err(op_fail("the frame is not a dotted pair"));
    };
    let values = cell.cdr.borrow().clone();
    let mut cursor = values;
    for index in 0..=displacement {
        let Value::Pair(pair) = cursor else {
            return Err(op_fail("lexical address out of range"));
        };
        let next = pair.cdr.borrow().clone();
        if index == displacement {
            return Ok(pair);
        }
        cursor = next;
    }
    Err(op_fail("lexical address out of range"))
}

/// The operations of the lexical machine: the environment pathway runs
/// on the book's list structure, and the two lexical addressing
/// operations walk it. Install these last so they override the word
/// environment pathway.
#[must_use]
pub fn lexical_operations(global: &Value) -> Vec<(&'static str, OpHandler)> {
    vec![
        lexical_get_global(global),
        lexical_extend(),
        lexical_lookup(),
        lexical_set(),
        lexical_define(),
        lexical_lookup_op(),
        lexical_set_op(),
    ]
}

fn lexical_get_global(global: &Value) -> (&'static str, OpHandler) {
    word_op("get-global-environment", {
        let global = global.clone();
        move |_| Ok(global.clone())
    })
}

fn lexical_extend() -> (&'static str, OpHandler) {
    word_op("extend-environment", |args| {
        let (params, argl) = two_args(args, "extend-environment")?;
        let base = args
            .get(2)
            .cloned()
            .ok_or_else(|| op_fail("extend-environment needs an environment"))?;
        let frame = cons_cell(params, argl);
        Ok(Value::Pair(cons_cell(Value::Pair(frame), base)))
    })
}

/// Walks a frame's names, answering the displacement of `name`, or
/// nothing.
fn frame_displacement(frame: &Value, name: &Value) -> Option<usize> {
    let Value::Pair(cell) = frame else {
        return None;
    };
    let names = cell.car.borrow().clone();
    let names = listed(&names, "the frame").ok()?;
    names.iter().position(|candidate| candidate == name)
}

fn lexical_lookup() -> (&'static str, OpHandler) {
    word_op("lookup-variable-value", |args| {
        let (name, env) = two_args(args, "lookup-variable-value")?;
        for frame in lexical_frames(&env)? {
            if let Some(displacement) = frame_displacement(&frame, &name) {
                let value = lexical_binding(&frame, displacement)?;
                if value == Value::sym("*unassigned*") {
                    return Err(op_fail(
                        "lexical-address-lookup: the variable is *unassigned*",
                    ));
                }
                return Ok(value);
            }
        }
        Err(op_fail(format!(
            "unbound variable: {}",
            display_value(&name)
        )))
    })
}

fn lexical_set() -> (&'static str, OpHandler) {
    word_op("set-variable-value!", |args| {
        let (name, value) = two_args(args, "set-variable-value!")?;
        let env = args
            .get(2)
            .cloned()
            .ok_or_else(|| op_fail("set-variable-value! needs an environment"))?;
        for frame in lexical_frames(&env)? {
            if let Some(displacement) = frame_displacement(&frame, &name) {
                let slot = lexical_value_cell(&frame, displacement)?;
                set_car(&slot, value);
                return Ok(Value::sym("ok"));
            }
        }
        Err(op_fail(format!(
            "unbound variable: {}",
            display_value(&name)
        )))
    })
}

fn lexical_define() -> (&'static str, OpHandler) {
    word_op("define-variable!", |args| {
        let (name, value) = two_args(args, "define-variable!")?;
        let env = args
            .get(2)
            .cloned()
            .ok_or_else(|| op_fail("define-variable! needs an environment"))?;
        let frame = lexical_frames(&env)?
            .into_iter()
            .next()
            .ok_or_else(|| op_fail("define-variable! needs a frame"))?;
        let Value::Pair(frame_cell) = &frame else {
            return Err(op_fail("the frame is not a dotted pair"));
        };
        if let Some(displacement) = frame_displacement(&frame, &name) {
            let slot = lexical_value_cell(&frame, displacement)?;
            set_car(&slot, value);
            return Ok(Value::sym("ok"));
        }
        let names = frame_cell.car.borrow().clone();
        let values = frame_cell.cdr.borrow().clone();
        set_car(frame_cell, Value::Pair(cons_cell(name, names)));
        set_cdr(frame_cell, Value::Pair(cons_cell(value, values)));
        Ok(Value::sym("ok"))
    })
}

fn lexical_lookup_op() -> (&'static str, OpHandler) {
    word_op("lexical-address-lookup", |args| {
        let frame_number = number_of(
            "lexical-address-lookup",
            &args.first().cloned().unwrap_or(Value::Nil),
        )?;
        let displacement = number_of(
            "lexical-address-lookup",
            &args.get(1).cloned().unwrap_or(Value::Nil),
        )?;
        let env = args
            .get(2)
            .cloned()
            .ok_or_else(|| op_fail("lexical-address-lookup needs an environment"))?;
        let frame = lexical_frame_at(&env, frame_number)?;
        let index =
            usize::try_from(displacement).map_err(|_| op_fail("lexical address out of range"))?;
        let value = lexical_binding(&frame, index)?;
        if value == Value::sym("*unassigned*") {
            return Err(op_fail(
                "lexical-address-lookup: the variable is *unassigned*",
            ));
        }
        Ok(value)
    })
}

fn lexical_set_op() -> (&'static str, OpHandler) {
    word_op("lexical-address-set!", |args| {
        let frame_number = number_of(
            "lexical-address-set!",
            &args.first().cloned().unwrap_or(Value::Nil),
        )?;
        let displacement = number_of(
            "lexical-address-set!",
            &args.get(1).cloned().unwrap_or(Value::Nil),
        )?;
        let value = args.get(2).cloned().unwrap_or(Value::Nil);
        let env = args
            .get(3)
            .cloned()
            .ok_or_else(|| op_fail("lexical-address-set! needs an environment"))?;
        let frame = lexical_frame_at(&env, frame_number)?;
        let index =
            usize::try_from(displacement).map_err(|_| op_fail("lexical address out of range"))?;
        let slot = lexical_value_cell(&frame, index)?;
        set_car(&slot, value);
        Ok(Value::sym("ok"))
    })
}

// ---------------------------------------------------------------------------
// The object-language evaluator source of exercises 5.50 and 5.52
// ---------------------------------------------------------------------------

/// The metacircular evaluator of 4.1 as object-language source: the
/// program 5.50 compiles and 5.52's C backend compiles again. The
/// shared corpus ships the same evaluator at
/// `spec/scheme-subset/programs/core/metacircular.scm`; this copy
/// differs in the ways this machine forces, all mechanical:
///
/// - The book's list-structure environments (whose frame machinery
///   needs mutable pairs) are the machine's own environments behind
///   the runtime primitives, so their object-level definitions and
///   the frame machinery below them are gone; `'the-empty` marks the
///   empty parent.
/// - The `apply` table entry is an object-level definition applied
///   through `m-eval`, because a primitive of this machine applies
///   primitives only; `apply-in-underlying-scheme` is pre-bound by
///   the machine.
/// - The `/` table entry loses its variadic `lambda` and
///   `exact->inexact`, which the shared grammar has no form for.
/// - The corpus's apply-on-compound session (`(twice cons 7)`) is not
///   reproduced: a compiled procedure object cannot re-enter the
///   evaluator's `m-apply` through a primitive, so the object-level
///   `apply` definition the corpus needs has nothing to call.
///
/// The evaluator proper (`m-eval`, `m-apply`, the syntax procedures,
/// `cond->if`, the driver calls) is the corpus's text verbatim.
pub const METACIRCULAR: &str = include_str!("metacircular_source.scm");

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// The whitespace-free token stream of one controller line.
    fn tokens(line: &str) -> Vec<String> {
        line.split_whitespace().map(String::from).collect()
    }

    /// The compiled factorial's statements.
    fn compiled_factorial() -> Vec<String> {
        let source = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";
        let seq = compile_program(&default_config(), &new_state(), source, &Linkage::Next)
            .expect("compiles");
        seq.stmts
    }

    /// The book's Figure 5.17 listing, comments stripped and wrapped
    /// instructions unwrapped.
    const FIGURE_5_17: &str = "\
(assign val (op make-compiled-procedure) (label entry2) (reg env))
(goto (label after-lambda1))
entry2
(assign env (op compiled-procedure-env) (reg proc))
(assign env (op extend-environment) (const (n)) (reg argl) (reg env))
(save continue)
(save env)
(assign proc (op lookup-variable-value) (const =) (reg env))
(assign val (const 1))
(assign argl (op list) (reg val))
(assign val (op lookup-variable-value) (const n) (reg env))
(assign argl (op cons) (reg val) (reg argl))
(test (op primitive-procedure?) (reg proc))
(branch (label primitive-branch17))
compiled-branch16
(assign continue (label after-call15))
(assign val (op compiled-procedure-entry) (reg proc))
(goto (reg val))
primitive-branch17
(assign val (op apply-primitive-procedure) (reg proc) (reg argl))
after-call15
(restore env)
(restore continue)
(test (op false?) (reg val))
(branch (label false-branch4))
true-branch5
(assign val (const 1))
(goto (reg continue))
false-branch4
(assign proc (op lookup-variable-value) (const *) (reg env))
(save continue)
(save proc)
(assign val (op lookup-variable-value) (const n) (reg env))
(assign argl (op list) (reg val))
(save argl)
(assign proc (op lookup-variable-value) (const factorial) (reg env))
(save proc)
(assign proc (op lookup-variable-value) (const -) (reg env))
(assign val (const 1))
(assign argl (op list) (reg val))
(assign val (op lookup-variable-value) (const n) (reg env))
(assign argl (op cons) (reg val) (reg argl))
(test (op primitive-procedure?) (reg proc))
(branch (label primitive-branch8))
compiled-branch7
(assign continue (label after-call6))
(assign val (op compiled-procedure-entry) (reg proc))
(goto (reg val))
primitive-branch8
(assign val (op apply-primitive-procedure) (reg proc) (reg argl))
after-call6
(assign argl (op list) (reg val))
(restore proc)
(test (op primitive-procedure?) (reg proc))
(branch (label primitive-branch11))
compiled-branch10
(assign continue (label after-call9))
(assign val (op compiled-procedure-entry) (reg proc))
(goto (reg val))
primitive-branch11
(assign val (op apply-primitive-procedure) (reg proc) (reg argl))
after-call9
(restore argl)
(assign argl (op cons) (reg val) (reg argl))
(restore proc)
(restore continue)
(test (op primitive-procedure?) (reg proc))
(branch (label primitive-branch14))
compiled-branch13
(assign val (op compiled-procedure-entry) (reg proc))
(goto (reg val))
primitive-branch14
(assign val (op apply-primitive-procedure) (reg proc) (reg argl))
(goto (reg continue))
after-call12
after-if3
after-lambda1
(perform (op define-variable!) (const factorial) (reg val) (reg env))
(assign val (const ok))";

    /// The book's Figure 5.18 listing, comments stripped.
    const FIGURE_5_18: &str = "\
(assign val (op make-compiled-procedure) (label entry16) (reg env))
(goto (label after-lambda15))
entry16
(assign env (op compiled-procedure-env) (reg proc))
(assign env (op extend-environment) (const (x)) (reg argl) (reg env))
(assign proc (op lookup-variable-value) (const +) (reg env))
(save continue) (save proc) (save env)
(assign proc (op lookup-variable-value) (const g) (reg env))
(save proc)
(assign proc (op lookup-variable-value) (const +) (reg env))
(assign val (const 2))
(assign argl (op list) (reg val))
(assign val (op lookup-variable-value) (const x) (reg env))
(assign argl (op cons) (reg val) (reg argl))
(test (op primitive-procedure?) (reg proc))
(branch (label primitive-branch19))
compiled-branch18
(assign continue (label after-call17))
(assign val (op compiled-procedure-entry) (reg proc))
(goto (reg val))
primitive-branch19
(assign val (op apply-primitive-procedure) (reg proc) (reg argl))
after-call17
(assign argl (op list) (reg val))
(restore proc)
(test (op primitive-procedure?) (reg proc))
(branch (label primitive-branch22))
compiled-branch21
(assign continue (label after-call20))
(assign val (op compiled-procedure-entry) (reg proc))
(goto (reg val))
primitive-branch22
(assign val (op apply-primitive-procedure) (reg proc) (reg argl))
after-call20
(assign argl (op list) (reg val))
(restore env)
(assign val (op lookup-variable-value) (const x) (reg env))
(assign argl (op cons) (reg val) (reg argl))
(restore proc)
(restore continue)
(test (op primitive-procedure?) (reg proc))
(branch (label primitive-branch25))
compiled-branch24
(assign val (op compiled-procedure-entry) (reg proc))
(goto (reg val))
primitive-branch25
(assign val (op apply-primitive-procedure) (reg proc) (reg argl))
(goto (reg continue))
after-call23
after-lambda15
(perform (op define-variable!) (const f) (reg val) (reg env))
(assign val (const ok))";

    /// The book's token streams, with the make-compiled-procedure
    /// operand mapped to this edition's spelling.
    fn book_tokens(figure: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut previous_was_make = false;
        for token in figure.lines().flat_map(tokens) {
            if token == "(label" && previous_was_make {
                out.push("(const".to_owned());
            } else {
                out.push(token.clone());
            }
            previous_was_make = token.contains("make-compiled-procedure)");
        }
        out
    }

    /// The compiled factorial is the book's Figure 5.17, token for
    /// token, modulo the one forced spelling.
    #[test]
    fn the_compiled_factorial_is_figure_5_17() {
        let statements = compiled_factorial();
        let actual = book_tokens(&statements_text(&Seq {
            needs: Vec::new(),
            modifies: Vec::new(),
            stmts: statements,
        }));
        let expected = book_tokens(FIGURE_5_17);
        assert_eq!(actual, expected);
    }

    /// The 5.35 expression behind Figure 5.18, compiled with the
    /// counter seeded at 14, matches the figure token for token.
    #[test]
    fn the_seeded_compilation_is_figure_5_18() {
        let source = "(define (f x) (+ x (g (+ x 2))))";
        let seq = compile_program(
            &default_config(),
            &new_state_seeded(14),
            source,
            &Linkage::Next,
        )
        .expect("compiles");
        let actual = book_tokens(&statements_text(&seq));
        let expected = book_tokens(FIGURE_5_18);
        assert_eq!(actual, expected);
    }

    /// The monitored driver of 5.4.4 over the 5.5.7 fragments.
    fn monitored_driver() -> String {
        ";; branches if the compiled entry is armed:
  (test (op compiled-entry-armed?))
  (branch (label external-entry))
read-eval-print-loop
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
  (goto (label read-eval-print-loop))"
            .to_owned()
    }

    /// The `(total-pushes ...)` lines of a monitored session.
    fn stats_lines(transcript: &[String]) -> Vec<String> {
        transcript
            .iter()
            .filter(|line| line.starts_with("(total-pushes"))
            .cloned()
            .collect()
    }

    /// The value of the last interaction.
    fn last_value(transcript: &[String]) -> &str {
        let at = transcript.len().saturating_sub(2);
        transcript[at].as_str()
    }

    /// The book's 5.5.7 session: the compiled define answers ok, the
    /// interpreted call answers 120 through the compiled
    /// apply-dispatch, and the monitored counters are the book's 0/0
    /// and 31/14.
    #[test]
    fn the_compiled_session_matches_the_book() {
        let source = "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))";
        let (entry, block) =
            compile_block(&default_config(), &new_state(), source).expect("compiles");
        let controller = controller_replacing_driver(&monitored_driver()) + "\n" + &block;
        let mut evaluator = make_compiled_evaluator(Some(&controller), &[], &[], "(factorial 5)")
            .expect("assembles");
        evaluator.arm_entry(&entry);
        evaluator.run().expect("runs");
        let transcript = evaluator.transcript();
        assert_eq!(
            stats_lines(&transcript),
            vec![
                "(total-pushes = 0 maximum-depth = 0)".to_owned(),
                "(total-pushes = 31 maximum-depth = 14)".to_owned(),
            ]
        );
        assert_eq!(last_value(&transcript), "120");
    }

    /// The driver path still interprets: a plain interaction runs the
    /// interpreted evaluator over the 5.5.7 controller.
    #[test]
    fn the_driver_path_still_interprets() {
        let mut evaluator = make_compiled_evaluator(None, &[], &[], "(+ 2 3)").expect("assembles");
        evaluator.run().expect("runs");
        assert_eq!(last_value(&evaluator.transcript()), "5");
    }

    /// Compiled procedures flow through the compiled apply-dispatch,
    /// and the driver path still interprets its own lambda: the
    /// compiled `twice` runs a compiled lambda, then the driver
    /// applies an interpreted one.
    #[test]
    fn compiled_code_calls_both_kinds() {
        let mut evaluator = compile_and_go(
            &default_config(),
            &new_state(),
            "(define (twice f x) (f (f x)))\n(twice (lambda (y) (* y y)) 3)",
            "(define (g x) (* x x))\n(g 7)",
        )
        .expect("compiles");
        evaluator.run().expect("runs");
        let transcript = evaluator.transcript();
        assert!(transcript.contains(&"81".to_owned()), "{transcript:?}");
        assert_eq!(last_value(&transcript), "49");
    }

    /// The 5.5.7 controller assembles with the new entries present.
    #[test]
    fn the_controller_carries_the_new_entries() {
        let controller = eceval_controller();
        assert!(controller.contains("compiled-apply"));
        assert!(controller.contains("external-entry"));
        assert!(controller.contains("compiled-entry-armed?"));
    }
}
