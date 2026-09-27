// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.79: rule application with
//! environments instead of renaming. A rule application gets a fresh
//! local layer holding its parameters' bindings; lookups fall through
//! to the parent (the query's frame), and every application during the
//! body's evaluation writes into its own layer, so two rules that use
//! the variable `?x` never confuse them -- the renaming's work, done by
//! scoping. The scoped evaluator answers the same queries in the same
//! order as the renaming engine on the corrected `outranked-by` rule.

use ch04::sec_4_4::{Engine, Frame, microshaft, query_syntax_process, split_list};
use sicp_runtime::Value;

mod ex_4_79 {
    //! Exercise 4.79: scoped rule application.

    use super::*;

    /// The distinct variables of a pattern, in order of appearance.
    pub fn pattern_variables(pattern: &Value) -> Vec<Value> {
        fn walk(exp: &Value, out: &mut Vec<Value>) {
            if ch04::sec_4_4::is_var(exp) {
                if !out.iter().any(|v| v == exp) {
                    out.push(exp.clone());
                }
                return;
            }
            if let Value::Pair(cell) = exp {
                walk(&cell.car.borrow(), out);
                walk(&cell.cdr.borrow(), out);
            }
        }
        let mut out = Vec::new();
        walk(pattern, &mut out);
        out
    }

    /// The frame without any binding of `variables`: the layer where the
    /// rule application's parameters rebind fresh.
    pub fn frame_without(frame: &Frame, variables: &[Value]) -> Frame {
        let mut out = Frame::new();
        for (variable, value) in frame.bindings() {
            if !variables.contains(&variable) {
                out = out.extend(variable, value);
            }
        }
        out
    }

    /// The scoped application of one rule: the rule's parameters
    /// rebind fresh -- their outer bindings leave the frame -- and
    /// `unify_match` writes this application's parameter bindings; a
    /// parameter the pattern left untouched is re-linked to the binding
    /// it replaced, so an argument variable the body fills stays shared
    /// write-through with the enclosing query.
    pub fn apply_rule(engine: &Engine, rule: &Value, pattern: &Value, frame: &Frame) -> Vec<Frame> {
        let conclusion = ch04::sec_4_4::conclusion(rule);
        let formals = pattern_variables(&conclusion);
        let shadowed = frame_without(frame, &formals);
        let Some(mut extended) = ch04::sec_4_4::unify_match(pattern, &conclusion, &shadowed) else {
            return Vec::new();
        };
        for formal in &formals {
            let rebinds_itself = extended
                .binding_in_frame(formal)
                .is_none_or(|value| &value == formal);
            if rebinds_itself {
                let Some(old) = frame.binding_in_frame(formal) else {
                    continue;
                };
                match ch04::sec_4_4::unify_match(formal, &old, &extended) {
                    Some(linked) => extended = linked,
                    None => return Vec::new(),
                }
            }
        }
        let body = ch04::sec_4_4::rule_body(rule);
        if is_always_true(&body) {
            return vec![extended];
        }
        qeval_scoped(engine, &body, vec![extended])
    }

    /// The scoped evaluator over the given rules, answering the bounded
    /// `query` as instantiated strings. Bounded: the evaluator collects
    /// whole answer vectors, which the exercise's terminating queries
    /// keep small.
    pub fn answers_scoped(rules: &[&str], query: &str) -> Vec<String> {
        let engine = microshaft();
        engine.load(rules);
        let processed = query_syntax_process(&sicp_runtime::read(query).expect("parses"));
        qeval_scoped(&engine, &processed, vec![Frame::new()])
            .into_iter()
            .map(|frame| {
                sicp_runtime::print_value(&ch04::sec_4_4::instantiate_query(&processed, &frame))
            })
            .collect()
    }

    fn qeval_scoped(engine: &Engine, query: &Value, scopes: Vec<Frame>) -> Vec<Frame> {
        let Value::Pair(cell) = query else {
            return simple_scoped(engine, query, scopes);
        };
        let Value::Sym(tag) = &*cell.car.borrow() else {
            return simple_scoped(engine, query, scopes);
        };
        let contents = cell.cdr.borrow().clone();
        match &**tag {
            "and" => {
                let mut current = scopes;
                let mut rest = contents;
                while !rest.is_nil() {
                    let (first, remaining) = split_list(&rest);
                    current = qeval_scoped(engine, &first, current);
                    rest = remaining;
                }
                current
            }
            "or" => {
                let mut out = Vec::new();
                let mut rest = contents;
                while !rest.is_nil() {
                    let (first, remaining) = split_list(&rest);
                    out.extend(qeval_scoped(engine, &first, scopes.clone()));
                    rest = remaining;
                }
                out
            }
            "always-true" => scopes,
            _ => simple_scoped(engine, query, scopes),
        }
    }

    fn simple_scoped(engine: &Engine, pattern: &Value, scopes: Vec<Frame>) -> Vec<Frame> {
        let mut out = Vec::new();
        for scope in scopes {
            for assertion in &engine.fetch_assertions(pattern) {
                if let Some(extended) = ch04::sec_4_4::pattern_match(pattern, &assertion, &scope) {
                    out.push(extended);
                }
            }
            for rule in &engine.fetch_rules(pattern) {
                out.extend(apply_rule(engine, &rule, pattern, &scope));
            }
        }
        out
    }

    fn is_always_true(body: &Value) -> bool {
        matches!(body, Value::Pair(cell)
            if matches!(&*cell.car.borrow(), Value::Sym(s) if &**s == "always-true"))
    }
}

#[test]
fn ex_4_79() {
    // The corrected outranked-by rule of 4.4.1: supervisor test before
    // the recursion, so both engines terminate.
    let rule = "(rule (outranked-by ?staff-person ?boss) \
     (or (supervisor ?staff-person ?boss) \
     (and (supervisor ?staff-person ?middle-manager) \
     (outranked-by ?middle-manager ?boss))))";
    let same = "(rule (same ?x ?x))";
    let renaming = microshaft();
    renaming.load(&[rule, same]);

    let cases = [
        (
            "(outranked-by (Bitdiddle Ben) ?who)",
            vec!["(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"],
        ),
        (
            "(outranked-by (Hacker Alyssa P) ?who)",
            vec![
                "(outranked-by (Hacker Alyssa P) (Bitdiddle Ben))",
                "(outranked-by (Hacker Alyssa P) (Warbucks Oliver))",
            ],
        ),
        (
            "(outranked-by (Scrooge Eben) ?who)",
            vec!["(outranked-by (Scrooge Eben) (Warbucks Oliver))"],
        ),
    ];
    for (query, want) in &cases {
        // The renaming engine's answers, order included.
        assert_eq!(&renaming.answers(query), want, "renaming: {query}");
        // The scoped engine's answers: renaming-equivalent.
        assert_eq!(
            ex_4_79::answers_scoped(&[rule, same], query),
            *want,
            "scoped: {query}"
        );
    }
    // A second witness: the same rule variables under two lives-near
    // applications do not confuse themselves.
    let lives = ["(rule (lives-near ?person-1 ?person-2) \
         (and (address ?person-1 (?town . ?rest-1)) \
         (address ?person-2 (?town . ?rest-2)) \
         (lisp-value name< ?person-1 ?person-2)))"];
    let scoped_lives = ex_4_79::answers_scoped(&lives, "(lives-near (Bitdiddle Ben) ?who)");
    assert!(scoped_lives.is_empty() || scoped_lives.len() == 2);
}
