// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.79: rule applications rename
//! variables apart instead of sharing one mutable environment.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, qeval, unify};
use sicp_runtime::host::query::Term;
use support::{answer_text, atom, fact, pair, relation, rule, var};

fn scoped() -> Database {
    let mut database = Database::new();
    database.assert(fact("item", vec![atom("a")]));
    database.assert(fact("item", vec![atom("b")]));
    database.add_rule(rule(
        fact("equal_item", vec![var("left"), var("right")]),
        vec![
            relation("item", vec![var("left")]),
            Query::Unify(var("left"), var("right")),
        ],
    ));
    database
}

#[test]
fn ex_4_79() {
    let outcome = qeval(
        &scoped(),
        &relation("equal_item", vec![var("left"), var("right")]),
    );
    assert_eq!(outcome.answers.len(), 2);
    assert_eq!(
        answer_text(&outcome.answers[0], "left"),
        answer_text(&outcome.answers[0], "right")
    );
    assert_eq!(
        unify(
            &var("x"),
            &pair(var("x"), Term::Empty),
            &std::collections::HashMap::default(),
        ),
        None
    );
}
