// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.76: `and` merges compatible
//! frames from independent conjuncts.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, qeval};
use support::{answer_text, atom, fact, relation, var};

fn facts() -> Database {
    let mut database = Database::new();
    database.assert(fact("left", vec![atom("a"), atom("x")]));
    database.assert(fact("left", vec![atom("b"), atom("y")]));
    database.assert(fact("right", vec![atom("x"), atom("second")]));
    database.assert(fact("right", vec![atom("y"), atom("second")]));
    database
}

#[test]
fn ex_4_76() {
    let query = Query::And(vec![
        relation("left", vec![var("name"), var("key")]),
        relation("right", vec![var("key"), var("suffix")]),
    ]);
    let outcome = qeval(&facts(), &query);
    assert_eq!(outcome.answers.len(), 2);
    assert_eq!(answer_text(&outcome.answers[0], "suffix"), "second");
}
