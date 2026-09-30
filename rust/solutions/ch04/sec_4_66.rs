// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.66: accumulation reads the
//! values a surviving frame binds.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::Predicate;
use ch04::sec_4_4::{Database, Query, qeval};
use support::{atom, fact, int, relation, var};

fn values() -> Database {
    let mut database = Database::new();
    database.assert(fact("value", vec![atom("a"), int(1)]));
    database.assert(fact("value", vec![atom("b"), int(2)]));
    database.assert(fact("value", vec![atom("c"), int(3)]));
    database
}

#[test]
fn ex_4_66() {
    let query = Query::And(vec![
        relation("value", vec![atom("a"), var("left")]),
        relation("value", vec![atom("b"), var("right")]),
        Query::Value(Predicate::SumEq(vec![var("left"), var("right")], 3), vec![]),
    ]);
    let outcome = qeval(&values(), &query);
    assert_eq!(outcome.answers.len(), 1);
}
