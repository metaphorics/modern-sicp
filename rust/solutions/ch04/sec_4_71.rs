// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.71: delayed and undelayed
//! query production over the Microshaft `outranked-by` rule under the
//! same explicit fuel budget.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Query, QueryRunReport, qeval_bounded, qeval_undelayed_bounded};
use support::{and, answer_text, atom, fact, relation, rule, var};

const FUEL: u64 = 128;

fn microshaft() -> Database {
    let mut database = Database::new();
    for (person, boss) in [
        ("Bitdiddle Ben", "Warbucks Oliver"),
        ("Hacker Alyssa P", "Bitdiddle Ben"),
        ("Fect Cy D", "Bitdiddle Ben"),
        ("Tweakit Lem E", "Bitdiddle Ben"),
        ("Reasoner Louis", "Hacker Alyssa P"),
        ("Scrooge Eben", "Warbucks Oliver"),
        ("Cratchet Robert", "Scrooge Eben"),
        ("Aull DeWitt", "Warbucks Oliver"),
    ] {
        database.assert(fact("supervisor", vec![atom(person), atom(boss)]));
    }
    database.add_rule(rule(
        fact("outranked_by", vec![var("staff"), var("boss")]),
        vec![Query::Or(vec![
            relation("supervisor", vec![var("staff"), var("boss")]),
            and(vec![
                relation("supervisor", vec![var("staff"), var("middle")]),
                relation("outranked_by", vec![var("middle"), var("boss")]),
            ]),
        ])],
    ));
    database
}

fn ben_query() -> Query {
    relation("outranked_by", vec![atom("Bitdiddle Ben"), var("boss")])
}

fn assert_probe(report: &QueryRunReport) {
    assert_eq!(report.answers.len(), 1);
    assert_eq!(answer_text(&report.answers[0], "boss"), "Warbucks Oliver");
    assert!(report.steps <= FUEL);
}

#[test]
fn ex_4_71() {
    let delayed = qeval_bounded(&microshaft(), &ben_query(), FUEL);
    let undelayed = qeval_undelayed_bounded(&microshaft(), &ben_query(), FUEL);
    assert_probe(&delayed);
    assert_probe(&undelayed);
}

#[test]
fn ex_4_71a() {
    let delayed = qeval_bounded(&microshaft(), &ben_query(), FUEL);
    let undelayed = qeval_undelayed_bounded(&microshaft(), &ben_query(), FUEL);
    assert_probe(&delayed);
    assert_probe(&undelayed);
    assert!(delayed.steps > 0);
    assert!(undelayed.steps > 0);
}
