// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.57: the `can_replace` rule
//! composes job and salary facts.

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
        "salary",
        vec![atom("Bitdiddle Ben"), Term::Integer(60_000)],
    ));
    database.assert(fact(
        "salary",
        vec![atom("Hacker Alyssa P"), Term::Integer(40_000)],
    ));
    database.assert(fact(
        "can_do_job",
        vec![
            list(vec![atom("computer"), atom("wizard")]),
            list(vec![atom("computer"), atom("programmer")]),
        ],
    ));
    database.add_rule(rule(
        fact("can_replace", vec![var("person"), var("replacement")]),
        vec![
            relation("job", vec![var("person"), var("person_job")]),
            relation("job", vec![var("replacement"), var("replacement_job")]),
            Query::Or(vec![
                Query::Unify(var("person_job"), var("replacement_job")),
                relation(
                    "can_do_job",
                    vec![var("person_job"), var("replacement_job")],
                ),
            ]),
            Query::Value(
                Predicate::Ne(
                    Term::Variable("person".to_owned()),
                    Term::Variable("replacement".to_owned()),
                ),
                vec![],
            ),
            relation("salary", vec![var("person"), var("person_salary")]),
            relation(
                "salary",
                vec![var("replacement"), var("replacement_salary")],
            ),
            Query::Value(
                Predicate::Lt(
                    Term::Variable("replacement_salary".to_owned()),
                    Term::Variable("person_salary".to_owned()),
                ),
                vec![],
            ),
        ],
    ));
    database
}

#[test]
fn ex_4_57() {
    let outcome = qeval(
        &microshaft(),
        &relation("can_replace", vec![var("person"), var("replacement")]),
    );
    let people: Vec<String> = outcome
        .answers
        .iter()
        .map(|answer| {
            format!(
                "{} -> {}",
                answer_text(answer, "person"),
                answer_text(answer, "replacement")
            )
        })
        .collect();
    assert_eq!(people, vec!["Bitdiddle Ben -> Hacker Alyssa P"]);
}
