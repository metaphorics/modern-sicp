// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.63: family relations as
//! typed rules over parent facts.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, qeval};
use support::{answer_text, atom, fact, relation, rule, var};

fn family() -> Database {
    let mut database = Database::new();
    database.assert(fact("parent", vec![atom("adam"), atom("cain")]));
    database.assert(fact("parent", vec![atom("cain"), atom("enoch")]));
    database.assert(fact("parent", vec![atom("enoch"), atom("irad")]));
    database.add_rule(rule(
        fact("child", vec![var("child"), var("parent")]),
        vec![relation("parent", vec![var("parent"), var("child")])],
    ));
    database.add_rule(rule(
        fact("grandson", vec![var("grandson"), var("grandfather")]),
        vec![
            relation("parent", vec![var("grandfather"), var("parent")]),
            relation("parent", vec![var("parent"), var("grandson")]),
        ],
    ));
    database
}

#[test]
fn ex_4_63() {
    let outcome = qeval(
        &family(),
        &relation("grandson", vec![var("grandson"), atom("adam")]),
    );
    assert_eq!(outcome.answers.len(), 1);
    assert_eq!(answer_text(&outcome.answers[0], "grandson"), "enoch");
}
