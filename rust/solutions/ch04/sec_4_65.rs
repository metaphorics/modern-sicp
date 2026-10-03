// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.65: the wheel rule produces
//! duplicate answers unless uniqueness is explicit.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, qeval};
use support::{answer_text, atom, fact, relation, rule, var};

fn organization() -> Database {
    let mut database = Database::new();
    for (staff, boss) in [
        ("Bitdiddle Ben", "Warbucks Oliver"),
        ("Hacker Alyssa P", "Bitdiddle Ben"),
        ("Fect Cy D", "Bitdiddle Ben"),
        ("Tweakit Lem E", "Bitdiddle Ben"),
    ] {
        database.assert(fact("supervisor", vec![atom(staff), atom(boss)]));
    }
    database.add_rule(rule(
        fact("wheel", vec![var("person")]),
        vec![
            relation("supervisor", vec![var("middle"), var("person")]),
            relation("supervisor", vec![var("staff"), var("middle")]),
        ],
    ));
    database
}

#[test]
fn ex_4_65() {
    let outcome = qeval(&organization(), &relation("wheel", vec![var("person")]));
    assert!(outcome.answers.len() >= 2);
    let unique = qeval(
        &organization(),
        &Query::UniqueBy(
            vec!["person".to_owned()],
            Box::new(relation("wheel", vec![var("person")])),
        ),
    );
    assert_eq!(unique.answers.len(), 1);
    assert_eq!(answer_text(&unique.answers[0], "person"), "Warbucks Oliver");
}
