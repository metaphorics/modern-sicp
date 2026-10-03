// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Section 4.4.4.1: the driver loop and instantiation: filing an
//! assertion, answering queries, and printing unbound variables as
//! `?name`.

pub mod common;

use ch04::sec_4_4::{Query, Rule, qeval};
use common::{atom, fact, list, microshaft, person, render, render_query, resolve, var};

fn relation(name: &str, arguments: Vec<ch04::sec_4_4::Term>) -> Query {
    Query::Relation {
        name: name.to_owned(),
        arguments,
    }
}

fn main() {
    let database = microshaft();

    // A query streams its answers; each answer renders as the book's
    // instantiated query. Ben's direct reports are the three people
    // supervised by `(Bitdiddle Ben)`, in assertion order.
    let query = relation(
        "supervisor",
        vec![var("who"), person(&["Bitdiddle", "Ben"])],
    );
    let answers = qeval(&database, &query).answers;
    let rendered: Vec<String> = answers
        .iter()
        .map(|frame| {
            render_query(
                "supervisor",
                &[var("who"), person(&["Bitdiddle", "Ben"])],
                frame,
            )
        })
        .collect();
    assert_eq!(answers.len(), 3);
    assert_eq!(
        rendered[0],
        "(supervisor (Hacker Alyssa P) (Bitdiddle Ben))"
    );

    // Filing an assertion: a new hire's job answers exactly once
    // afterwards. The ground query answers its empty substitution
    // exactly once.
    let mut filed = microshaft();
    filed.assert(fact(
        "job",
        vec![
            person(&["Hacker", "Kay"]),
            list(&[atom("computer"), atom("programmer")]),
        ],
    ));
    let probe = relation(
        "job",
        vec![
            person(&["Hacker", "Kay"]),
            list(&[atom("computer"), atom("programmer")]),
        ],
    );
    let filed_answers = qeval(&filed, &probe).answers;
    assert_eq!(filed_answers.len(), 1);
    assert!(filed_answers[0].is_empty());

    // An unbound variable prints with the book's `?` prefix: the
    // printer is the only place the notation lives.
    assert_eq!(render(&var("amount")), "?amount");
    let frame = resolve(&filed_answers[0], "amount");
    assert_eq!(render(&frame), "?amount");

    // A rule is filed the same way and answers queries afterwards.
    // The bodyless `same` rule has no conditions: its empty body is
    // always true.
    let mut ruled = microshaft();
    ruled.add_rule(Rule {
        conclusion: fact("same", vec![var("x"), var("x")]),
        conditions: Vec::new(),
    });
    let answers = qeval(
        &ruled,
        &relation(
            "same",
            vec![person(&["Bitdiddle", "Ben"]), person(&["Bitdiddle", "Ben"])],
        ),
    )
    .answers;
    assert_eq!(answers.len(), 1);
}
