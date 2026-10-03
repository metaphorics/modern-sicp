// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Section 4.4.4.2: the evaluator: compound queries and the filters.
//! Dispatch is data: each `Query` constructor is one clause of the
//! evaluator's `match`, and the `Value` filter keeps only the frames
//! whose bound terms satisfy the host predicate.

pub mod common;

use ch04::sec_4_4::{Predicate, Query, qeval, qeval_prefix, reference_answers};
use common::{microshaft, person, var};
use sicp_runtime::host::query::Term;

fn relation(name: &str, arguments: Vec<Term>) -> Query {
    Query::Relation {
        name: name.to_owned(),
        arguments,
    }
}

fn supervised_by(who: Term) -> Query {
    relation("supervisor", vec![var("x"), who])
}

fn main() {
    let database = microshaft();

    // The compound query of the book's and/or discussion: the or of
    // Ben's reports and Alyssa's trainee interleaves its disjuncts.
    let query = Query::Or(vec![
        supervised_by(person(&["Bitdiddle", "Ben"])),
        supervised_by(person(&["Hacker", "Alyssa", "P"])),
    ]);
    let answers = qeval(&database, &query).answers;
    assert_eq!(answers.len(), 4);
    let names: Vec<Term> = answers
        .iter()
        .map(|frame| common::resolve(frame, "x"))
        .collect();
    assert_eq!(names[0], person(&["Hacker", "Alyssa", "P"]));
    assert_eq!(names[1], person(&["Reasoner", "Louis"]));

    // The `Value` filter keeps only instantiated salaries above 30000:
    // five of the nine people qualify.
    let rich = qeval(
        &database,
        &Query::And(vec![
            relation("salary", vec![var("person"), var("amount")]),
            Query::Value(
                Predicate::Gt(var("amount"), Term::Integer(30000)),
                vec![var("amount"), Term::Integer(30000)],
            ),
        ]),
    )
    .answers;
    assert_eq!(rich.len(), 5);

    // Order matters: the host predicate sees an unbound `amount` before
    // the salary relation binds it, so the frame fails.
    let filtered_before_binding = qeval(
        &database,
        &Query::And(vec![
            Query::Value(
                Predicate::Gt(var("amount"), Term::Integer(30000)),
                vec![var("amount"), Term::Integer(30000)],
            ),
            relation("salary", vec![var("person"), var("amount")]),
        ]),
    )
    .answers;
    assert_eq!(filtered_before_binding.len(), 0);

    // Negation as failure: Oliver has no supervisor on file, so the
    // negation answers once; Ben does, so it answers never.
    let no_boss = |who: Term| Query::Not(Box::new(relation("supervisor", vec![who, var("chief")])));
    assert_eq!(
        qeval(&database, &no_boss(person(&["Warbucks", "Oliver"])))
            .answers
            .len(),
        1
    );
    assert_eq!(
        qeval(&database, &no_boss(person(&["Bitdiddle", "Ben"])))
            .answers
            .len(),
        0
    );

    let prefix = qeval_prefix(
        &database,
        &relation("supervisor", vec![var("x"), var("y")]),
        2,
    );
    assert_eq!(prefix.answers.len(), 2);

    let all = qeval(&database, &relation("supervisor", vec![var("x"), var("y")])).answers;
    assert_eq!(all.len(), 8);

    assert_eq!(
        reference_answers(&database, &query),
        qeval(&database, &query).answers
    );
}
