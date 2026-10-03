// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.73: delayed flattening keeps
//! nested answers ordered and finite at a prefix.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, qeval_prefix};
use support::{answer_text, atom, fact, relation, var};

fn nested() -> Database {
    let mut database = Database::new();
    database.assert(fact("outer", vec![atom("a")]));
    database.assert(fact("outer", vec![atom("b")]));
    database.assert(fact("inner", vec![atom("a"), atom("x")]));
    database.assert(fact("inner", vec![atom("b"), atom("y")]));
    database
}

#[test]
fn ex_4_73() {
    let query = Query::Or(vec![
        Query::And(vec![
            relation("outer", vec![var("outer")]),
            relation("inner", vec![var("outer"), var("answer")]),
        ]),
        relation("outer", vec![var("answer")]),
    ]);
    let outcome = qeval_prefix(&nested(), &query, 4);
    let answers: Vec<String> = outcome
        .answers
        .iter()
        .map(|answer| answer_text(answer, "answer"))
        .collect();
    assert_eq!(answers.len(), 4);
}
