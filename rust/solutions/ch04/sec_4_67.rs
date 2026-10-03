// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.67: a query loop detector
//! stops recursive answers at an explicit prefix.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, qeval_prefix};
use support::{fact, relation, rule, var};

fn recursive() -> Database {
    let mut database = Database::new();
    database.add_rule(rule(
        fact("ancestor", vec![var("person"), var("person")]),
        vec![],
    ));
    database.add_rule(rule(
        fact("ancestor", vec![var("person"), var("ancestor")]),
        vec![Query::And(vec![
            relation("ancestor", vec![var("person"), var("middle")]),
            relation("ancestor", vec![var("middle"), var("ancestor")]),
        ])],
    ));
    database
}

#[test]
fn ex_4_67() {
    let outcome = qeval_prefix(
        &recursive(),
        &relation("ancestor", vec![var("a"), var("b")]),
        3,
    );
    assert_eq!(outcome.answers.len(), 3);
}
