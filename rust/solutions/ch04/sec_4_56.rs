// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.56: the three compound
//! Microshaft queries with concrete ordered substitutions.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::Predicate;
use ch04::sec_4_4::{Database, Query, qeval};
use sicp_runtime::host::query::Term;
use support::{and, answer_text, atom, fact, int, list, not, relation, var};

type Employee = (
    &'static str,
    &'static str,
    &'static str,
    i64,
    &'static str,
    &'static str,
    i64,
    &'static str,
);

fn people() -> [Employee; 9] {
    [
        (
            "Bitdiddle Ben",
            "Slumerville",
            "Ridge Road",
            10,
            "computer",
            "wizard",
            60_000,
            "Warbucks Oliver",
        ),
        (
            "Hacker Alyssa P",
            "Cambridge",
            "Mass Ave",
            78,
            "computer",
            "programmer",
            40_000,
            "Bitdiddle Ben",
        ),
        (
            "Fect Cy D",
            "Cambridge",
            "Ames Street",
            3,
            "computer",
            "programmer",
            35_000,
            "Bitdiddle Ben",
        ),
        (
            "Tweakit Lem E",
            "Boston",
            "Bay State Road",
            22,
            "computer",
            "technician",
            25_000,
            "Bitdiddle Ben",
        ),
        (
            "Reasoner Louis",
            "Slumerville",
            "Pine Tree Road",
            80,
            "computer",
            "programmer trainee",
            30_000,
            "Hacker Alyssa P",
        ),
        (
            "Warbucks Oliver",
            "Swellesley",
            "Top Heap Road",
            0,
            "administration",
            "big wheel",
            150_000,
            "",
        ),
        (
            "Scrooge Eben",
            "Weston",
            "Shady Lane",
            10,
            "accounting",
            "chief accountant",
            75_000,
            "Warbucks Oliver",
        ),
        (
            "Cratchet Robert",
            "Allston",
            "N Harvard Street",
            16,
            "accounting",
            "scrivener",
            18_000,
            "Scrooge Eben",
        ),
        (
            "Aull DeWitt",
            "Slumerville",
            "Onion Square",
            5,
            "administration",
            "secretary",
            25_000,
            "Warbucks Oliver",
        ),
    ]
}

fn microshaft() -> Database {
    let mut database = Database::new();
    for (person, town, street, number, division, title, salary, supervisor) in people() {
        database.assert(fact(
            "address",
            vec![
                atom(person),
                list(vec![atom(town), atom(street), int(number)]),
            ],
        ));
        database.assert(fact(
            "job",
            vec![atom(person), list(vec![atom(division), atom(title)])],
        ));
        database.assert(fact("salary", vec![atom(person), int(salary)]));
        if !supervisor.is_empty() {
            database.assert(fact("supervisor", vec![atom(person), atom(supervisor)]));
        }
    }
    database
}

fn slumerville_addresses() -> Query {
    relation(
        "address",
        vec![
            var("person"),
            list(vec![atom("Slumerville"), var("street"), var("number")]),
        ],
    )
}

fn salaries_below_ben() -> Query {
    and(vec![
        relation("salary", vec![var("person"), var("salary")]),
        relation("salary", vec![atom("Bitdiddle Ben"), var("ben_salary")]),
        Query::Value(
            Predicate::Lt(
                Term::Variable("salary".to_owned()),
                Term::Variable("ben_salary".to_owned()),
            ),
            vec![],
        ),
    ])
}

fn supervised_outside_computer() -> Query {
    and(vec![
        relation("supervisor", vec![var("person"), var("supervisor")]),
        relation("job", vec![var("supervisor"), var("supervisor_job")]),
        not(relation(
            "job",
            vec![var("supervisor"), list(vec![atom("computer"), var("role")])],
        )),
    ])
}

fn assert_addresses(database: &Database) {
    let addresses = qeval(database, &slumerville_addresses());
    let towns: Vec<String> = addresses
        .answers
        .iter()
        .map(|answer| answer_text(answer, "person"))
        .collect();
    assert_eq!(
        towns,
        vec!["Bitdiddle Ben", "Reasoner Louis", "Aull DeWitt"]
    );
}

fn assert_salary_order(database: &Database) {
    let salaries = qeval(database, &salaries_below_ben());
    assert!(salaries.answers.len() > 1);
    assert!(salaries.answers.iter().all(|answer| {
        let salary = answer_text(answer, "salary");
        let ben_salary = answer_text(answer, "ben_salary");
        salary.parse::<i64>().expect("salary integer")
            < ben_salary.parse::<i64>().expect("Ben salary integer")
    }));
    assert!(
        salaries
            .answers
            .iter()
            .any(|answer| answer_text(answer, "person") == "Reasoner Louis")
    );
}

fn assert_supervisors(database: &Database) {
    let supervised = qeval(database, &supervised_outside_computer());
    let people: Vec<String> = supervised
        .answers
        .iter()
        .map(|answer| answer_text(answer, "person"))
        .collect();
    assert_eq!(
        people,
        vec![
            "Bitdiddle Ben",
            "Scrooge Eben",
            "Cratchet Robert",
            "Aull DeWitt"
        ]
    );
}

#[test]
fn ex_4_56() {
    let database = microshaft();
    assert_addresses(&database);
    assert_salary_order(&database);
    assert_supervisors(&database);
}
