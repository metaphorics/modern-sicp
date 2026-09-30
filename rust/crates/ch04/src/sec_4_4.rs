// SPDX-License-Identifier: GPL-3.0-only
//
// Section 4.4: the query system over the explicit data language of
// grammar §7. Queries, terms, and substitutions are Rust values;
// unification carries its stated occurs-check policy (enabled); answers
// come from a delayed, interleaved stream, so recursive and infinite
// rules answer fairly and a finite prefix is observable (the 4.71/4.72
// fairness lessons). Ground queries answer their empty substitution.
// Rule applications rename their variables apart deterministically
// (`name#n`); `HashMap` is only an index and never decides answer
// order.

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

pub use sicp_runtime::host::query::{Predicate, Query, Substitution, Term, extend};

/// One rule: a conclusion and its conditions.
#[derive(Debug, Clone)]
pub struct Rule {
    /// The rule's conclusion pattern.
    pub conclusion: Term,
    /// The rule's conditions, in order.
    pub conditions: Vec<Query>,
}

/// The query database: chronological assertions and rules. Fetching
/// preserves insertion order, which is the production order of the
/// answer stream.
#[derive(Debug, Clone, Default)]
pub struct Database {
    /// The asserted terms, in insertion order.
    pub assertions: Vec<Term>,
    /// The rules, in insertion order.
    pub rules: Vec<Rule>,
}

impl Database {
    /// Builds an empty database.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Files one assertion.
    pub fn assert(&mut self, term: Term) {
        self.assertions.push(term);
    }

    /// Files one rule.
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
}

/// One query run's ordered answers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryOutcome {
    /// The answer substitutions, in production order; a ground query
    /// answers its empty substitution exactly once.
    pub answers: Vec<Substitution>,
}

/// The delayed answer stream: a first answer and a delayed rest, the
/// discipline of the section's `stream-append-delayed` and
/// `interleave-delayed`.
enum Stream {
    /// No further answers.
    Nil,
    /// One answer and a delayed rest.
    Cons(Substitution, Box<dyn FnOnce() -> Stream>),
    /// A computation not yet forced.
    Delay(Box<dyn FnOnce() -> Stream>),
}

/// Forces one layer of delay.
fn force(stream: Stream) -> Stream {
    match stream {
        Stream::Delay(thunk) => force(thunk()),
        other => other,
    }
}

/// Round-robin combination of two streams: the fairness discipline of
/// `interleave-delayed`.
fn interleave(left: Stream, right: Stream) -> Stream {
    match force(left) {
        Stream::Nil => right,
        Stream::Cons(answer, rest) => {
            Stream::Cons(answer, Box::new(move || interleave(right, rest())))
        }
        Stream::Delay(_) => unreachable!("force resolves delays"),
    }
}

/// Stream monadic bind with a delayed rest: the discipline of
/// `stream-flatmap-delayed`.
fn flatmap(stream: Stream, project: &Rc<dyn Fn(Substitution) -> Stream>) -> Stream {
    match force(stream) {
        Stream::Nil => Stream::Nil,
        Stream::Cons(answer, rest) => {
            let project = Rc::clone(project);
            interleave(
                project(answer),
                Stream::Delay(Box::new(move || flatmap(rest(), &project))),
            )
        }
        Stream::Delay(_) => unreachable!("force resolves delays"),
    }
}

/// Runs one query to its answer stream.
fn qeval_stream(engine: &Rc<Engine>, query: &Query, frame: Substitution) -> Stream {
    match query {
        Query::Unify(left, right) => match unify(left, right, &frame) {
            Some(extended) => Stream::Cons(extended, Box::new(|| Stream::Nil)),
            None => Stream::Nil,
        },
        Query::Value(predicate, arguments) => {
            if holds(predicate, arguments, &frame) {
                Stream::Cons(frame, Box::new(|| Stream::Nil))
            } else {
                Stream::Nil
            }
        }
        Query::Relation { name, arguments } => relation_stream(engine, name, arguments, &frame),
        Query::And(subs) => {
            let mut stream = Stream::Cons(frame, Box::new(|| Stream::Nil));
            for sub in subs {
                let sub = sub.clone();
                let engine = Rc::clone(engine);
                let project: Rc<dyn Fn(Substitution) -> Stream> =
                    Rc::new(move |frame| qeval_stream(&engine, &sub, frame));
                stream = flatmap(stream, &project);
            }
            stream
        }
        Query::Or(subs) => {
            // `disjoin`: the first disjunct's stream interleaved with
            // the delayed disjunction of the rest, in source order.
            let mut stream = Stream::Nil;
            for sub in subs.iter().rev() {
                let engine = Rc::clone(engine);
                let sub = sub.clone();
                let frame = frame.clone();
                let later = stream;
                stream = Stream::Delay(Box::new(move || {
                    interleave(qeval_stream(&engine, &sub, frame), later)
                }));
            }
            stream
        }
        Query::Not(sub) => {
            let probe = collect(qeval_stream(engine, sub, frame.clone()), 1);
            if probe.is_empty() {
                Stream::Cons(frame, Box::new(|| Stream::Nil))
            } else {
                Stream::Nil
            }
        }
        Query::Unique(sub) => {
            let mut probe = collect(qeval_stream(engine, sub, frame.clone()), 1);
            match probe.pop() {
                Some(first) => Stream::Cons(first, Box::new(|| Stream::Nil)),
                None => Stream::Nil,
            }
        }
        Query::UniqueBy(variables, sub) => {
            // The first answer for each projection survives, in the
            // sub-query's production order.
            let mut seen: Vec<Vec<Term>> = Vec::new();
            let mut kept = Vec::new();
            for answer in collect(qeval_stream(engine, sub, frame.clone()), usize::MAX) {
                let projection: Vec<Term> = variables
                    .iter()
                    .map(|name| walk(&Term::Variable(name.clone()), &answer).clone())
                    .collect();
                if seen.contains(&projection) {
                    continue;
                }
                seen.push(projection);
                kept.push(answer);
            }
            kept.into_iter().rev().fold(Stream::Nil, |rest, answer| {
                Stream::Cons(answer, Box::new(move || rest))
            })
        }
    }
}

/// One data-base entry a simple query may match: an assertion, or the
/// rule at an index of the rule store.
#[derive(Clone)]
enum Candidate {
    Assertion(Term),
    Rule(usize),
}

/// A simple query against one frame: every assertion and then every
/// rule of the relation, in insertion order, each contributing a
/// delayed stream; the streams combine as the right-nested interleave
/// `flatmap` builds, so the first candidate answers first and no
/// candidate's stream can starve the rest.
fn relation_stream(
    engine: &Rc<Engine>,
    name: &str,
    arguments: &[Term],
    frame: &Substitution,
) -> Stream {
    let assertions = engine
        .db
        .assertions
        .iter()
        .filter(|assertion| relation_named(assertion, name))
        .map(|assertion| Candidate::Assertion(assertion.clone()));
    let rules = engine
        .db
        .rules
        .iter()
        .enumerate()
        .filter(|(_, rule)| relation_named(&rule.conclusion, name))
        .map(|(index, _)| Candidate::Rule(index));
    let candidates: Vec<Candidate> = assertions.chain(rules).collect();
    let mut stream = Stream::Nil;
    for candidate in candidates.into_iter().rev() {
        let engine = Rc::clone(engine);
        let arguments = arguments.to_vec();
        let frame = frame.clone();
        let later = stream;
        stream = Stream::Delay(Box::new(move || {
            interleave(
                candidate_stream(&engine, &arguments, candidate, frame),
                later,
            )
        }));
    }
    stream
}

fn candidate_stream(
    engine: &Rc<Engine>,
    arguments: &[Term],
    candidate: Candidate,
    frame: Substitution,
) -> Stream {
    let (head, conditions) = match candidate {
        Candidate::Assertion(assertion) => (assertion, Vec::new()),
        // A rule application renames the rule apart with a fresh
        // application number (`apply-a-rule`), then continues with
        // its conditions.
        Candidate::Rule(index) => {
            let fresh = engine.next_var.get();
            engine.next_var.set(fresh + 1);
            rename_rule(&engine.db.rules[index], fresh)
        }
    };
    let head_args = relation_arguments(&head);
    if head_args.len() != arguments.len() {
        return Stream::Nil;
    }
    let mut current = frame;
    for (pattern, datum) in arguments.iter().zip(head_args) {
        match unify(pattern, &datum, &current) {
            Some(extended) => current = extended,
            None => return Stream::Nil,
        }
    }
    let mut stream = Stream::Cons(current, Box::new(|| Stream::Nil));
    for condition in conditions {
        let engine = Rc::clone(engine);
        let project: Rc<dyn Fn(Substitution) -> Stream> =
            Rc::new(move |frame| qeval_stream(&engine, &condition, frame));
        stream = flatmap(stream, &project);
    }
    stream
}

fn relation_named(term: &Term, name: &str) -> bool {
    match term {
        Term::Atom(head) => head == name,
        Term::Pair(head, _) => matches!(head.as_ref(), Term::Atom(head) if head == name),
        _ => false,
    }
}

fn relation_arguments(term: &Term) -> Vec<Term> {
    let mut args = Vec::new();
    let mut cursor = term.clone();
    loop {
        match cursor {
            Term::Pair(head, tail) => {
                args.push(*head);
                cursor = *tail;
            }
            Term::Empty => break,
            other => {
                args.push(other);
                break;
            }
        }
    }
    if matches!(args.first(), Some(Term::Atom(_))) {
        args.remove(0);
    }
    args
}

fn rename_rule(rule: &Rule, fresh: usize) -> (Term, Vec<Query>) {
    let mut seen: HashMap<String, String> = HashMap::new();
    let conclusion = rename_term(&rule.conclusion, &mut seen, fresh);
    let conditions = rule
        .conditions
        .iter()
        .map(|query| rename_query(query, &mut seen, fresh))
        .collect();
    (conclusion, conditions)
}

fn rename_term(term: &Term, seen: &mut HashMap<String, String>, fresh: usize) -> Term {
    match term {
        Term::Variable(name) => {
            let renamed = seen
                .entry(name.clone())
                .or_insert_with(|| format!("{name}#{fresh}"));
            Term::Variable(renamed.clone())
        }
        Term::Pair(left, right) => Term::Pair(
            Box::new(rename_term(left, seen, fresh)),
            Box::new(rename_term(right, seen, fresh)),
        ),
        other => other.clone(),
    }
}

/// Renames every variable a host predicate names: term positions
/// through [`rename_term`], binding-name positions through the same
/// table, so a renamed rule's `Value` condition reads the renamed
/// frame (SICP's `rename-variables-in` renames the whole rule).
fn rename_predicate(
    predicate: &Predicate,
    seen: &mut HashMap<String, String>,
    fresh: usize,
) -> Predicate {
    let renamed = |name: &String, seen: &mut HashMap<String, String>| {
        seen.entry(name.clone())
            .or_insert_with(|| format!("{name}#{fresh}"))
            .clone()
    };
    match predicate {
        Predicate::Eq(a, b) => {
            Predicate::Eq(rename_term(a, seen, fresh), rename_term(b, seen, fresh))
        }
        Predicate::Ne(a, b) => {
            Predicate::Ne(rename_term(a, seen, fresh), rename_term(b, seen, fresh))
        }
        Predicate::Lt(a, b) => {
            Predicate::Lt(rename_term(a, seen, fresh), rename_term(b, seen, fresh))
        }
        Predicate::Le(a, b) => {
            Predicate::Le(rename_term(a, seen, fresh), rename_term(b, seen, fresh))
        }
        Predicate::Gt(a, b) => {
            Predicate::Gt(rename_term(a, seen, fresh), rename_term(b, seen, fresh))
        }
        Predicate::Ge(a, b) => {
            Predicate::Ge(rename_term(a, seen, fresh), rename_term(b, seen, fresh))
        }
        Predicate::TextLt(a, b) => {
            Predicate::TextLt(rename_term(a, seen, fresh), rename_term(b, seen, fresh))
        }
        Predicate::SumEq(terms, bound) => Predicate::SumEq(
            terms
                .iter()
                .map(|term| rename_term(term, seen, fresh))
                .collect(),
            *bound,
        ),
        Predicate::Bound(term) => Predicate::Bound(rename_term(term, seen, fresh)),
        Predicate::Or(items) => Predicate::Or(
            items
                .iter()
                .map(|item| rename_predicate(item, seen, fresh))
                .collect(),
        ),
        Predicate::DiffEq(a, b, c, d) => Predicate::DiffEq(
            renamed(a, seen),
            renamed(b, seen),
            renamed(c, seen),
            renamed(d, seen),
        ),
        Predicate::Pythagorean(a, b, c) => {
            Predicate::Pythagorean(renamed(a, seen), renamed(b, seen), renamed(c, seen))
        }
    }
}

fn rename_query(query: &Query, seen: &mut HashMap<String, String>, fresh: usize) -> Query {
    match query {
        Query::Unify(left, right) => Query::Unify(
            rename_term(left, seen, fresh),
            rename_term(right, seen, fresh),
        ),
        Query::Relation { name, arguments } => Query::Relation {
            name: name.clone(),
            arguments: arguments
                .iter()
                .map(|term| rename_term(term, seen, fresh))
                .collect(),
        },
        Query::And(subs) => Query::And(
            subs.iter()
                .map(|sub| rename_query(sub, seen, fresh))
                .collect(),
        ),
        Query::Or(subs) => Query::Or(
            subs.iter()
                .map(|sub| rename_query(sub, seen, fresh))
                .collect(),
        ),
        Query::Not(sub) => Query::Not(Box::new(rename_query(sub, seen, fresh))),
        Query::Unique(sub) => Query::Unique(Box::new(rename_query(sub, seen, fresh))),
        Query::Value(predicate, arguments) => Query::Value(
            rename_predicate(predicate, seen, fresh),
            arguments
                .iter()
                .map(|term| rename_term(term, seen, fresh))
                .collect(),
        ),
        Query::UniqueBy(variables, sub) => {
            Query::UniqueBy(variables.clone(), Box::new(rename_query(sub, seen, fresh)))
        }
    }
}

/// Evaluates one host predicate against the current frame (the
/// `lisp-value` discipline: an unbound pattern variable fails the
/// frame rather than raising).
fn holds(predicate: &Predicate, arguments: &[Term], frame: &Substitution) -> bool {
    for argument in arguments {
        if !is_ground(argument, frame) {
            return false;
        }
    }
    let value = |term: &Term| -> Option<i64> {
        match ground(term, frame)? {
            Term::Integer(value) => Some(value),
            _ => None,
        }
    };
    let text = |term: &Term| -> Option<String> {
        Some(match ground(term, frame)? {
            Term::Text(text) | Term::Atom(text) => text,
            Term::Integer(value) => value.to_string(),
            other => format!("{other:?}"),
        })
    };
    let operands = |a: &Term, b: &Term| Some((value(a)?, value(b)?));
    let text_operands = |a: &Term, b: &Term| Some((text(a)?, text(b)?));
    // Equality compares any two fully resolved terms, including nested
    // pairs; an unbound variable anywhere in either term fails the frame.
    let resolved = |a: &Term, b: &Term| -> Option<(Term, Term)> {
        Some((ground(a, frame)?, ground(b, frame)?))
    };
    match predicate {
        Predicate::Eq(a, b) => resolved(a, b).is_some_and(|(a, b)| a == b),
        Predicate::Ne(a, b) => resolved(a, b).is_some_and(|(a, b)| a != b),
        Predicate::Lt(a, b) => operands(a, b).is_some_and(|(a, b)| a < b),
        Predicate::Le(a, b) => operands(a, b).is_some_and(|(a, b)| a <= b),
        Predicate::Gt(a, b) => operands(a, b).is_some_and(|(a, b)| a > b),
        Predicate::Ge(a, b) => operands(a, b).is_some_and(|(a, b)| a >= b),
        Predicate::TextLt(a, b) => text_operands(a, b).is_some_and(|(a, b)| a < b),
        Predicate::SumEq(terms, bound_total) => {
            let mut total = 0_i64;
            for term in terms {
                match value(term) {
                    Some(value) => total = total.checked_add(value).unwrap_or(i64::MAX),
                    None => return false,
                }
            }
            total == *bound_total
        }
        Predicate::Bound(term) => is_ground(term, frame),
        Predicate::Or(items) => items.iter().any(|item| holds(item, arguments, frame)),
        Predicate::DiffEq(a, b, c, d) => {
            let named = |name: &String| -> Option<i64> { value(&Term::Variable(name.clone())) };
            match (named(a), named(b), named(c), named(d)) {
                (Some(a), Some(b), Some(c), Some(d)) => a - b == c - d,
                _ => false,
            }
        }
        Predicate::Pythagorean(a, b, c) => {
            let named = |name: &String| -> Option<i64> { value(&Term::Variable(name.clone())) };
            match (named(a), named(b), named(c)) {
                (Some(a), Some(b), Some(c)) => {
                    match (a.checked_mul(a), b.checked_mul(b), c.checked_mul(c)) {
                        (Some(aa), Some(bb), Some(cc)) => aa + bb == cc,
                        _ => false,
                    }
                }
                _ => false,
            }
        }
    }
}

fn collect(mut stream: Stream, limit: usize) -> Vec<Substitution> {
    let mut answers = Vec::new();
    while answers.len() < limit {
        match force(stream) {
            Stream::Nil => break,
            Stream::Cons(answer, rest) => {
                answers.push(answer);
                stream = rest();
            }
            Stream::Delay(_) => unreachable!("force resolves delays"),
        }
    }
    answers
}

/// One bounded query run: the first answers in production order plus
/// whether the stream exhausted before the fuel ran out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryRunReport {
    /// The first answers, in production order.
    pub answers: Vec<Substitution>,
    /// Whether the answer stream exhausted within budget.
    pub exhausted: bool,
    /// The evaluation operations this run consumed: forced answer
    /// cells on the delayed engine, unification attempts and candidate
    /// expansions on the undelayed engine.
    pub steps: u64,
}

fn collect_bounded(mut stream: Stream, fuel: u64) -> QueryRunReport {
    let mut answers = Vec::new();
    let mut steps = 0_u64;
    let mut remaining = fuel;
    loop {
        match force(stream) {
            Stream::Nil => {
                return QueryRunReport {
                    answers,
                    exhausted: true,
                    steps,
                };
            }
            Stream::Cons(answer, rest) => {
                if remaining == 0 {
                    return QueryRunReport {
                        answers,
                        exhausted: false,
                        steps,
                    };
                }
                remaining -= 1;
                steps += 1;
                answers.push(answer);
                stream = rest();
            }
            Stream::Delay(_) => unreachable!("force resolves delays"),
        }
    }
}

/// Runs one query under a step budget on the delayed engine: each
/// forced answer cell consumes one unit of fuel.
#[must_use]
pub fn qeval_bounded(database: &Database, query: &Query, fuel: u64) -> QueryRunReport {
    let engine = Rc::new(Engine {
        db: database.clone(),
        next_var: Cell::new(0),
    });
    collect_bounded(qeval_stream(&engine, query, Substitution::new()), fuel)
}

struct EagerEngine {
    db: Database,
    next_var: Cell<usize>,
}

struct EagerRun {
    fuel: u64,
    steps: u64,
    truncated: bool,
}

fn eager_spend(run: &mut EagerRun) -> bool {
    if run.fuel == 0 {
        run.truncated = true;
        return false;
    }
    run.fuel -= 1;
    run.steps += 1;
    true
}

fn eager_conditions(
    engine: &EagerEngine,
    probe: &Rc<Engine>,
    conditions: &[Query],
    frame: &Substitution,
    run: &mut EagerRun,
    out: &mut Vec<Substitution>,
) {
    let Some((first, rest)) = conditions.split_first() else {
        out.push(frame.clone());
        return;
    };
    let mut here = Vec::new();
    eager_query(engine, probe, first, frame, run, &mut here);
    for next in here {
        eager_conditions(engine, probe, rest, &next, run, out);
    }
}

// This exhaustive Query match keeps the undelayed evaluation rules adjacent.
#[allow(clippy::too_many_lines)]
fn eager_query(
    engine: &EagerEngine,
    probe: &Rc<Engine>,
    query: &Query,
    frame: &Substitution,
    run: &mut EagerRun,
    out: &mut Vec<Substitution>,
) {
    if run.truncated {
        return;
    }
    match query {
        Query::Unify(left, right) => {
            if !eager_spend(run) {
                return;
            }
            if let Some(next) = unify(left, right, frame) {
                out.push(next);
            }
        }
        Query::Value(predicate, arguments) => {
            if !eager_spend(run) {
                return;
            }
            if holds(predicate, arguments, frame) {
                out.push(frame.clone());
            }
        }
        Query::Relation { name, arguments } => {
            for assertion in &engine.db.assertions {
                if !relation_named(assertion, name) {
                    continue;
                }
                if !eager_spend(run) {
                    return;
                }
                let candidate_args = relation_arguments(assertion);
                if candidate_args.len() != arguments.len() {
                    continue;
                }
                let mut current = frame.clone();
                let mut matched = true;
                for (pattern, datum) in arguments.iter().zip(candidate_args.iter()) {
                    if let Some(extended) = unify(pattern, datum, &current) {
                        current = extended;
                    } else {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    out.push(current);
                }
            }
            for rule in &engine.db.rules {
                if !relation_named(&rule.conclusion, name) {
                    continue;
                }
                if !eager_spend(run) {
                    return;
                }
                let fresh = engine.next_var.get();
                engine.next_var.set(fresh + 1);
                let (conclusion, conditions) = rename_rule(rule, fresh);
                let mut current = frame.clone();
                let mut matched = true;
                for (pattern, datum) in arguments.iter().zip(relation_arguments(&conclusion).iter())
                {
                    if let Some(extended) = unify(pattern, datum, &current) {
                        current = extended;
                    } else {
                        matched = false;
                        break;
                    }
                }
                if !matched {
                    continue;
                }
                eager_conditions(engine, probe, &conditions, &current, run, out);
            }
        }
        Query::And(subs) => {
            eager_conditions(engine, probe, subs, frame, run, out);
        }
        Query::Or(subs) => {
            for sub in subs {
                eager_query(engine, probe, sub, frame, run, out);
            }
        }
        Query::Not(sub) => {
            let mut probe_out = Vec::new();
            eager_query(engine, probe, sub, frame, run, &mut probe_out);
            if probe_out.is_empty() && !run.truncated {
                out.push(frame.clone());
            }
        }
        Query::Unique(sub) => {
            let mut probe_out = Vec::new();
            eager_query(engine, probe, sub, frame, run, &mut probe_out);
            if let Some(first) = probe_out.into_iter().next() {
                out.push(first);
            }
        }
        Query::UniqueBy(variables, sub) => {
            let mut probe_out = Vec::new();
            eager_query(engine, probe, sub, frame, run, &mut probe_out);
            let mut seen: Vec<Vec<Term>> = Vec::new();
            for answer in probe_out {
                let projection: Vec<Term> = variables
                    .iter()
                    .map(|name| walk(&Term::Variable(name.clone()), &answer).clone())
                    .collect();
                if seen.contains(&projection) {
                    continue;
                }
                seen.push(projection);
                out.push(answer);
            }
        }
    }
}

/// Runs one query under a step budget on the eager, undelayed engine:
/// the same database and unification with depth-first concatenation
/// instead of fair delayed streams, so 4.71a can compare which
/// strategy yields the first answer within equal fuel.
#[must_use]
pub fn qeval_undelayed_bounded(database: &Database, query: &Query, fuel: u64) -> QueryRunReport {
    let engine = EagerEngine {
        db: database.clone(),
        next_var: Cell::new(0),
    };
    let probe = Rc::new(Engine {
        db: database.clone(),
        next_var: Cell::new(0),
    });
    let mut run = EagerRun {
        fuel,
        steps: 0,
        truncated: false,
    };
    let mut answers = Vec::new();
    eager_query(
        &engine,
        &probe,
        query,
        &Substitution::new(),
        &mut run,
        &mut answers,
    );
    QueryRunReport {
        answers,
        exhausted: !run.truncated,
        steps: run.steps,
    }
}

/// Runs one query over one database, answering every substitution in
/// fair, delayed production order. A ground query answers its empty
/// substitution exactly once.
#[must_use]
pub fn qeval(database: &Database, query: &Query) -> QueryOutcome {
    qeval_prefix(database, query, usize::MAX)
}

/// Runs one query for at most `n` answers: the observable prefix
/// discipline for recursive and infinite rule sets.
#[must_use]
pub fn qeval_prefix(database: &Database, query: &Query, n: usize) -> QueryOutcome {
    let engine = Rc::new(Engine {
        db: database.clone(),
        next_var: Cell::new(0),
    });
    let stream = qeval_stream(&engine, query, Substitution::new());
    QueryOutcome {
        answers: collect(stream, n),
    }
}

struct Engine {
    db: Database,
    next_var: Cell<usize>,
}

/// The unifier: explicit substitution extension with its stated
/// occurs-check policy (enabled), so `?x = (?x . ?y)` never binds.
#[must_use]
pub fn unify(left: &Term, right: &Term, frame: &Substitution) -> Option<Substitution> {
    let left_walked = walk(left, frame).clone();
    let right_walked = walk(right, frame).clone();
    let left = &left_walked;
    let right = &right_walked;
    match (left, right) {
        (Term::Variable(a), Term::Variable(b)) if a == b => Some(frame.clone()),
        (Term::Variable(name), other) | (other, Term::Variable(name)) => {
            if occurs(name, other, frame) {
                return None;
            }
            Some(extend(frame, name, other.clone()))
        }
        (Term::Pair(a_left, a_right), Term::Pair(b_left, b_right)) => {
            let first = unify(a_left, b_left, frame)?;
            unify(a_right, b_right, &first)
        }
        (Term::Empty, Term::Empty) => Some(frame.clone()),
        (Term::Integer(a), Term::Integer(b)) if a == b => Some(frame.clone()),
        (Term::Text(a), Term::Text(b)) if a == b => Some(frame.clone()),
        (Term::Atom(a), Term::Atom(b)) if a == b => Some(frame.clone()),
        _ => None,
    }
}

fn walk(term: &Term, frame: &Substitution) -> Term {
    match term {
        Term::Variable(name) => match frame.get(name) {
            Some(bound) => walk(bound, frame),
            None => term.clone(),
        },
        other => other.clone(),
    }
}
fn is_ground(term: &Term, frame: &Substitution) -> bool {
    match term {
        Term::Variable(name) => frame.get(name).is_some_and(|bound| is_ground(bound, frame)),
        Term::Pair(left, right) => is_ground(left, frame) && is_ground(right, frame),
        _ => true,
    }
}

fn ground(term: &Term, frame: &Substitution) -> Option<Term> {
    match term {
        Term::Variable(name) => ground(frame.get(name)?, frame),
        Term::Pair(left, right) => Some(Term::Pair(
            Box::new(ground(left, frame)?),
            Box::new(ground(right, frame)?),
        )),
        other => Some(other.clone()),
    }
}

fn occurs(name: &str, term: &Term, frame: &Substitution) -> bool {
    match walk(term, frame) {
        Term::Variable(other) => other == name,
        Term::Pair(left, right) => occurs(name, &left, frame) || occurs(name, &right, frame),
        _ => false,
    }
}

fn list_term(items: &[Term]) -> Term {
    let mut list = Term::Empty;
    for item in items.iter().rev() {
        list = Term::Pair(Box::new(item.clone()), Box::new(list));
    }
    list
}

fn atom(name: &str) -> Term {
    Term::Atom(name.to_owned())
}

fn var(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

/// Case `query/01-basic-personnel`: ground facts and one open query;
/// ordered answers.
#[must_use]
pub fn query_personnel() -> (Database, Query) {
    let mut database = Database::default();
    database.assert(list_term(&[
        atom("job"),
        list_term(&[atom("Bitdiddle"), atom("Ben")]),
        list_term(&[atom("computer"), atom("wizard")]),
    ]));
    database.assert(list_term(&[
        atom("job"),
        list_term(&[atom("Hacker"), atom("Alyssa")]),
        list_term(&[atom("computer"), atom("programmer")]),
    ]));
    database.assert(list_term(&[
        atom("job"),
        list_term(&[atom("Tweakit"), atom("Louis")]),
        list_term(&[atom("computer"), atom("technician")]),
    ]));
    let query = Query::Relation {
        name: "job".to_owned(),
        arguments: vec![var("person"), var("title")],
    };
    (database, query)
}

/// Case `query/02-compound-queries`: `and`, `or`, and `not`
/// composition over the personnel facts.
#[must_use]
pub fn query_compound() -> (Database, Query) {
    let (database, _) = query_personnel();
    let query = Query::And(vec![
        Query::Relation {
            name: "job".to_owned(),
            arguments: vec![var("person"), list_term(&[atom("computer"), var("role")])],
        },
        Query::Not(Box::new(Query::Unify(var("role"), atom("technician")))),
    ]);
    (database, query)
}

/// Case `query/03-rules`: rule application with renamed rule
/// variables.
#[must_use]
pub fn query_rules() -> (Database, Query) {
    let (mut database, _) = query_personnel();
    database.assert(list_term(&[
        atom("supervisor"),
        list_term(&[atom("Hacker"), atom("Alyssa")]),
        list_term(&[atom("Bitdiddle"), atom("Ben")]),
    ]));
    database.assert(list_term(&[
        atom("supervisor"),
        list_term(&[atom("Tweakit"), atom("Louis")]),
        list_term(&[atom("Bitdiddle"), atom("Ben")]),
    ]));
    database.add_rule(Rule {
        conclusion: list_term(&[atom("boss"), var("x"), var("y")]),
        conditions: vec![Query::Relation {
            name: "supervisor".to_owned(),
            arguments: vec![var("x"), var("y")],
        }],
    });
    let query = Query::Relation {
        name: "boss".to_owned(),
        arguments: vec![var("worker"), var("chief")],
    };
    (database, query)
}

/// Case `query/04-append-form`: one recursive rule over `Pair` terms;
/// the five splits of a four-element list.
#[must_use]
pub fn query_append() -> (Database, Query) {
    let mut database = Database::default();
    database.add_rule(Rule {
        conclusion: list_term(&[atom("append-to-form"), Term::Empty, var("y"), var("y")]),
        conditions: Vec::new(),
    });
    database.add_rule(Rule {
        conclusion: list_term(&[
            atom("append-to-form"),
            Term::Pair(Box::new(var("u")), Box::new(var("v"))),
            var("y"),
            Term::Pair(Box::new(var("u")), Box::new(var("z"))),
        ]),
        conditions: vec![Query::Relation {
            name: "append-to-form".to_owned(),
            arguments: vec![var("v"), var("y"), var("z")],
        }],
    });
    let query = Query::Relation {
        name: "append-to-form".to_owned(),
        arguments: vec![
            var("x"),
            var("y"),
            list_term(&[atom("a"), atom("b"), atom("c"), atom("d")]),
        ],
    };
    (database, query)
}

/// The independent finite reference for grammar §7 query semantics:
/// a fresh naive evaluator with its own walk/unify and an independent
/// interleave/flatmap answer production, so answer sets and their fair
/// production order can be checked against the teaching engine without
/// calling it.
#[must_use]
pub fn reference_answers(database: &Database, query: &Query) -> Vec<Substitution> {
    let mut fresh = 0_usize;
    reference_stream(database, query, &Substitution::new(), 0, &mut fresh)
}

fn reference_interleave(
    mut left: Vec<Substitution>,
    mut right: Vec<Substitution>,
) -> Vec<Substitution> {
    let mut out = Vec::with_capacity(left.len() + right.len());
    while !left.is_empty() || !right.is_empty() {
        if let Some(first) = left.first().cloned() {
            out.push(first);
            left.remove(0);
        }
        if let Some(next) = right.first().cloned() {
            out.push(next);
            right.remove(0);
        }
    }
    out
}

/// Combines streams the way `flatmap` does: the first stream
/// interleaved with the combination of the rest (a right fold).
fn reference_flatten(streams: Vec<Vec<Substitution>>) -> Vec<Substitution> {
    streams.into_iter().rev().fold(Vec::new(), |later, stream| {
        reference_interleave(stream, later)
    })
}

/// One query over a finite frame stream: each frame's answers, in
/// frame order, flattened.
fn reference_over_frames(
    database: &Database,
    query: &Query,
    frames: &[Substitution],
    depth: usize,
    fresh: &mut usize,
) -> Vec<Substitution> {
    let streams = frames
        .iter()
        .map(|frame| reference_stream(database, query, frame, depth, fresh))
        .collect();
    reference_flatten(streams)
}

/// A conjunction: each condition runs over the frames the previous
/// conditions produced.
fn reference_conjoin(
    database: &Database,
    conditions: &[Query],
    frame: Substitution,
    depth: usize,
    fresh: &mut usize,
) -> Vec<Substitution> {
    conditions.iter().fold(vec![frame], |frames, condition| {
        reference_over_frames(database, condition, &frames, depth, fresh)
    })
}

fn reference_stream(
    database: &Database,
    query: &Query,
    frame: &Substitution,
    depth: usize,
    fresh: &mut usize,
) -> Vec<Substitution> {
    if depth > 32 {
        return Vec::new();
    }
    match query {
        Query::Unify(left, right) => reference_unify(left, right, frame).into_iter().collect(),
        Query::Relation { name, arguments } => {
            let mut head_items = vec![Term::Atom(name.clone())];
            head_items.extend(arguments.iter().cloned());
            let pattern = list_term(&head_items);
            // Every assertion, then every rule, in insertion order.
            let mut streams: Vec<Vec<Substitution>> = database
                .assertions
                .iter()
                .map(|assertion| {
                    reference_unify(&pattern, assertion, frame)
                        .into_iter()
                        .collect()
                })
                .collect();
            for rule in &database.rules {
                let (conclusion, conditions) = reference_rename(rule, fresh);
                streams.push(match reference_unify(&pattern, &conclusion, frame) {
                    Some(next) => reference_conjoin(database, &conditions, next, depth + 1, fresh),
                    None => Vec::new(),
                });
            }
            reference_flatten(streams)
        }
        Query::And(conditions) => {
            reference_conjoin(database, conditions, frame.clone(), depth, fresh)
        }
        Query::Or(branches) => reference_flatten(
            branches
                .iter()
                .map(|branch| reference_stream(database, branch, frame, depth, fresh))
                .collect(),
        ),
        Query::Not(sub) => {
            if reference_stream(database, sub, frame, depth, fresh).is_empty() {
                vec![frame.clone()]
            } else {
                Vec::new()
            }
        }
        Query::Unique(sub) => reference_stream(database, sub, frame, depth, fresh)
            .into_iter()
            .next()
            .into_iter()
            .collect(),
        Query::Value(predicate, terms) => {
            if reference_holds(predicate, terms, frame) {
                vec![frame.clone()]
            } else {
                Vec::new()
            }
        }
        Query::UniqueBy(names, sub) => {
            let mut seen: Vec<Vec<Term>> = Vec::new();
            let mut out = Vec::new();
            for answer in reference_stream(database, sub, frame, depth, fresh) {
                let key: Vec<Term> = names
                    .iter()
                    .map(|name| reference_walk(&Term::Variable(name.clone()), &answer))
                    .collect();
                if !seen.contains(&key) {
                    seen.push(key);
                    out.push(answer);
                }
            }
            out
        }
    }
}

fn reference_rename(rule: &Rule, fresh: &mut usize) -> (Term, Vec<Query>) {
    *fresh += 1;
    let suffix = *fresh;
    let mut names = std::collections::HashMap::new();
    (
        reference_rename_term(&rule.conclusion, suffix, &mut names),
        rule.conditions
            .iter()
            .map(|condition| reference_rename_query(condition, suffix, &mut names))
            .collect(),
    )
}

fn reference_rename_term(
    term: &Term,
    suffix: usize,
    names: &mut std::collections::HashMap<String, String>,
) -> Term {
    match term {
        Term::Variable(name) => {
            let renamed = names
                .entry(name.clone())
                .or_insert_with(|| format!("{name}#{suffix}"))
                .clone();
            Term::Variable(renamed)
        }
        Term::Pair(left, right) => Term::Pair(
            Box::new(reference_rename_term(left, suffix, names)),
            Box::new(reference_rename_term(right, suffix, names)),
        ),
        other => other.clone(),
    }
}

fn reference_rename_query(
    query: &Query,
    suffix: usize,
    names: &mut std::collections::HashMap<String, String>,
) -> Query {
    match query {
        Query::Unify(left, right) => Query::Unify(
            reference_rename_term(left, suffix, names),
            reference_rename_term(right, suffix, names),
        ),
        Query::Relation { name, arguments } => Query::Relation {
            name: name.clone(),
            arguments: arguments
                .iter()
                .map(|argument| reference_rename_term(argument, suffix, names))
                .collect(),
        },
        Query::And(items) => Query::And(
            items
                .iter()
                .map(|item| reference_rename_query(item, suffix, names))
                .collect(),
        ),
        Query::Or(items) => Query::Or(
            items
                .iter()
                .map(|item| reference_rename_query(item, suffix, names))
                .collect(),
        ),
        Query::Not(sub) => Query::Not(Box::new(reference_rename_query(sub, suffix, names))),
        Query::Unique(sub) => Query::Unique(Box::new(reference_rename_query(sub, suffix, names))),
        Query::UniqueBy(vars, sub) => Query::UniqueBy(
            vars.clone(),
            Box::new(reference_rename_query(sub, suffix, names)),
        ),
        Query::Value(predicate, arguments) => Query::Value(
            reference_rename_predicate(predicate, suffix, names),
            arguments
                .iter()
                .map(|argument| reference_rename_term(argument, suffix, names))
                .collect(),
        ),
    }
}

fn reference_rename_predicate(
    predicate: &Predicate,
    suffix: usize,
    names: &mut std::collections::HashMap<String, String>,
) -> Predicate {
    let renamed = |name: &String, names: &mut std::collections::HashMap<String, String>| {
        names
            .entry(name.clone())
            .or_insert_with(|| format!("{name}#{suffix}"))
            .clone()
    };
    match predicate {
        Predicate::Eq(a, b) => Predicate::Eq(
            reference_rename_term(a, suffix, names),
            reference_rename_term(b, suffix, names),
        ),
        Predicate::Ne(a, b) => Predicate::Ne(
            reference_rename_term(a, suffix, names),
            reference_rename_term(b, suffix, names),
        ),
        Predicate::Lt(a, b) => Predicate::Lt(
            reference_rename_term(a, suffix, names),
            reference_rename_term(b, suffix, names),
        ),
        Predicate::Le(a, b) => Predicate::Le(
            reference_rename_term(a, suffix, names),
            reference_rename_term(b, suffix, names),
        ),
        Predicate::Gt(a, b) => Predicate::Gt(
            reference_rename_term(a, suffix, names),
            reference_rename_term(b, suffix, names),
        ),
        Predicate::Ge(a, b) => Predicate::Ge(
            reference_rename_term(a, suffix, names),
            reference_rename_term(b, suffix, names),
        ),
        Predicate::TextLt(a, b) => Predicate::TextLt(
            reference_rename_term(a, suffix, names),
            reference_rename_term(b, suffix, names),
        ),
        Predicate::SumEq(terms, bound) => Predicate::SumEq(
            terms
                .iter()
                .map(|term| reference_rename_term(term, suffix, names))
                .collect(),
            *bound,
        ),
        Predicate::Bound(term) => Predicate::Bound(reference_rename_term(term, suffix, names)),
        Predicate::Or(items) => Predicate::Or(
            items
                .iter()
                .map(|item| reference_rename_predicate(item, suffix, names))
                .collect(),
        ),
        Predicate::DiffEq(a, b, c, d) => Predicate::DiffEq(
            renamed(a, names),
            renamed(b, names),
            renamed(c, names),
            renamed(d, names),
        ),
        Predicate::Pythagorean(a, b, c) => {
            Predicate::Pythagorean(renamed(a, names), renamed(b, names), renamed(c, names))
        }
    }
}

fn reference_walk(term: &Term, frame: &Substitution) -> Term {
    match term {
        Term::Variable(name) => match frame.get(name) {
            Some(bound) => reference_walk(bound, frame),
            None => term.clone(),
        },
        _ => term.clone(),
    }
}
fn reference_is_ground(term: &Term, frame: &Substitution) -> bool {
    match term {
        Term::Variable(name) => frame
            .get(name)
            .is_some_and(|bound| reference_is_ground(bound, frame)),
        Term::Pair(left, right) => {
            reference_is_ground(left, frame) && reference_is_ground(right, frame)
        }
        _ => true,
    }
}

fn reference_ground(term: &Term, frame: &Substitution) -> Option<Term> {
    match term {
        Term::Variable(name) => reference_ground(frame.get(name)?, frame),
        Term::Pair(left, right) => Some(Term::Pair(
            Box::new(reference_ground(left, frame)?),
            Box::new(reference_ground(right, frame)?),
        )),
        other => Some(other.clone()),
    }
}

fn reference_occurs(name: &str, term: &Term, frame: &Substitution) -> bool {
    match reference_walk(term, frame) {
        Term::Variable(other) => other == name,
        Term::Pair(left, right) => {
            reference_occurs(name, &left, frame) || reference_occurs(name, &right, frame)
        }
        _ => false,
    }
}

fn reference_unify(left: &Term, right: &Term, frame: &Substitution) -> Option<Substitution> {
    let left = reference_walk(left, frame);
    let right = reference_walk(right, frame);
    match (&left, &right) {
        (Term::Variable(a), Term::Variable(b)) if a == b => Some(frame.clone()),
        (Term::Variable(name), other) | (other, Term::Variable(name)) => {
            if reference_occurs(name, other, frame) {
                None
            } else {
                let mut next = frame.clone();
                next.insert(name.clone(), other.clone());
                Some(next)
            }
        }
        (Term::Integer(a), Term::Integer(b)) if a == b => Some(frame.clone()),
        (Term::Text(a), Term::Text(b)) if a == b => Some(frame.clone()),
        (Term::Atom(a), Term::Atom(b)) if a == b => Some(frame.clone()),
        (Term::Empty, Term::Empty) => Some(frame.clone()),
        (Term::Pair(a1, a2), Term::Pair(b1, b2)) => {
            let next = reference_unify(a1, b1, frame)?;
            reference_unify(a2, b2, &next)
        }
        _ => None,
    }
}

fn reference_holds(predicate: &Predicate, terms: &[Term], frame: &Substitution) -> bool {
    use Predicate::{Bound, DiffEq, Eq, Ge, Gt, Le, Lt, Ne, Or, Pythagorean, SumEq, TextLt};
    if terms.iter().any(|term| !reference_is_ground(term, frame)) {
        return false;
    }
    let value = |name: &String| -> Option<i64> {
        match reference_walk(&Term::Variable(name.clone()), frame) {
            Term::Integer(value) => Some(value),
            _ => None,
        }
    };
    match predicate {
        Eq(a, b) => match (reference_ground(a, frame), reference_ground(b, frame)) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        },
        Ne(a, b) => match (reference_ground(a, frame), reference_ground(b, frame)) {
            (Some(a), Some(b)) => a != b,
            _ => false,
        },
        Lt(a, b) | Le(a, b) | Gt(a, b) | Ge(a, b) | TextLt(a, b) => {
            let (Some(a), Some(b)) = (reference_ground(a, frame), reference_ground(b, frame))
            else {
                return false;
            };
            match (&a, &b) {
                (Term::Integer(a), Term::Integer(b)) => match predicate {
                    Lt(..) => a < b,
                    Le(..) => a <= b,
                    Gt(..) => a > b,
                    Ge(..) => a >= b,
                    _ => false,
                },
                (Term::Text(a), Term::Text(b)) => match predicate {
                    TextLt(..) | Lt(..) => a < b,
                    Le(..) => a <= b,
                    Gt(..) => a > b,
                    Ge(..) => a >= b,
                    _ => false,
                },
                _ => false,
            }
        }
        SumEq(items, bound) => {
            let mut sum = 0_i64;
            for item in items {
                match reference_walk(item, frame) {
                    Term::Integer(value) => sum += value,
                    _ => return false,
                }
            }
            sum == *bound
        }
        Bound(term) => reference_is_ground(term, frame),
        Or(items) => items.iter().any(|item| reference_holds(item, terms, frame)),
        DiffEq(a, b, c, d) => match (value(a), value(b), value(c), value(d)) {
            (Some(a), Some(b), Some(c), Some(d)) => a - b == c - d,
            _ => false,
        },
        Pythagorean(a, b, c) => match (value(a), value(b), value(c)) {
            (Some(a), Some(b), Some(c)) => a * a + b * b == c * c,
            _ => false,
        },
    }
}
#[cfg(test)]
mod tests {
    use super::{Database, Predicate, Query, Term, qeval, reference_answers};

    #[test]
    fn host_equality_resolves_nested_pairs_and_rejects_unbound_variables() {
        let variable = |name: &str| Term::Variable(name.to_owned());
        let pair = |head| Term::Pair(Box::new(head), Box::new(Term::Empty));
        let database = Database::new();

        let unbound = Query::And(vec![
            Query::Unify(variable("whole"), pair(variable("nested"))),
            Query::Value(
                Predicate::Eq(pair(variable("nested")), pair(variable("nested"))),
                vec![variable("whole")],
            ),
        ]);
        assert!(qeval(&database, &unbound).answers.is_empty());
        assert!(reference_answers(&database, &unbound).is_empty());

        let atom = Term::Atom("item".to_owned());
        let bound = Query::And(vec![
            Query::Unify(variable("whole"), pair(variable("nested"))),
            Query::Unify(variable("nested"), atom.clone()),
            Query::Value(
                Predicate::Eq(pair(variable("nested")), pair(atom)),
                vec![variable("whole")],
            ),
        ]);
        assert_eq!(qeval(&database, &bound).answers.len(), 1);
        assert_eq!(reference_answers(&database, &bound).len(), 1);
    }
}
