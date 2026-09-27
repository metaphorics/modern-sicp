// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Section 4.4: the query system over the chapter's [`Value`] domain.
//!
//! The port keeps the book's four layers. The driver ([`QueryEngine::session`],
//! [`QueryEngine::answers_upto`]) reads one input, files an `assert!` body, or
//! runs [`qeval`] over a stream holding one empty frame and instantiates the
//! answers. The evaluator ([`qeval`]) classifies a query by the symbol in its
//! `car` and dispatches through the data-directed table — the book's
//! `put`/`get` keyed `(type, 'qeval)` — with any untagged pattern falling to
//! the simple-query processor. The matcher ([`pattern_match`]) and unifier
//! ([`unify_match`]) walk `Value` pairs exactly as the book's 4.4.4.3 and
//! 4.4.4.4 walk list structure, and the stream layer ([`stream_flatmap`],
//! [`stream_append_delayed`], [`interleave_delayed`]) combines frame streams.
//!
//! # One stream choice
//!
//! Every `delay`/`force` in the book's listings is the chapter 3 memoized
//! stream: [`Stream`] over [`Frame`], so `cons-stream`'s thunk is
//! [`Stream::cons_stream`]'s tail closure and `stream-append-delayed`'s
//! delayed argument is a plain `FnOnce` tail. The plain (undelayed) twins
//! [`stream_append`] and [`interleave`] are exported too, because exercises
//! 4.71 to 4.73 are about exactly the difference between the two.
//!
//! # Frames and the unmarked-inhabitant rule
//!
//! A [`Frame`] is an immutable association structure: a chain of
//! variable-value pairs built only through [`Frame::extend`], read through
//! [`Frame::binding_in_frame`]. Failure never inhabits a frame — the
//! book's `failed` symbol left the data, and the matcher's failure is
//! `None`. The frame variables are the internal `(? name)` list values
//! produced by [`query_syntax_process`], and renamed rule variables are
//! `(? id name)` ([`make_new_variable`]); both are list values distinct
//! from every data symbol, so a renamed rule variable cannot collide with
//! an explicitly written one.
//!
//! # The data base is chronological
//!
//! The book's `add-assertion!` conses the newest entry in front, yet every
//! sample interaction the book pins lists answers in insertion order. This
//! port stores assertions and rules as appended vectors behind the stream
//! interface, indexed by the leading symbol exactly as 4.4.4.5 describes
//! (plus the `?` bucket for rules whose conclusion starts with a
//! variable); each fetch answers a stream of the stored items in
//! insertion order, and installation binds the old collection before the
//! new one lands — the discipline the book's `let` enforces (exercise
//! 4.70 models what goes wrong without it).
//!
//! # Reading queries
//!
//! The driver reads query input with the shared `sicp_runtime::read`. The
//! OCaml edition needed a private data-language scanner because its shared
//! reader rejects `()`, dotted patterns, and reads `9am` as a bad number;
//! this edition's reader already accepts all three — `()` is `Nil`,
//! `(computer . ?type)` is a dotted pair, and `9am` classifies as a symbol
//! — so no separate scanner exists here and the lexical conventions are
//! the shared reader's.
//!
//! # Object errors
//!
//! `lisp-value` on an unbound pattern variable raises the book's `Unknown
//! pat var` error. A raised error lands in the engine's error slot
//! ([`QueryEngine::take_error`]) because a stream cannot carry a `Result`;
//! the driver turns a raised error into the transcript's `Error:` line,
//! the way the other drivers in this chapter report.
//!
//! # The two seams
//!
//! Exercise extensions install through data, not edits. The dispatch table
//! is open: [`QueryEngine::put`] registers a processor for any type symbol
//! (4.75's `unique`, 4.76's merging `and`, 4.77's promised filters,
//! 4.72's appending `or`). The simple-query fallback is replaceable:
//! [`QueryEngine::set_fallback`] swaps the processor untagged patterns
//! take (4.71a's undelayed simple query, 4.74's simple-flatmap evaluator,
//! 4.67's loop detector, 4.79's scoped rule application). Nothing in the
//! engine routes through an installed extension, and a fresh engine is
//! plain book behavior.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use sicp_runtime::{Handler, SchemeError, Stream, Value, cons_cell, print_value};

use crate::sec_4_1::{OutputSink, is_true, primitive_table};

/// One evaluation's answer in the object language.
pub type EvalResult = Result<Value, SchemeError>;

/// An engine handle: the recursive layer closes over this, because a
/// stream's tail must be `'static` and the evaluator recurses inside
/// stream tails.
pub type Engine = Rc<QueryEngine>;

/// A query processor: the book's entry in the `(type 'qeval)` table. It
/// receives the engine, the contents of the tagged query (or the whole
/// pattern, when the processor is the simple-query fallback), and the
/// input frame stream.
pub type QProc = Rc<dyn Fn(&Engine, &Value, Stream<Frame>) -> Stream<Frame>>;

/// The per-element step a flatmap maps over a stream of `A`.
pub type StepFn<A, B> = Rc<dyn Fn(&A) -> Stream<B>>;

/// The book's `stream-flatmap`: the one frame combinator the evaluator
/// uses. The default combines by interleaving; exercise 4.74 swaps in a
/// combiner with the same two duties.
pub trait Combiner {
    /// Maps `proc` over the frame stream and combines the inner streams.
    fn combine_frames(&self, proc: StepFn<Frame, Frame>, s: Stream<Frame>) -> Stream<Frame>;

    /// The same combination when the mapped stream carries data values,
    /// the shape `find-assertions` maps over.
    fn combine_values(&self, proc: StepFn<Value, Frame>, s: Stream<Value>) -> Stream<Frame>;
}

/// The book's combinator: `stream-flatmap`, interleaving the inner
/// streams.
#[derive(Debug, Default)]
pub struct Interleaved;

impl Combiner for Interleaved {
    fn combine_frames(&self, proc: StepFn<Frame, Frame>, s: Stream<Frame>) -> Stream<Frame> {
        stream_flatmap(proc, s)
    }

    fn combine_values(&self, proc: StepFn<Value, Frame>, s: Stream<Value>) -> Stream<Frame> {
        stream_flatmap(proc, s)
    }
}

// ---------------------------------------------------------------------------
// Frames (4.4.4.8).
// ---------------------------------------------------------------------------

/// One binding of the chain: the book's `(cons variable value)` pair.
#[derive(Debug)]
struct FrameNode {
    variable: Value,
    value: Value,
    next: Frame,
}

/// The frame of 4.4.4.8: an immutable association list of variable-value
/// pairs. `extend` conses a binding, cloning a pointer; `binding_in_frame`
/// is the book's `assoc` under `equal?`. Nothing here stores a sentinel:
/// an absent binding is `None`, never a marker value.
#[derive(Clone, Debug, Default)]
pub struct Frame(Option<Rc<FrameNode>>);

impl Frame {
    /// The book's empty frame, the singleton the driver starts from.
    #[must_use]
    pub fn new() -> Frame {
        Frame(None)
    }

    /// The book's `extend`: one more binding in front of the chain.
    #[must_use]
    pub fn extend(&self, variable: Value, value: Value) -> Frame {
        Frame(Some(Rc::new(FrameNode {
            variable,
            value,
            next: self.clone(),
        })))
    }

    /// The book's `binding-in-frame`, answering the binding's value
    /// directly; `None` is the absence the book's `false` named.
    #[must_use]
    pub fn binding_in_frame(&self, variable: &Value) -> Option<Value> {
        let mut node = self.0.as_ref();
        while let Some(n) = node {
            if &n.variable == variable {
                return Some(n.value.clone());
            }
            node = n.next.0.as_ref();
        }
        None
    }

    /// Whether the frame holds no bindings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    /// The bindings, newest first (the chain's cons order).
    #[must_use]
    pub fn bindings(&self) -> Vec<(Value, Value)> {
        let mut out = Vec::new();
        let mut node = self.0.as_ref();
        while let Some(n) = node {
            out.push((n.variable.clone(), n.value.clone()));
            node = n.next.0.as_ref();
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Query syntax procedures (4.4.4.7).
// ---------------------------------------------------------------------------

/// The book's `var?`: an internal variable is the list `(? name)`.
#[must_use]
pub fn is_var(exp: &Value) -> bool {
    matches!(exp, Value::Pair(cell) if matches!(&*cell.car.borrow(), Value::Sym(s) if &**s == "?"))
}

/// The book's `constant-symbol?`: the data symbols a pattern can start with.
#[must_use]
pub fn constant_symbol(exp: &Value) -> bool {
    matches!(exp, Value::Sym(_))
}

/// The book's `query-syntax-process`: every `?x` symbol becomes the list
/// `(? x)`, walking pairs including dotted tails.
#[must_use]
pub fn query_syntax_process(exp: &Value) -> Value {
    map_over_symbols(exp, &expand_question_mark)
}

fn map_over_symbols(exp: &Value, proc: &dyn Fn(&str) -> Value) -> Value {
    match exp {
        Value::Pair(cell) => Value::Pair(cons_cell(
            map_over_symbols(&cell.car.borrow(), proc),
            map_over_symbols(&cell.cdr.borrow(), proc),
        )),
        Value::Sym(s) => proc(s),
        other => other.clone(),
    }
}

fn expand_question_mark(symbol: &str) -> Value {
    match symbol.strip_prefix('?') {
        Some(name) => Value::list(vec![Value::sym("?"), Value::sym(name)]),
        None => Value::sym(symbol),
    }
}

/// The book's `contract-question-mark`: the printed form of an internal
/// variable, `?name` or the renamed `?name-id`.
#[must_use]
pub fn contract_question_mark(variable: &Value) -> Value {
    let Ok(items) = variable.list_items() else {
        return variable.clone();
    };
    match items.as_slice() {
        [_, Value::Sym(name)] => Value::sym(&format!("?{name}")),
        [_, Value::Int(id), Value::Sym(name)] => Value::sym(&format!("?{name}-{id}")),
        _ => variable.clone(),
    }
}

/// The book's `make-new-variable`: `(? name)` becomes `(? id name)` so two
/// applications of one rule never confuse their variables.
#[must_use]
pub fn make_new_variable(var: &Value, rule_application_id: u64) -> Value {
    let Value::Pair(cell) = var else {
        return var.clone();
    };
    let Value::Pair(rest) = cell.cdr.borrow().clone() else {
        return var.clone();
    };
    let name = rest.car.borrow().clone();
    Value::list(vec![
        Value::sym("?"),
        Value::int(i128::from(rule_application_id)),
        name,
    ])
}

/// The book's `rename-variables-in`, with the application id the caller
/// drew from the engine's counter.
#[must_use]
pub fn rename_variables_in(rule: &Value, rule_application_id: u64) -> Value {
    fn tree_walk(exp: &Value, id: u64) -> Value {
        if is_var(exp) {
            return make_new_variable(exp, id);
        }
        if let Value::Pair(cell) = exp {
            return Value::Pair(cons_cell(
                tree_walk(&cell.car.borrow(), id),
                tree_walk(&cell.cdr.borrow(), id),
            ));
        }
        exp.clone()
    }
    tree_walk(rule, rule_application_id)
}

/// The book's `rule?`.
#[must_use]
pub fn is_rule(statement: &Value) -> bool {
    matches!(statement, Value::Pair(cell)
        if matches!(&*cell.car.borrow(), Value::Sym(s) if &**s == "rule"))
}

/// The book's `conclusion`: the `cadr` of a rule.
/// # Panics
/// Panics on a rule too short to have a conclusion, which only a broken
/// program causes.
#[must_use]
pub fn conclusion(rule: &Value) -> Value {
    let Value::Pair(cell) = rule else {
        return Value::Nil;
    };
    let Value::Pair(rest) = cell.cdr.borrow().clone() else {
        return Value::Nil;
    };
    rest.car.borrow().clone()
}

/// The book's `rule-body`: the `caddr`, or `(always-true)` for a rule
/// without a body.
/// # Panics
/// Panics on a rule too short to have a body slot, which only a broken
/// program causes.
#[must_use]
pub fn rule_body(rule: &Value) -> Value {
    let Value::Pair(cell) = rule else {
        return Value::list(vec![Value::sym("always-true")]);
    };
    let Value::Pair(rest) = cell.cdr.borrow().clone() else {
        return Value::list(vec![Value::sym("always-true")]);
    };
    match rest.cdr.borrow().clone() {
        Value::Pair(tail) => tail.car.borrow().clone(),
        _ => Value::list(vec![Value::sym("always-true")]),
    }
}

/// The book's `assertion-to-be-added?`: a tagged `assert!` form.
#[must_use]
pub fn assertion_to_be_added(exp: &Value) -> bool {
    matches!(exp, Value::Pair(cell)
        if matches!(&*cell.car.borrow(), Value::Sym(s) if &**s == "assert!"))
}

/// The book's `add-assertion-body`: the datum inside an `assert!` form.
#[must_use]
pub fn add_assertion_body(exp: &Value) -> Value {
    match exp {
        Value::Pair(cell) => match &*cell.cdr.borrow() {
            Value::Pair(rest) => rest.car.borrow().clone(),
            tail => tail.clone(),
        },
        other => other.clone(),
    }
}

// ---------------------------------------------------------------------------
// The matcher (4.4.4.3).
// ---------------------------------------------------------------------------

/// The book's `pattern-match`: `None` is the book's `failed`, never a
/// value any frame carries.
#[must_use]
pub fn pattern_match(pat: &Value, dat: &Value, frame: &Frame) -> Option<Frame> {
    if pat == dat {
        return Some(frame.clone());
    }
    if is_var(pat) {
        return extend_if_consistent(pat, dat, frame);
    }
    if let (Value::Pair(p), Value::Pair(d)) = (pat, dat) {
        let car_frame = pattern_match(&p.car.borrow(), &d.car.borrow(), frame)?;
        return pattern_match(&p.cdr.borrow(), &d.cdr.borrow(), &car_frame);
    }
    None
}

/// The book's `extend-if-consistent`: a bound variable re-matches its
/// stored value (which unification may have left a pattern), an unbound
/// one takes the datum.
#[must_use]
pub fn extend_if_consistent(var: &Value, dat: &Value, frame: &Frame) -> Option<Frame> {
    match frame.binding_in_frame(var) {
        Some(stored) => pattern_match(&stored, dat, frame),
        None => Some(frame.extend(var.clone(), dat.clone())),
    }
}

// ---------------------------------------------------------------------------
// Unification (4.4.4.4).
// ---------------------------------------------------------------------------

/// The book's `unify-match`: the matcher made symmetrical — variables on
/// both sides.
#[must_use]
pub fn unify_match(p1: &Value, p2: &Value, frame: &Frame) -> Option<Frame> {
    if p1 == p2 {
        return Some(frame.clone());
    }
    if is_var(p1) {
        return extend_if_possible(p1, p2, frame);
    }
    if is_var(p2) {
        return extend_if_possible(p2, p1, frame);
    }
    if let (Value::Pair(a), Value::Pair(b)) = (p1, p2) {
        let car_frame = unify_match(&a.car.borrow(), &b.car.borrow(), frame)?;
        return unify_match(&a.cdr.borrow(), &b.cdr.borrow(), &car_frame);
    }
    None
}

/// The book's `extend-if-possible`, with the two `***` checks: a value
/// that is itself a variable is looked through first, and a binding that
/// would make an expression depend on itself is rejected.
#[must_use]
pub fn extend_if_possible(var: &Value, val: &Value, frame: &Frame) -> Option<Frame> {
    if let Some(stored) = frame.binding_in_frame(var) {
        return unify_match(&stored, val, frame);
    }
    if is_var(val) {
        return match frame.binding_in_frame(val) {
            Some(stored) => unify_match(var, &stored, frame),
            None => Some(frame.extend(var.clone(), val.clone())),
        };
    }
    if depends_on(val, var, frame) {
        return None;
    }
    Some(frame.extend(var.clone(), val.clone()))
}

/// The book's `depends-on?`: whether `exp`, under `frame`'s bindings,
/// mentions `var`.
#[must_use]
pub fn depends_on(exp: &Value, var: &Value, frame: &Frame) -> bool {
    fn tree_walk(e: &Value, var: &Value, frame: &Frame) -> bool {
        if is_var(e) {
            if e == var {
                return true;
            }
            return frame
                .binding_in_frame(e)
                .is_some_and(|value| tree_walk(&value, var, frame));
        }
        if let Value::Pair(cell) = e {
            return tree_walk(&cell.car.borrow(), var, frame)
                || tree_walk(&cell.cdr.borrow(), var, frame);
        }
        false
    }
    tree_walk(exp, var, frame)
}

/// The book's `instantiate`: a copy of `exp` with every variable replaced
/// by its frame value; unbound variables go to `unbound`.
#[must_use]
pub fn instantiate(exp: &Value, frame: &Frame, unbound: &dyn Fn(&Value) -> Value) -> Value {
    fn copy(exp: &Value, frame: &Frame, unbound: &dyn Fn(&Value) -> Value) -> Value {
        if is_var(exp) {
            return match frame.binding_in_frame(exp) {
                Some(value) => copy(&value, frame, unbound),
                None => unbound(exp),
            };
        }
        if let Value::Pair(cell) = exp {
            let car = copy(&cell.car.borrow(), frame, unbound);
            let cdr = copy(&cell.cdr.borrow(), frame, unbound);
            return Value::Pair(cons_cell(car, cdr));
        }
        exp.clone()
    }
    copy(exp, frame, unbound)
}

// ---------------------------------------------------------------------------
// Stream operations (4.4.4.6).
// ---------------------------------------------------------------------------

/// The book's `singleton-stream`.
#[must_use]
pub fn singleton_stream(x: Frame) -> Stream<Frame> {
    Stream::cons_stream(x, || Stream::Empty)
}

/// The chapter 3 `stream-map`, lifting a step over a frame stream.
#[must_use]
pub fn stream_map<T, U>(f: Rc<dyn Fn(T) -> U>, s: Stream<T>) -> Stream<U>
where
    T: Clone + 'static,
    U: Clone + 'static,
{
    if s.is_empty() {
        return Stream::Empty;
    }
    let head = f(s.head().clone());
    let rest = s;
    Stream::cons_stream(head, move || stream_map(Rc::clone(&f), rest.tail()))
}

/// The chapter 3 `stream-append`: both operands are values before the
/// first element is produced, which is exactly the eagerness the book's
/// 4.71 listings remove.
#[must_use]
pub fn stream_append<T: Clone + 'static>(s1: Stream<T>, s2: Stream<T>) -> Stream<T> {
    if s1.is_empty() {
        return s2;
    }
    let head = s1.head().clone();
    let rest = s1;
    Stream::cons_stream(head, move || stream_append(rest.tail(), s2.clone()))
}

/// The chapter 3 `interleave`: alternates, swapping the operands.
#[must_use]
pub fn interleave<T: Clone + 'static>(s1: Stream<T>, s2: Stream<T>) -> Stream<T> {
    if s1.is_empty() {
        return s2;
    }
    let head = s1.head().clone();
    let rest = s1;
    Stream::cons_stream(head, move || interleave(s2.clone(), rest.tail()))
}

/// The book's `delay`ed stream argument: a thunk, the book's delayed
/// data. Type-erased so the recursive combinations share one shape.
pub type Thunk<B> = Box<dyn FnOnce() -> Stream<B>>;

/// The book's `stream-append-delayed`: `s2`'s construction waits until the
/// append walks past `s1`'s elements.
pub fn stream_append_delayed<B: Clone + 'static>(s1: Stream<B>, delayed_s2: Thunk<B>) -> Stream<B> {
    if s1.is_empty() {
        return delayed_s2();
    }
    let head = s1.head().clone();
    let rest = s1;
    Stream::cons_stream(head, move || stream_append_delayed(rest.tail(), delayed_s2))
}

/// The book's `interleave-delayed`: forces `delayed-s2` only after `s1`'s
/// first element, which is what keeps recursive rules answerable.
pub fn interleave_delayed<B: Clone + 'static>(s1: Stream<B>, delayed_s2: Thunk<B>) -> Stream<B> {
    if s1.is_empty() {
        return delayed_s2();
    }
    let head = s1.head().clone();
    let rest = s1;
    Stream::cons_stream(head, move || {
        interleave_delayed(delayed_s2(), Box::new(move || rest.tail()))
    })
}

/// The book's `flatten-stream`: combines the mapped inner streams with
/// `interleave-delayed`, the `delay` guarding the rest of the walk.
#[must_use]
pub fn flatten_stream<B: Clone + 'static>(stream: Stream<Stream<B>>) -> Stream<B> {
    if stream.is_empty() {
        return Stream::Empty;
    }
    let inner = stream.head().clone();
    let rest = stream;
    interleave_delayed(inner, Box::new(move || flatten_stream(rest.tail())))
}

/// The book's `stream-flatmap`: map, then interleave the inner streams.
#[must_use]
pub fn stream_flatmap<A: Clone + 'static, B: Clone + 'static>(
    proc: StepFn<A, B>,
    s: Stream<A>,
) -> Stream<B> {
    flatten_stream(stream_map(Rc::new(move |a: A| proc(&a)), s))
}

/// The book's undelayed `flatten-stream` of exercise 4.73, for the
/// comparison probes: it must construct every inner stream before the
/// first element emerges.
#[must_use]
pub fn flatten_stream_undelayed<B: Clone + 'static>(stream: Stream<Stream<B>>) -> Stream<B> {
    if stream.is_empty() {
        return Stream::Empty;
    }
    let inner = stream.head().clone();
    let rest = stream;
    interleave(inner, flatten_stream_undelayed(rest.tail()))
}

// ---------------------------------------------------------------------------
// The engine.
// ---------------------------------------------------------------------------

/// The book's `get-stream`: the stored bucket as a stream.
fn get_bucket(index: &RefCell<HashMap<String, Vec<Value>>>, pattern: &Value) -> Stream<Value> {
    stream_of_values(&get_bucket_items(index, pattern))
}

/// The index bucket for `pattern`, if any.
fn get_bucket_items(index: &RefCell<HashMap<String, Vec<Value>>>, pattern: &Value) -> Vec<Value> {
    match index_key_of(pattern) {
        Some(key) => index.borrow().get(&key).cloned().unwrap_or_default(),
        None => Vec::new(),
    }
}

/// Builds a proper-list stream from stored items, in order.
fn stream_of_values(items: &[Value]) -> Stream<Value> {
    fn build(items: Rc<[Value]>, index: usize) -> Stream<Value> {
        match items.get(index) {
            None => Stream::Empty,
            Some(head) => {
                let head = head.clone();
                Stream::cons_stream(head, move || build(Rc::clone(&items), index + 1))
            }
        }
    }
    build(Rc::from(items), 0)
}

/// The query engine: the data base, the dispatch table, the rule counter,
/// and the host procedures `lisp-value` applies — the book's driver loop,
/// `qeval` dispatch, and table of 4.4.4.5 in one structure.
pub struct QueryEngine {
    /// The `(type 'qeval)` table of 4.4.4.2, keyed by the type symbol.
    procs: RefCell<HashMap<String, QProc>>,
    /// The processor untagged patterns take; `None` is the standard
    /// simple query of 4.4.4.2. Exercises that redefine simple query
    /// install theirs here.
    fallback: RefCell<Option<QProc>>,
    /// The frame combinator, the book's `stream-flatmap`; 4.74 swaps it.
    combiner: RefCell<Rc<dyn Combiner>>,
    /// All assertions, in insertion order (see the module docs).
    assertions: RefCell<Vec<Value>>,
    /// All rules, in insertion order.
    rules: RefCell<Vec<Value>>,
    /// The assertion index, keyed by the leading symbol or `?`.
    assertion_index: RefCell<HashMap<String, Vec<Value>>>,
    /// The rule index, keyed by the conclusion's leading symbol or `?`.
    rule_index: RefCell<HashMap<String, Vec<Value>>>,
    /// The book's `rule-counter`, one id per rule application.
    rule_counter: std::cell::Cell<u64>,
    /// The host procedures `lisp-value` applies: the shared subset's
    /// primitives, the edition's stand-in for the underlying Lisp.
    predicates: RefCell<HashMap<String, Handler>>,
    /// The slot a raised object error lands in, read by the driver.
    error: RefCell<Option<SchemeError>>,
}

impl QueryEngine {
    /// One engine with the standard dispatch table installed: `and`,
    /// `or`, `not`, `lisp-value`, and `always-true` (the book's five
    /// `put`s).
    #[must_use]
    pub fn new() -> Engine {
        let (sink, _cell) = OutputSink::buffer();
        let predicates: HashMap<String, Handler> = primitive_table(&sink)
            .into_iter()
            .map(|(name, handler)| (name.to_string(), handler))
            .collect();
        let engine = Rc::new(QueryEngine {
            procs: RefCell::new(HashMap::new()),
            fallback: RefCell::new(None),
            combiner: RefCell::new(Rc::new(Interleaved)),
            assertions: RefCell::new(Vec::new()),
            rules: RefCell::new(Vec::new()),
            assertion_index: RefCell::new(HashMap::new()),
            rule_index: RefCell::new(HashMap::new()),
            rule_counter: std::cell::Cell::new(0),
            predicates: RefCell::new(predicates),
            error: RefCell::new(None),
        });
        engine.put("and", conjoin_proc());
        engine.put("or", disjoin_proc());
        engine.put("not", negate_proc());
        engine.put("lisp-value", lisp_value_proc());
        engine.put("always-true", always_true_proc());
        engine
    }

    /// The book's `put`: registers the processor for one type symbol.
    pub fn put(&self, name: &str, proc: QProc) {
        self.procs.borrow_mut().insert(name.to_string(), proc);
    }

    /// The book's `get`: the installed processor for one type symbol.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<QProc> {
        self.procs.borrow().get(name).cloned()
    }

    /// Swaps the simple-query fallback (see the module docs).
    pub fn set_fallback(&self, proc: Option<QProc>) {
        *self.fallback.borrow_mut() = proc;
    }

    /// The standard simple-query processor, as a table-shaped value the
    /// wrappers of 4.67 and 4.79 delegate to.
    #[must_use]
    pub fn simple_query_proc(self: &Rc<Self>) -> QProc {
        let engine = Rc::clone(self);
        Rc::new(move |_eng, pattern, frames| engine.standard_simple_query(pattern, frames))
    }

    /// Swaps the frame combinator (4.74's simple flatmap).
    pub fn set_combiner(&self, combiner: Rc<dyn Combiner>) {
        *self.combiner.borrow_mut() = combiner;
    }

    /// Runs the engine's combinator over frames.
    pub fn flatmap(&self, proc: StepFn<Frame, Frame>, s: Stream<Frame>) -> Stream<Frame> {
        self.combiner.borrow().combine_frames(proc, s)
    }

    /// Runs the engine's combinator over data values, `find-assertions`'s
    /// shape; `apply-rules` stays on the book combinator (4.74).
    pub fn flatmap_values(&self, proc: StepFn<Value, Frame>, s: Stream<Value>) -> Stream<Frame> {
        self.combiner.borrow().combine_values(proc, s)
    }

    /// Installs a host predicate `lisp-value` can apply, beside the
    /// shared subset's primitives (4.60's name comparison).
    pub fn install_predicate(&self, name: &str, handler: Handler) {
        self.predicates
            .borrow_mut()
            .insert(name.to_string(), handler);
    }

    /// One rule-application id, the book's `new-rule-application-id`.
    fn next_rule_id(&self) -> u64 {
        self.rule_counter.set(self.rule_counter.get() + 1);
        self.rule_counter.get()
    }

    /// The error a raised object error left, if any, clearing the slot.
    #[must_use]
    pub fn take_error(&self) -> Option<SchemeError> {
        self.error.borrow_mut().take()
    }

    /// Records a raised object error for the driver.
    fn raise(&self, error: SchemeError) {
        *self.error.borrow_mut() = Some(error);
    }

    // -- The data base (4.4.4.5). ------------------------------------------

    /// The book's `add-rule-or-assertion!`, filing by shape.
    pub fn add_rule_or_assertion(&self, assertion: &Value) {
        if is_rule(assertion) {
            self.add_rule(assertion);
        } else {
            self.add_assertion(assertion);
        }
    }

    /// Reads and files each line of a program of `assert!`-free data:
    /// every form is an assertion or a rule. Parse errors panic; the
    /// callers feed checked constants.
    /// # Panics
    /// Panics when a line does not parse, which only a broken constant
    /// table causes.
    pub fn load(&self, lines: &[&str]) {
        for line in lines {
            let form = sicp_runtime::read(line).expect("the data base line parses");
            self.add_rule_or_assertion(&query_syntax_process(&form));
        }
    }

    fn add_assertion(&self, assertion: &Value) {
        self.store_assertion_in_index(assertion);
        self.assertions.borrow_mut().push(assertion.clone());
    }

    fn add_rule(&self, rule: &Value) {
        self.store_rule_in_index(rule);
        self.rules.borrow_mut().push(rule.clone());
    }

    fn store_assertion_in_index(&self, assertion: &Value) {
        if let Some(key) = index_key_of(assertion) {
            self.assertion_index
                .borrow_mut()
                .entry(key)
                .or_default()
                .push(assertion.clone());
        }
    }

    fn store_rule_in_index(&self, rule: &Value) {
        if let Some(key) = index_key_of(&conclusion(rule)) {
            self.rule_index
                .borrow_mut()
                .entry(key)
                .or_default()
                .push(rule.clone());
        }
    }

    /// The book's `fetch-assertions`: the indexed bucket when the pattern
    /// starts with a constant symbol, everything otherwise.
    #[must_use]
    pub fn fetch_assertions(&self, pattern: &Value) -> Stream<Value> {
        if use_index(pattern) {
            get_bucket(&self.assertion_index, pattern)
        } else {
            stream_of_values(&self.assertions.borrow())
        }
    }

    /// The book's `fetch-rules`: the bucket for the pattern's leading
    /// symbol plus the `?` bucket of variable-led conclusions.
    #[must_use]
    pub fn fetch_rules(&self, pattern: &Value) -> Stream<Value> {
        if use_index(pattern) {
            let mut items = get_bucket_items(&self.rule_index, pattern);
            items.extend(get_bucket_items(&self.rule_index, &var_key()));
            stream_of_values(&items)
        } else {
            stream_of_values(&self.rules.borrow())
        }
    }

    // -- The evaluator (4.4.4.2). ------------------------------------------

    /// The book's `qeval`: dispatch on the type symbol through the table,
    /// then the fallback, then the standard simple query. Engine-recursive
    /// helpers close over [`Engine`] handles because stream tails must be
    /// `'static`.
    pub fn qeval(self: &Rc<Self>, query: &Value, frames: Stream<Frame>) -> Stream<Frame> {
        #[allow(
            clippy::collapsible_if,
            reason = "the three dispatch stages read best nested"
        )]
        {
            if let Value::Pair(cell) = query {
                if let Value::Sym(name) = &*cell.car.borrow() {
                    if let Some(proc) = self.get(name) {
                        let contents = cell.cdr.borrow().clone();
                        return proc(self, &contents, frames);
                    }
                }
            }
        }
        if let Some(fallback) = self.fallback.borrow().clone() {
            return fallback(self, query, frames);
        }
        self.standard_simple_query(query, frames)
    }

    /// The book's `simple-query`: for each frame, the assertion matches
    /// then the rule applications, the assertions' stream first and the
    /// rules' construction delayed behind it.
    #[must_use]
    pub fn standard_simple_query(
        self: &Rc<Self>,
        query_pattern: &Value,
        frame_stream: Stream<Frame>,
    ) -> Stream<Frame> {
        let engine = Rc::clone(self);
        let pattern = query_pattern.clone();
        self.flatmap(
            Rc::new(move |frame: &Frame| {
                let frame = frame.clone();
                let engine2 = Rc::clone(&engine);
                let pattern2 = pattern.clone();
                stream_append_delayed(
                    engine.find_assertions_in(&pattern, &frame),
                    Box::new(move || engine2.apply_rules(&pattern2, &frame)),
                )
            }),
            frame_stream,
        )
    }

    /// The book's `find-assertions`: every data-base match of the pattern
    /// in the frame.
    #[must_use]
    pub fn find_assertions_in(self: &Rc<Self>, pattern: &Value, frame: &Frame) -> Stream<Frame> {
        let engine = Rc::clone(self);
        let pattern = pattern.clone();
        let frame = frame.clone();
        let assertions = engine.fetch_assertions(&pattern);
        self.flatmap_values(
            Rc::new(
                move |datum: &Value| match check_an_assertion(datum, &pattern, &frame) {
                    Some(extended) => singleton_stream(extended),
                    None => Stream::Empty,
                },
            ),
            assertions,
        )
    }

    /// The book's `apply-rules` over the fetched rules. The book composes
    /// it with the plain `stream-flatmap`: a rule application's inner
    /// stream is the rule body's whole answer stream, neither empty nor
    /// singleton, so 4.74's simple combiner must not serve it (it would
    /// keep only the first frame per rule and its eager collection
    /// diverges on recursive rules).
    #[must_use]
    pub fn apply_rules(self: &Rc<Self>, pattern: &Value, frame: &Frame) -> Stream<Frame> {
        let engine = Rc::clone(self);
        let pattern = pattern.clone();
        let frame = frame.clone();
        let rules = engine.fetch_rules(&pattern);
        Interleaved.combine_values(
            Rc::new(move |rule: &Value| engine.apply_a_rule(rule, &pattern, &frame)),
            rules,
        )
    }

    /// The book's `apply-a-rule`: rename, unify the conclusion with the
    /// pattern in the frame, evaluate the body in the extension.
    #[must_use]
    pub fn apply_a_rule(
        self: &Rc<Self>,
        rule: &Value,
        query_pattern: &Value,
        query_frame: &Frame,
    ) -> Stream<Frame> {
        let clean_rule = rename_variables_in(rule, self.next_rule_id());
        match unify_match(query_pattern, &conclusion(&clean_rule), query_frame) {
            None => Stream::Empty,
            Some(unify_result) => {
                self.qeval(&rule_body(&clean_rule), singleton_stream(unify_result))
            }
        }
    }

    /// The book's `execute`: applies the named host predicate to the
    /// already-actual arguments.
    ///
    /// # Errors
    /// [`SchemeError::UnboundVariable`] when the predicate names no host
    /// procedure; whatever the procedure raises.
    #[allow(
        clippy::similar_names,
        reason = "call/cell are the call form and its cell, one field apart"
    )]
    pub fn execute(&self, call: &Value) -> EvalResult {
        let Value::Pair(cell) = call else {
            return Err(SchemeError::TypeMismatch(format!(
                "lisp-value: not a call: {call}"
            )));
        };
        let Value::Sym(name) = &*cell.car.borrow() else {
            return Err(SchemeError::TypeMismatch(format!(
                "lisp-value: no predicate: {call}"
            )));
        };
        let handler = self
            .predicates
            .borrow()
            .get(&**name)
            .cloned()
            .ok_or_else(|| SchemeError::UnboundVariable(name.to_string()))?;
        let args = cell.cdr.borrow().list_items()?;
        handler(&args)
    }

    // -- The driver loop (4.4.4.1). ----------------------------------------

    /// The frame stream one query produces, from the singleton empty
    /// frame — the [`qeval`] entry the driver uses.
    #[must_use]
    pub fn query_frames(self: &Rc<Self>, query: &Value) -> Stream<Frame> {
        self.qeval(query, singleton_stream(Frame::new()))
    }

    fn answer_stream(self: &Rc<Self>, processed: &Value) -> Vec<String> {
        let frames = self.query_frames(processed);
        let mut out = Vec::new();
        for frame in &frames {
            out.push(print_value(&instantiate_query(processed, &frame)));
        }
        out
    }

    /// Every answer of a query string, instantiated and printed. The
    /// query must terminate; sample infinite streams with
    /// [`QueryEngine::answers_upto`].
    ///
    /// # Panics
    /// Panics when the query does not parse, which the exercise strings
    /// never do.
    #[must_use]
    pub fn answers(self: &Rc<Self>, query: &str) -> Vec<String> {
        self.answers_from(&read_query(query), usize::MAX).0
    }

    /// The first `n` answers of a query string, for the infinite streams
    /// recursive rules generate.
    /// # Panics
    /// As [`QueryEngine::answers`].
    #[must_use]
    pub fn answers_upto(self: &Rc<Self>, query: &str, n: usize) -> Vec<String> {
        self.answers_from(&read_query(query), n).0
    }

    /// [`QueryEngine::answers_upto`] that also reports a raised error.
    /// # Panics
    /// As [`QueryEngine::answers`].
    #[must_use]
    pub fn answers_checked(
        self: &Rc<Self>,
        query: &str,
        n: usize,
    ) -> (Vec<String>, Option<SchemeError>) {
        self.answers_from(&read_query(query), n)
    }

    fn answers_from(
        self: &Rc<Self>,
        processed: &Value,
        n: usize,
    ) -> (Vec<String>, Option<SchemeError>) {
        let frames = self.query_frames(processed);
        let mut out = Vec::new();
        for frame in &frames {
            out.push(print_value(&instantiate_query(processed, &frame)));
            if out.len() >= n {
                return (out, self.take_error());
            }
        }
        (out, self.take_error())
    }

    /// The book's driver session over `lines`: one `;;; Query input:`
    /// line each, `Assertion added to data base.` after a filing, and the
    /// `;;; Query results:` block with each instantiated answer; a raised
    /// object error or a parse error ends the transcript with one `Error:`
    /// line. Only terminating queries belong here.
    ///
    /// # Panics
    /// Panics only through [`QueryEngine::answers`]'s parse rule, which
    /// the exercise strings never trip.
    #[must_use]
    pub fn session(self: &Rc<Self>, lines: &[&str]) -> String {
        let mut out = String::new();
        for line in lines {
            out.push_str(";;; Query input: ");
            out.push_str(line);
            out.push('\n');
            let form = match sicp_runtime::read(line) {
                Ok(form) => form,
                Err(error) => {
                    out.push_str("Error: ");
                    out.push_str(&error.to_string());
                    out.push('\n');
                    return out;
                }
            };
            let processed = query_syntax_process(&form);
            if assertion_to_be_added(&processed) {
                let body = add_assertion_body(&processed);
                self.add_rule_or_assertion(&body);
                out.push_str("Assertion added to data base.\n");
                continue;
            }
            out.push_str(";;; Query results:\n");
            for answer in self.answer_stream(&processed) {
                out.push_str(&answer);
                out.push('\n');
            }
            if let Some(error) = self.take_error() {
                out.push_str("Error: ");
                out.push_str(&error.to_string());
                out.push('\n');
                return out;
            }
        }
        out
    }
}

/// The book's `instantiate` with the driver's unbound-variable handler:
/// a printed `?name`, the book's `contract-question-mark`.
#[must_use]
pub fn instantiate_query(pattern: &Value, frame: &Frame) -> Value {
    instantiate(pattern, frame, &|var| contract_question_mark(var))
}

/// Reads one query and expands its pattern variables.
/// # Panics
/// Panics on a parse error, which the exercise strings never contain.
#[must_use]
pub fn read_query(query: &str) -> Value {
    let form = sicp_runtime::read(query).expect("the query parses");
    query_syntax_process(&form)
}

/// The `?` key the rule index stores variable-led conclusions under.
fn var_key() -> Value {
    Value::sym("?")
}

/// The book's `indexable?`: a pattern starts with a variable or a
/// constant symbol.
#[must_use]
pub fn is_indexable(pat: &Value) -> bool {
    match pat {
        Value::Pair(cell) => {
            let head = cell.car.borrow();
            is_var(&head) || constant_symbol(&head)
        }
        _ => false,
    }
}

/// The book's `index-key-of`: `?` for a variable-led pattern, the leading
/// symbol otherwise; `None` when the pattern is not indexable.
#[must_use]
pub fn index_key_of(pat: &Value) -> Option<String> {
    if !is_indexable(pat) {
        return None;
    }
    let Value::Pair(cell) = pat else { return None };
    let head = cell.car.borrow();
    if is_var(&head) {
        return Some("?".to_string());
    }
    match &*head {
        Value::Sym(s) => Some(s.to_string()),
        _ => None,
    }
}

/// The book's `use-index?`: only constant-led patterns fetch by index.
#[must_use]
pub fn use_index(pat: &Value) -> bool {
    match pat {
        Value::Pair(cell) => constant_symbol(&cell.car.borrow()),
        _ => false,
    }
}

/// The book's `check-an-assertion`: the match's extension, or no frame.
#[must_use]
pub fn check_an_assertion(
    assertion: &Value,
    query_pat: &Value,
    query_frame: &Frame,
) -> Option<Frame> {
    pattern_match(query_pat, assertion, query_frame)
}

// ---------------------------------------------------------------------------
// The standard processors: the book's five `put`s plus the compound
// helpers, as table-shaped values.
// ---------------------------------------------------------------------------

/// The book's `conjoin`: each conjunct filters the previous one's frames.
#[must_use]
pub fn conjoin_proc() -> QProc {
    Rc::new(conjoin)
}

/// The book's `conjoin` body, callable by the merging `and` of 4.76.
pub fn conjoin(engine: &Engine, conjuncts: &Value, frames: Stream<Frame>) -> Stream<Frame> {
    if conjuncts.is_nil() {
        return frames;
    }
    let (first, rest) = split_list(conjuncts);
    conjoin(engine, &rest, qeval_engine(engine, &first, frames))
}

/// The book's `disjoin`: the disjuncts' streams merged with
/// `interleave-delayed`.
#[must_use]
pub fn disjoin_proc() -> QProc {
    Rc::new(disjoin)
}

/// The book's `disjoin` body, callable by the appending `or` of 4.72.
pub fn disjoin(engine: &Engine, disjuncts: &Value, frames: Stream<Frame>) -> Stream<Frame> {
    if disjuncts.is_nil() {
        return Stream::Empty;
    }
    let (first, rest) = split_list(disjuncts);
    let engine2 = Rc::clone(engine);
    let frames2 = frames.clone();
    interleave_delayed(
        qeval_engine(engine, &first, frames),
        Box::new(move || disjoin(&engine2, &rest, frames2)),
    )
}

/// The book's `negate`: keeps the frames the negated query cannot extend.
#[must_use]
pub fn negate_proc() -> QProc {
    Rc::new(|engine, operands, frames| {
        let (negated, _) = split_list(operands);
        let engine2 = Rc::clone(engine);
        let negated = negated.clone();
        engine.flatmap(
            Rc::new(move |frame: &Frame| {
                if engine2
                    .qeval(&negated, singleton_stream(frame.clone()))
                    .is_empty()
                {
                    singleton_stream(frame.clone())
                } else {
                    Stream::Empty
                }
            }),
            frames,
        )
    })
}

/// The book's `lisp-value`: keeps the frames whose instantiated call
/// holds; an unbound pattern variable raises (the engine's error slot).
#[must_use]
pub fn lisp_value_proc() -> QProc {
    Rc::new(|engine, call, frames| {
        let engine2 = Rc::clone(engine);
        let call = call.clone();
        engine.flatmap(
            Rc::new(move |frame: &Frame| {
                let instantiated = instantiate(&call, frame, &|var: &Value| {
                    engine2.raise(SchemeError::TypeMismatch(format!(
                        "Unknown pat var -- LISP-VALUE: {}",
                        contract_question_mark(var)
                    )));
                    contract_question_mark(var)
                });
                let holds = engine2.execute(&instantiated).is_ok_and(|v| is_true(&v));
                if holds {
                    singleton_stream(frame.clone())
                } else {
                    Stream::Empty
                }
            }),
            frames,
        )
    })
}

/// The book's `always-true`: every frame passes.
#[must_use]
pub fn always_true_proc() -> QProc {
    Rc::new(|_engine, _ignore, frames| frames)
}

/// [`QueryEngine::qeval`] over an [`Engine`] handle, for the helpers the
/// engine recursion threads.
pub fn qeval_engine(engine: &Engine, query: &Value, frames: Stream<Frame>) -> Stream<Frame> {
    engine.qeval(query, frames)
}

/// Splits a proper list value into its head and tail.
/// # Panics
/// Panics on an atom where a list is required, which only a broken
/// program causes.
#[must_use]
pub fn split_list(exps: &Value) -> (Value, Value) {
    match exps {
        Value::Pair(cell) => (cell.car.borrow().clone(), cell.cdr.borrow().clone()),
        other => (other.clone(), Value::Nil),
    }
}

// ---------------------------------------------------------------------------
// The Microshaft data base of 4.4.1.
// ---------------------------------------------------------------------------

/// The personnel data base of 4.4.1, exactly as the book lists it.
pub const MICROSHAFT: &[&str] = &[
    "(address (Bitdiddle Ben) (Slumerville (Ridge Road) 10))",
    "(job (Bitdiddle Ben) (computer wizard))",
    "(salary (Bitdiddle Ben) 60000)",
    "(address (Hacker Alyssa P) (Cambridge (Mass Ave) 78))",
    "(job (Hacker Alyssa P) (computer programmer))",
    "(salary (Hacker Alyssa P) 40000)",
    "(supervisor (Hacker Alyssa P) (Bitdiddle Ben))",
    "(address (Fect Cy D) (Cambridge (Ames Street) 3))",
    "(job (Fect Cy D) (computer programmer))",
    "(salary (Fect Cy D) 35000)",
    "(supervisor (Fect Cy D) (Bitdiddle Ben))",
    "(address (Tweakit Lem E) (Boston (Bay State Road) 22))",
    "(job (Tweakit Lem E) (computer technician))",
    "(salary (Tweakit Lem E) 25000)",
    "(supervisor (Tweakit Lem E) (Bitdiddle Ben))",
    "(address (Reasoner Louis) (Slumerville (Pine Tree Road) 80))",
    "(job (Reasoner Louis) (computer programmer trainee))",
    "(salary (Reasoner Louis) 30000)",
    "(supervisor (Reasoner Louis) (Hacker Alyssa P))",
    "(supervisor (Bitdiddle Ben) (Warbucks Oliver))",
    "(address (Warbucks Oliver) (Swellesley (Top Heap Road)))",
    "(job (Warbucks Oliver) (administration big wheel))",
    "(salary (Warbucks Oliver) 150000)",
    "(address (Scrooge Eben) (Weston (Shady Lane) 10))",
    "(job (Scrooge Eben) (accounting chief accountant))",
    "(salary (Scrooge Eben) 75000)",
    "(supervisor (Scrooge Eben) (Warbucks Oliver))",
    "(address (Cratchet Robert) (Allston (N Harvard Street) 16))",
    "(job (Cratchet Robert) (accounting scrivener))",
    "(salary (Cratchet Robert) 18000)",
    "(supervisor (Cratchet Robert) (Scrooge Eben))",
    "(address (Aull DeWitt) (Slumerville (Onion Square) 5))",
    "(job (Aull DeWitt) (administration secretary))",
    "(salary (Aull DeWitt) 25000)",
    "(supervisor (Aull DeWitt) (Warbucks Oliver))",
    "(can-do-job (computer wizard) (computer programmer))",
    "(can-do-job (computer wizard) (computer technician))",
    "(can-do-job (computer programmer) (computer programmer trainee))",
    "(can-do-job (administration secretary) (administration big wheel))",
];

/// One engine over the 4.4.1 data base: the shared fixture every
/// Microshaft exercise and example runs on.
#[must_use]
pub fn microshaft() -> Engine {
    let engine = QueryEngine::new();
    engine.load(MICROSHAFT);
    engine
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(text: &str) -> Value {
        query_syntax_process(&sicp_runtime::read(text).expect("parses"))
    }

    fn frame_of(pairs: &[(&str, &str)]) -> Frame {
        let mut frame = Frame::new();
        for (var, value) in pairs {
            let var = q(var);
            let value = q(value);
            frame = frame.extend(var, value);
        }
        frame
    }

    #[test]
    fn pattern_match_examples_agree_with_the_book() {
        let data = q("((a b) c (a b))");
        let hit = pattern_match(&q("(?x c ?x)"), &data, &Frame::new());
        assert_eq!(
            hit.as_ref().map(|f| instantiate_query(&q("(?x c ?x)"), f)),
            Some(q("((a b) c (a b))"))
        );
        assert!(pattern_match(&q("(?x a ?y)"), &data, &Frame::new()).is_none());
        // ?y bound to b, ?x free: the frame gains only ?x.
        let frame = frame_of(&[("?y", "b")]);
        let extended = pattern_match(&q("(?x ?y ?x)"), &q("(a b a)"), &frame).expect("matches");
        assert_eq!(instantiate_query(&q("(?x ?y ?x)"), &extended), q("(a b a)"));
        // ?y bound to a fails.
        let frame = frame_of(&[("?y", "a")]);
        assert!(pattern_match(&q("(?x ?y ?x)"), &q("(a b a)"), &frame).is_none());
    }

    /// The frame's view of one variable, chased through bindings — the
    /// book presents unified variables as their final values.
    fn resolved(frame: &Frame, var: &str) -> Option<Value> {
        let var = &q(var);
        frame
            .binding_in_frame(var)
            .map(|value| instantiate(&value, frame, &|v| contract_question_mark(v)))
    }

    #[test]
    fn unification_examples_agree_with_the_book() {
        let frame = unify_match(&q("(?x a ?y)"), &q("(?y ?z a)"), &Frame::new()).expect("unifies");
        for var in ["?x", "?y", "?z"] {
            assert_eq!(resolved(&frame, var), Some(q("a")), "{var}");
        }
        assert!(unify_match(&q("(?x ?y a)"), &q("(?x b ?y)"), &Frame::new()).is_none());
        // (?x ?x) against ((a ?y c) (a b ?z)): ?x becomes (a b c).
        let frame =
            unify_match(&q("(?x ?x)"), &q("((a ?y c) (a b ?z))"), &Frame::new()).expect("unifies");
        assert_eq!(resolved(&frame, "?x"), Some(q("(a b c)")));
        // Partial: ?x is bound to a pattern still containing ?y.
        let frame = unify_match(&q("(?x a)"), &q("((b ?y) ?z)"), &Frame::new()).expect("unifies");
        assert_eq!(frame.binding_in_frame(&q("?x")), Some(q("(b ?y)")));
        assert_eq!(frame.binding_in_frame(&q("?y")), None);
        assert_eq!(resolved(&frame, "?z"), Some(q("a")));
        // A binding that would depend on itself is rejected.
        assert!(unify_match(&q("?x"), &q("(f ?x)"), &Frame::new()).is_none());
    }

    #[test]
    fn microshaft_answers_the_book_transcripts() {
        let engine = microshaft();
        assert_eq!(
            engine.answers("(job ?x (computer programmer))"),
            [
                "(job (Hacker Alyssa P) (computer programmer))",
                "(job (Fect Cy D) (computer programmer))",
            ]
        );
        assert_eq!(
            engine.answers("(job ?x (computer . ?type))"),
            [
                "(job (Bitdiddle Ben) (computer wizard))",
                "(job (Hacker Alyssa P) (computer programmer))",
                "(job (Fect Cy D) (computer programmer))",
                "(job (Tweakit Lem E) (computer technician))",
                "(job (Reasoner Louis) (computer programmer trainee))",
            ]
        );
    }

    #[test]
    fn rules_answer_through_the_index_and_the_engine() {
        let engine = microshaft();
        engine.load(&[
            "(rule (lives-near ?person-1 ?person-2) \
             (and (address ?person-1 (?town . ?rest-1)) \
             (address ?person-2 (?town . ?rest-2)) \
             (not (same ?person-1 ?person-2))))",
            "(rule (same ?x ?x))",
            "(rule (wheel ?person) \
             (and (supervisor ?middle-manager ?person) \
             (supervisor ?x ?middle-manager)))",
        ]);
        assert_eq!(
            engine.answers("(lives-near ?x (Bitdiddle Ben))"),
            [
                "(lives-near (Reasoner Louis) (Bitdiddle Ben))",
                "(lives-near (Aull DeWitt) (Bitdiddle Ben))",
            ]
        );
        assert_eq!(
            engine.answers("(wheel ?who)"),
            [
                "(wheel (Bitdiddle Ben))",
                "(wheel (Warbucks Oliver))",
                "(wheel (Warbucks Oliver))",
                "(wheel (Warbucks Oliver))",
                "(wheel (Warbucks Oliver))",
            ],
            "the book's listing in scan-order rows: Ben once, Warbucks four times"
        );
    }

    #[test]
    fn the_driver_transcript_matches_the_book_format() {
        let engine = microshaft();
        let session = engine.session(&[
            "(job ?x (computer programmer))",
            "(assert! (job (Bitdiddle Ben) (computer wizard)))",
        ]);
        assert!(session.contains(";;; Query input: (job ?x (computer programmer))\n;;; Query results:\n(job (Hacker Alyssa P) (computer programmer))\n(job (Fect Cy D) (computer programmer))\n"));
        assert!(session.contains("Assertion added to data base.\n"));
    }
}
