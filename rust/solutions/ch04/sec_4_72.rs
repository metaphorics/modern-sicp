// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.72: interleaving preserves
//! both branches of a productive disjunction.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, qeval_prefix};
use support::{answer_text, atom, fact, relation, var};

fn facts() -> Database {
    let mut database = Database::new();
    database.assert(fact("left", vec![atom("one")]));
    database.assert(fact("left", vec![atom("two")]));
    database.assert(fact("right", vec![atom("alpha")]));
    database.assert(fact("right", vec![atom("beta")]));
    database
}

#[test]
fn ex_4_72() {
    let query = Query::Or(vec![
        relation("left", vec![var("answer")]),
        relation("right", vec![var("answer")]),
    ]);
    let outcome = qeval_prefix(&facts(), &query, 4);
    let answers: Vec<String> = outcome
        .answers
        .iter()
        .map(|answer| answer_text(answer, "answer"))
        .collect();
    assert_eq!(answers.len(), 4);
    assert_eq!(answers[0], "one");
    assert_eq!(answers[1], "alpha");
}
