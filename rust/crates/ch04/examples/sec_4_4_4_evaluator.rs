// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Section 4.4.4.2: the evaluator: data-directed dispatch, compound
//! queries, and the filters.

use ch04::sec_4_4::{Frame, microshaft, qeval_engine, singleton_stream};

fn main() {
    let engine = microshaft();

    // The compound query of the book's and/or discussion: the two
    // orders of the filters answer the same frames here, and the or
    // interleaves its disjuncts.
    let answers =
        engine.answers("(or (supervisor ?x (Bitdiddle Ben)) (supervisor ?x (Hacker Alyssa P)))");
    println!("@{answers:?}");
    assert_eq!(answers.len(), 4);
    assert!(answers[0].contains("(Hacker Alyssa P)"));
    assert!(answers[1].contains("(Reasoner Louis)"));

    // lisp-value filters instantiated salaries.
    let rich = engine.answers("(and (salary ?person ?amount) (lisp-value > ?amount 30000))");
    assert_eq!(rich.len(), 5);

    // The dispatch table is data: the standard processors are visible.
    assert!(engine.get("and").is_some());
    assert!(engine.get("or").is_some());
    assert!(engine.get("not").is_some());
    assert!(engine.get("lisp-value").is_some());
    assert!(engine.get("always-true").is_some());

    // The evaluator answers from any frame stream: a singleton carrying
    // a partial binding.
    let frames = qeval_engine(
        &engine,
        &ch04::sec_4_4::read_query("(supervisor ?x ?y)"),
        singleton_stream(Frame::new()),
    );
    assert_eq!(frames.iter().count(), 8);
}
