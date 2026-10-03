// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.60: `lives-near` deduplicates
//! symmetric pairs with an explicit name-order predicate.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::Predicate;
use ch04::sec_4_4::{Database, Query, qeval};
use sicp_runtime::host::query::Term;
use support::{answer_text, atom, fact, list, relation, rule, var};

fn addresses() -> Database {
    let mut database = Database::new();
    database.assert(fact(
        "address",
        vec![
            atom("Bitdiddle Ben"),
            list(vec![
                atom("Slumerville"),
                atom("Ridge Road"),
                Term::Integer(10),
            ]),
        ],
    ));
    database.assert(fact(
        "address",
        vec![
            atom("Hacker Alyssa P"),
            list(vec![atom("Cambridge"), atom("Mass Ave"), Term::Integer(78)]),
        ],
    ));
    database.assert(fact(
        "address",
        vec![
            atom("Fect Cy D"),
            list(vec![atom("Cambridge"), atom("Mass Ave"), Term::Integer(78)]),
        ],
    ));
    database.add_rule(rule(
        fact("lives_near", vec![var("person_1"), var("person_2")]),
        vec![
            relation(
                "address",
                vec![
                    var("person_1"),
                    list(vec![var("town_1"), var("street_1"), var("number_1")]),
                ],
            ),
            relation(
                "address",
                vec![
                    var("person_2"),
                    list(vec![var("town_2"), var("street_2"), var("number_2")]),
                ],
            ),
            Query::Value(
                Predicate::Eq(
                    Term::Variable("town_1".to_owned()),
                    Term::Variable("town_2".to_owned()),
                ),
                vec![],
            ),
            Query::Value(
                Predicate::TextLt(
                    Term::Variable("person_1".to_owned()),
                    Term::Variable("person_2".to_owned()),
                ),
                vec![],
            ),
        ],
    ));
    database
}

#[test]
fn ex_4_60() {
    let outcome = qeval(
        &addresses(),
        &relation("lives_near", vec![var("person_1"), var("person_2")]),
    );
    assert_eq!(outcome.answers.len(), 1);
    assert_eq!(answer_text(&outcome.answers[0], "person_1"), "Fect Cy D");
}
