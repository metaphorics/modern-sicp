// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.68: reverse and append as
//! typed rules over pair terms.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, Substitution, qeval};
use sicp_runtime::host::query::Term;
use support::{atom, fact, list, pair, relation, rule, term_text, var};

fn reverse_rules() -> Database {
    let mut database = Database::new();
    database.add_rule(rule(
        fact("reverse", vec![Term::Empty, Term::Empty]),
        vec![],
    ));
    database.add_rule(rule(
        fact(
            "reverse",
            vec![pair(var("item"), var("rest")), var("answer")],
        ),
        vec![
            relation("reverse", vec![var("rest"), var("reversed_rest")]),
            relation(
                "append",
                vec![
                    var("reversed_rest"),
                    pair(var("item"), Term::Empty),
                    var("answer"),
                ],
            ),
        ],
    ));
    database.add_rule(rule(
        fact("append", vec![Term::Empty, var("right"), var("right")]),
        vec![],
    ));
    database.add_rule(rule(
        fact(
            "append",
            vec![
                pair(var("item"), var("rest")),
                var("right"),
                pair(var("item"), var("appended")),
            ],
        ),
        vec![relation(
            "append",
            vec![var("rest"), var("right"), var("appended")],
        )],
    ));
    database
}

/// Instantiates a term through the frame, the way the query driver
/// prints an answer: every bound variable is replaced, at any depth.
fn instantiate(term: &Term, frame: &Substitution) -> Term {
    match term {
        Term::Variable(name) => frame
            .get(name)
            .map_or_else(|| term.clone(), |bound| instantiate(bound, frame)),
        Term::Pair(left, right) => pair(instantiate(left, frame), instantiate(right, frame)),
        other => other.clone(),
    }
}

#[test]
fn ex_4_68() {
    let query = relation(
        "reverse",
        vec![list(vec![atom("a"), atom("b"), atom("c")]), var("answer")],
    );
    let outcome = qeval(&reverse_rules(), &query);
    assert_eq!(outcome.answers.len(), 1);
    let answer = instantiate(&var("answer"), &outcome.answers[0]);
    assert_eq!(term_text(&answer), "(c b a)");
}
