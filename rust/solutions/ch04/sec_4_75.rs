// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.75: the `unique` special form.
//! `uniquely-asserted` keeps only the frames whose extension of the
//! inner query holds exactly once, and `qeval`'s data-directed table
//! makes the dispatch a one-line `put`.

use std::rc::Rc;

use ch04::sec_4_4::{Engine, QProc, microshaft, singleton_stream};
use sicp_runtime::Stream;

mod ex_4_75 {
    //! Exercise 4.75: the `unique` special form.

    use super::*;

    /// The book's `uniquely-asserted`: keep the frames the inner query
    /// extends exactly once.
    pub fn uniquely_asserted() -> QProc {
        Rc::new(|engine, contents, frames| {
            let unique_query = match contents {
                Value::Pair(cell) => cell.car.borrow().clone(),
                other => other.clone(),
            };
            let engine2 = Rc::clone(engine);
            engine.flatmap(
                Rc::new(move |frame: &Frame| {
                    let extensions: Vec<Frame> = engine2
                        .qeval(&unique_query, singleton_stream(frame.clone()))
                        .iter()
                        .collect();
                    match extensions.as_slice() {
                        // The book's driver instantiates the `unique`
                        // form against the one extension, so the
                        // extension frame is what flows on.
                        [extension] => singleton_stream(extension.clone()),
                        _ => Stream::Empty,
                    }
                }),
                frames,
            )
        })
    }

    /// One Microshaft engine with `unique` installed.
    pub fn engine() -> Engine {
        let engine = microshaft();
        engine.put("unique", uniquely_asserted());
        engine
    }
}

use ch04::sec_4_4::Frame;
use sicp_runtime::Value;

#[test]
fn ex_4_75() {
    // The book's first case: Ben is the only computer wizard.
    assert_eq!(
        ex_4_75::engine().answers("(unique (job ?x (computer wizard)))"),
        ["(unique (job (Bitdiddle Ben) (computer wizard)))"]
    );
    // The second: more than one computer programmer, so nothing.
    assert!(
        ex_4_75::engine()
            .answers("(unique (job ?x (computer programmer)))")
            .is_empty()
    );
    // Jobs filled by exactly one person, with the people who fill them.
    let filled_once = ex_4_75::engine().answers("(and (job ?x ?j) (unique (job ?anyone ?j)))");
    assert_eq!(filled_once.len(), 7, "every job but the programmers'");
    assert!(filled_once[0].contains("(job (Bitdiddle Ben) (computer wizard))"));
    assert!(
        filled_once
            .iter()
            .all(|answer| !answer.contains("programmer)"))
    );

    // The exercise's closing test: people who supervise precisely one
    // person.
    assert_eq!(
        ex_4_75::engine()
            .answers("(and (supervisor ?x ?boss) (unique (supervisor ?anyone ?boss)))")
            .len(),
        2,
        "Alyssa supervises only Louis, Scrooge only Cratchet"
    );
}
