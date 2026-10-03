// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.69: great-grandson is a rule
//! composition over parent facts.

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
        fact(
            "great_grandson",
            vec![var("great_grandson"), var("great_grandfather")],
        ),
        vec![
            relation("parent", vec![var("great_grandfather"), var("child")]),
            relation("parent", vec![var("child"), var("grandchild")]),
            relation("parent", vec![var("grandchild"), var("great_grandson")]),
        ],
    ));
    database
}

#[test]
fn ex_4_69() {
    let outcome = qeval(
        &family(),
        &relation("great_grandson", vec![var("great_grandson"), atom("adam")]),
    );
    assert_eq!(outcome.answers.len(), 1);
    assert_eq!(answer_text(&outcome.answers[0], "great_grandson"), "irad");
}
