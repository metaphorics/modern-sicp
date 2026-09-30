// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.55: simple Microshaft queries
//! over typed assertions.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, qeval};
use support::{answer_text, atom, fact, list, relation, var};

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
            atom("Fect Cy D"),
            list(vec![atom("computer"), atom("programmer")]),
        ],
    ));
    database.assert(fact(
        "job",
        vec![
            atom("Tweakit Lem E"),
            list(vec![atom("computer"), atom("technician")]),
        ],
    ));
    database
}

#[test]
fn ex_4_55() {
    let outcome = qeval(
        &microshaft(),
        &relation(
            "job",
            vec![
                var("person"),
                list(vec![atom("computer"), atom("programmer")]),
            ],
        ),
    );
    let people: Vec<String> = outcome
        .answers
        .iter()
        .map(|answer| answer_text(answer, "person"))
        .collect();
    assert_eq!(people, vec!["Hacker Alyssa P", "Fect Cy D"]);
}
