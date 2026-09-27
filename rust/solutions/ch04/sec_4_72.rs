// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.72: why `disjoin` and
//! `stream-flatmap` interleave rather than append. The appending `or`
//! lets one disjunct's endless stream starve every later disjunct; the
//! interleaved `or` alternates, so each disjunct's answers keep
//! arriving. The probe runs one query under both processors over the
//! married cycle, whose stream never runs dry.

use std::rc::Rc;

use ch04::sec_4_4::{Engine, QProc, microshaft, qeval_engine, split_list, stream_append};
use sicp_runtime::{Stream, Value, cons_cell};

mod ex_4_72 {
    //! Exercise 4.72: the appending `or` and the starvation it causes.

    use super::*;

    /// The `or` with plain `stream-append` in place of
    /// `interleave-delayed`: the exercise's counterfactual.
    pub fn appending_disjoin() -> QProc {
        Rc::new(|engine, disjuncts, frames| {
            if disjuncts.is_nil() {
                return Stream::Empty;
            }
            let (first, rest) = split_list(disjuncts);
            let rest_or = Value::Pair(cons_cell(Value::sym("or"), rest));
            let engine2 = Rc::clone(engine);
            let frames2 = frames.clone();
            stream_append(
                qeval_engine(engine, &first, frames),
                qeval_engine(&engine2, &rest_or, frames2),
            )
        })
    }

    /// An engine whose `or` appends instead of interleaving.
    pub fn appending_engine(rules: &[&str]) -> Engine {
        let engine = microshaft();
        engine.load(rules);
        engine.put("or", appending_disjoin());
        engine
    }
}

#[test]
fn ex_4_72() {
    let rules = [
        "(married Minnie Mickey)",
        "(rule (married ?x ?y) (married ?y ?x))",
    ];
    let query = "(or (married Mickey ?who) (supervisor ?x (Bitdiddle Ben)))";

    // The book's interleaved or: the married stream never runs dry, but
    // the supervisor answers keep arriving -- even positions carry a
    // supervisor's name, odd ones repeat Minnie.
    let interleaved = microshaft();
    interleaved.load(&rules);
    let interleave_answers = interleaved.answers_upto(query, 6);
    let supervised_by = |answer: &str| {
        ["Hacker Alyssa P", "Fect Cy D", "Tweakit Lem E"]
            .iter()
            .any(|who| answer.contains(who))
    };
    assert!(interleave_answers[0].contains("Minnie"));
    assert!(supervised_by(&interleave_answers[1]));
    assert!(interleave_answers[2].contains("Minnie"));
    assert!(supervised_by(&interleave_answers[3]));
    assert!(interleave_answers[4].contains("Minnie"));
    assert!(supervised_by(&interleave_answers[5]));

    // The appending or: the married disjunct's endless stream comes
    // first, and not one supervisor answer arrives in the same budget.
    let appended = ex_4_72::appending_engine(&rules);
    let appended_answers = appended.answers_upto(query, 6);
    assert_eq!(appended_answers.len(), 6);
    assert!(
        appended_answers.iter().all(|answer| !supervised_by(answer)),
        "the second disjunct starves: {appended_answers:?}"
    );
}
