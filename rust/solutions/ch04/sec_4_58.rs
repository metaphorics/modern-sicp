// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.58: the `big_shot` rule keeps
//! a person whose supervisor is in another division.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::Predicate;
use ch04::sec_4_4::{Database, Query, qeval};
use sicp_runtime::host::query::Term;
use support::{answer_text, atom, fact, list, relation, rule, var};

fn microshaft() -> Database {
    let mut database = Database::new();
    database.assert(fact(
        "job",
        vec![
            atom("Bitdiddle Ben"),
            list(vec![atom("computer"), atom("wizard")]),
        ],
    ));
    database.assert(fact(
        "job",
        vec![
            atom("Hacker Alyssa P"),
            list(vec![atom("computer"), atom("programmer")]),
        ],
    ));
    database.assert(fact(
        "job",
        vec![
            atom("Warbucks Oliver"),
            list(vec![atom("administration"), atom("bigwheel")]),
        ],
    ));
    database.assert(fact(
        "division",
        vec![atom("Bitdiddle Ben"), atom("computer")],
    ));
    database.assert(fact(
        "division",
        vec![atom("Hacker Alyssa P"), atom("computer")],
    ));
    database.assert(fact(
        "supervisor",
        vec![atom("Hacker Alyssa P"), atom("Bitdiddle Ben")],
    ));
    database.assert(fact(
        "supervisor",
        vec![atom("Bitdiddle Ben"), atom("Warbucks Oliver")],
    ));
    database.add_rule(rule(
        fact("big_shot", vec![var("person"), var("division")]),
        vec![
            relation(
                "job",
                vec![var("person"), list(vec![var("division"), var("title")])],
            ),
            relation("supervisor", vec![var("person"), var("boss")]),
            relation(
                "job",
                vec![
                    var("boss"),
                    list(vec![var("boss_division"), var("boss_title")]),
                ],
            ),
            Query::Value(
                Predicate::Ne(
                    Term::Variable("division".to_owned()),
                    Term::Variable("boss_division".to_owned()),
                ),
                vec![],
            ),
        ],
    ));
    database
}

#[test]
fn ex_4_58() {
    let outcome = qeval(
        &microshaft(),
        &relation("big_shot", vec![var("person"), var("division")]),
    );
    assert_eq!(outcome.answers.len(), 1);
    assert_eq!(answer_text(&outcome.answers[0], "person"), "Bitdiddle Ben");
}
