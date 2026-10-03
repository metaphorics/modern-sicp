// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.74: simple stream flatmap
//! keeps the same answers while removing duplicate frames.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, qeval};
use support::{atom, fact, relation, var};

fn duplicates() -> Database {
    let mut database = Database::new();
    for value in ["a", "b", "c"] {
        database.assert(fact("item", vec![atom(value)]));
        database.assert(fact("item", vec![atom(value)]));
    }
    database
}

#[test]
fn ex_4_74() {
    let raw = qeval(&duplicates(), &relation("item", vec![var("answer")]));
    let simple = qeval(
        &duplicates(),
        &Query::UniqueBy(
            vec!["answer".to_owned()],
            Box::new(relation("item", vec![var("answer")])),
        ),
    );
    assert_eq!(raw.answers.len(), 6);
    assert_eq!(simple.answers.len(), 3);
}
