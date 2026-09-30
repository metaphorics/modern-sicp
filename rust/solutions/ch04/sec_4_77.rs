// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.77: `not` filters only after
//! its variables are bound.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, qeval_prefix};
use support::{answer_text, atom, fact, list, not, relation, var};

fn jobs() -> Database {
    let mut database = Database::new();
    database.assert(fact(
        "job",
        vec![
            atom("Bitdiddle Ben"),
            list(vec![atom("computer"), atom("wizard")]),
        ],
    ));
    database.assert(fact(
        "job",
        vec![
            atom("Hacker Alyssa P"),
            list(vec![atom("computer"), atom("programmer")]),
        ],
    ));
    database.assert(fact(
        "supervisor",
        vec![atom("Hacker Alyssa P"), atom("Bitdiddle Ben")],
    ));
    database
}

#[test]
fn ex_4_77() {
    let query = Query::And(vec![
        relation("supervisor", vec![var("person"), var("boss")]),
        not(relation(
            "job",
            vec![var("person"), list(vec![atom("computer"), atom("wizard")])],
        )),
    ]);
    let outcome = qeval_prefix(&jobs(), &query, 2);
    assert_eq!(outcome.answers.len(), 1);
    assert_eq!(
        answer_text(&outcome.answers[0], "person"),
        "Hacker Alyssa P"
    );
}
