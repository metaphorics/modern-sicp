// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.75: `Unique` answers one
//! frame even when the underlying query has many.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, qeval};
use support::{atom, fact, relation, var};

fn items() -> Database {
    let mut database = Database::new();
    for value in ["a", "b", "c"] {
        database.assert(fact("item", vec![atom(value)]));
    }
    database
}

#[test]
fn ex_4_75() {
    let outcome = qeval(
        &items(),
        &Query::Unique(Box::new(relation("item", vec![var("answer")]))),
    );
    assert_eq!(outcome.answers.len(), 1);
}
