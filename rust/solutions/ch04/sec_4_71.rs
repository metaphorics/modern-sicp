// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.71: Louis Reasoner's undelayed
//! `simple-query` and `disjoin`, and the queries where they misbehave.
//! With the delay removed from simple query, a query whose rule
//! recursion diverges during stream construction never produces even a
//! first answer; the delayed engine streams answers out of the same
//! divergence. With the delay removed from `disjoin`, the rest
//! disjunction is a value that must exist before the first element, so
//! the whole `or` inherits the divergence. A fuel-bounded fallback pins
//! both without executing an unbounded run.

use std::cell::Cell;
use std::rc::Rc;

use ch04::sec_4_4::{Engine, Frame, QProc, microshaft, qeval_engine, split_list};
use ch04::sec_4_4::{interleave, stream_append};
use sicp_runtime::{Stream, Value, cons_cell};

mod ex_4_71 {
    //! Exercise 4.71: Louis's undelayed pair and the divergence probes.

    use super::*;

    /// Louis's undelayed `simple-query`: both operand streams are
    /// values before the first element is produced.
    pub fn undelayed_simple_query() -> QProc {
        Rc::new(|engine, pattern, frames| {
            let pattern = pattern.clone();
            let engine2 = Rc::clone(engine);
            engine.flatmap(
                Rc::new(move |frame: &Frame| {
                    let frame = frame.clone();
                    let assertions = engine2.find_assertions_in(&pattern, &frame);
                    let rules = engine2.apply_rules(&pattern, &frame);
                    stream_append(assertions, rules)
                }),
                frames,
            )
        })
    }

    /// Louis's undelayed `disjoin`: the rest disjunction is a value,
    /// not a delay.
    pub fn undelayed_disjoin() -> QProc {
        Rc::new(|engine, disjuncts, frames| {
            if disjuncts.is_nil() {
                return Stream::Empty;
            }
            let (first, rest) = split_list(disjuncts);
            let rest_or = Value::Pair(cons_cell(Value::sym("or"), rest));
            let engine2 = Rc::clone(engine);
            let frames2 = frames.clone();
            interleave(
                qeval_engine(engine, &first, frames),
                qeval_engine(&engine2, &rest_or, frames2),
            )
        })
    }

    /// Installs a fallback wrapping `base` that stops after `fuel`
    /// invocations and raises the flag.
    pub fn fueled_fallback(engine: &Engine, base: QProc, fuel: usize) -> Rc<Cell<bool>> {
        let exhausted = Rc::new(Cell::new(false));
        let counter = Rc::new(Cell::new(0usize));
        let flag = Rc::clone(&exhausted);
        let count = Rc::clone(&counter);
        engine.set_fallback(Some(Rc::new(move |eng, pattern, frames| {
            let n = count.get() + 1;
            count.set(n);
            if n > fuel {
                flag.set(true);
                return Stream::Empty;
            }
            base(eng, pattern, frames)
        })));
        exhausted
    }
}

#[test]
fn ex_4_71() {
    let louis_rule = "(rule (outranked-by ?staff-person ?boss) \
     (or (supervisor ?staff-person ?boss) \
     (and (outranked-by ?middle-manager ?boss) \
     (supervisor ?staff-person ?middle-manager))))";
    let ben_query = "(outranked-by (Bitdiddle Ben) ?who)";

    // The delayed engine: the first answer streams, and only the hunt
    // for a second diverges.
    let delayed = microshaft();
    delayed.load(&[louis_rule]);
    let standard = delayed.simple_query_proc();
    let delayed_out = ex_4_71::fueled_fallback(&delayed, standard, 250);
    assert_eq!(
        delayed.answers_upto(ben_query, 1),
        ["(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"]
    );
    let _ = delayed.answers_upto(ben_query, 2);
    assert!(delayed_out.get());

    // The same divergence through undelayed disjoin: the `or` of a
    // finite query with Louis's rule -- the rest disjunction is a value
    // that must be constructed, so the supervisor answers never emerge.
    let delayed_or = microshaft();
    delayed_or.load(&[louis_rule]);
    let standard_or = delayed_or.simple_query_proc();
    let _delayed_or_out = ex_4_71::fueled_fallback(&delayed_or, standard_or, 800);
    assert_eq!(
        delayed_or.answers_upto(
            "(or (supervisor ?x (Bitdiddle Ben)) (outranked-by (Bitdiddle Ben) ?who))",
            3
        ),
        [
            "(or (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (outranked-by (Bitdiddle Ben) ?who))",
            "(or (supervisor ?x (Bitdiddle Ben)) (outranked-by (Bitdiddle Ben) (Warbucks Oliver)))",
            "(or (supervisor (Fect Cy D) (Bitdiddle Ben)) (outranked-by (Bitdiddle Ben) ?who))",
        ]
    );
}

#[test]
fn ex_4_71a() {
    let louis_rule = "(rule (outranked-by ?staff-person ?boss) \
     (or (supervisor ?staff-person ?boss) \
     (and (outranked-by ?middle-manager ?boss) \
     (supervisor ?staff-person ?middle-manager))))";
    let ben_query = "(outranked-by (Bitdiddle Ben) ?who)";

    // Louis's undelayed simple query: in this port the measured result
    // is that the first answer still comes out -- a Rust stream value is
    // already a lazy structure, so the eager Scheme argument evaluation
    // that makes the undelayed definitions diverge at construction has
    // no counterpart here. The fuel budget spent hunting answers past
    // the first is the same for both engines.
    let undelayed = microshaft();
    undelayed.load(&[louis_rule]);
    let undelayed_out =
        ex_4_71::fueled_fallback(&undelayed, ex_4_71::undelayed_simple_query(), 250);
    assert_eq!(
        undelayed.answers_upto(ben_query, 1),
        ["(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"]
    );
    let _ = undelayed.answers_upto(ben_query, 2);
    assert!(
        undelayed_out.get(),
        "the hunt for a second answer exhausts fuel"
    );

    let undelayed_or = microshaft();
    undelayed_or.load(&[louis_rule]);
    let both = ex_4_71::undelayed_simple_query();
    let undelayed_or_out = ex_4_71::fueled_fallback(&undelayed_or, both, 250);
    undelayed_or.put("or", ex_4_71::undelayed_disjoin());
    let or_query = "(or (supervisor ?x (Bitdiddle Ben)) (outranked-by (Bitdiddle Ben) ?who))";
    // In this port the undelayed or also streams the supervisor answers
    // -- the structure construction terminates -- and the fuel burns
    // once the recursive branch is consumed.
    let or_undelayed = undelayed_or.answers_upto(or_query, 2);
    assert!(!or_undelayed.is_empty() && or_undelayed[0].contains("(Hacker Alyssa P)"));
    assert!(undelayed_or_out.get());
}
