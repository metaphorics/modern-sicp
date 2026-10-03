// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.61: `last_pair` as two typed
//! rules over pair terms.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, qeval};
use sicp_runtime::host::query::Term;
use support::{answer_text, atom, fact, list, pair, relation, rule, var};

fn last_pair_rules() -> Database {
    let mut database = Database::new();
    database.add_rule(rule(
        fact(
            "last_pair",
            vec![pair(var("item"), Term::Empty), var("item")],
        ),
        vec![],
    ));
    database.add_rule(rule(
        fact(
            "last_pair",
            vec![pair(var("item"), var("rest")), var("last")],
        ),
        vec![relation("last_pair", vec![var("rest"), var("last")])],
    ));
    database
}

#[test]
fn ex_4_61() {
    let query = relation(
        "last_pair",
        vec![list(vec![atom("a"), atom("b"), atom("c")]), var("last")],
    );
    let outcome = qeval(&last_pair_rules(), &query);
    assert_eq!(outcome.answers.len(), 1);
    assert_eq!(answer_text(&outcome.answers[0], "last"), "c");
}
