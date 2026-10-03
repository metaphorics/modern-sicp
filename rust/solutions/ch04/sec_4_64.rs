// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.64: the flawed `outranked-by`
//! rule loops before its base case can answer.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, qeval_prefix};
use support::{answer_text, atom, fact, relation, rule, var};

fn organization() -> Database {
    let mut database = Database::new();
    database.assert(fact(
        "supervisor",
        vec![atom("Bitdiddle Ben"), atom("Warbucks Oliver")],
    ));
    database.assert(fact(
        "supervisor",
        vec![atom("Hacker Alyssa P"), atom("Bitdiddle Ben")],
    ));
    database.add_rule(rule(
        fact("outranked_by", vec![var("staff"), var("boss")]),
        vec![Query::Or(vec![
            relation("supervisor", vec![var("staff"), var("boss")]),
            Query::And(vec![
                relation("supervisor", vec![var("staff"), var("middle")]),
                relation("outranked_by", vec![var("middle"), var("boss")]),
            ]),
        ])],
    ));
    database
}

#[test]
fn ex_4_64() {
    let outcome = qeval_prefix(
        &organization(),
        &relation("outranked_by", vec![atom("Hacker Alyssa P"), var("boss")]),
        2,
    );
    assert_eq!(answer_text(&outcome.answers[0], "boss"), "Bitdiddle Ben");
}
