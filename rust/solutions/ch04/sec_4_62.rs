// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.62: logic gates expressed as
//! typed rules and facts.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, qeval};
use support::{answer_text, atom, fact, relation, rule, var};

fn gates() -> Database {
    let mut database = Database::new();
    database.assert(fact(
        "gate",
        vec![atom("and"), atom("false"), atom("false"), atom("false")],
    ));
    database.assert(fact(
        "gate",
        vec![atom("and"), atom("false"), atom("true"), atom("false")],
    ));
    database.assert(fact(
        "gate",
        vec![atom("and"), atom("true"), atom("false"), atom("false")],
    ));
    database.assert(fact(
        "gate",
        vec![atom("and"), atom("true"), atom("true"), atom("true")],
    ));
    database.assert(fact(
        "gate",
        vec![atom("not"), atom("false"), var("ignored"), atom("true")],
    ));
    database.assert(fact(
        "gate",
        vec![atom("not"), atom("true"), var("ignored"), atom("false")],
    ));
    database.add_rule(rule(
        fact("gate", vec![atom("or"), var("a"), var("b"), var("out")]),
        vec![
            relation(
                "gate",
                vec![atom("not"), var("a"), var("ignored_a"), var("not_a")],
            ),
            relation(
                "gate",
                vec![atom("not"), var("b"), var("ignored_b"), var("not_b")],
            ),
            relation(
                "gate",
                vec![atom("and"), var("not_a"), var("not_b"), var("neither")],
            ),
            relation(
                "gate",
                vec![atom("not"), var("neither"), var("ignored_c"), var("out")],
            ),
        ],
    ));
    database
}

#[test]
fn ex_4_62() {
    let outcome = qeval(
        &gates(),
        &relation(
            "gate",
            vec![atom("or"), atom("false"), atom("true"), var("out")],
        ),
    );
    assert_eq!(answer_text(&outcome.answers[0], "out"), "true");
}
