// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.38: the multiple-dwelling
//! search without the Smith-Fletcher adjacency clause.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Predicate, Search, SearchEngine};
use sicp_runtime::host::query::Term;

fn variable(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

fn integer(value: i64) -> Term {
    Term::Integer(value)
}

fn not_adjacent(left: &str, right: &str) -> Predicate {
    let difference = |amount: &str| {
        Predicate::DiffEq(
            left.to_owned(),
            right.to_owned(),
            amount.to_owned(),
            "zero".to_owned(),
        )
    };
    Predicate::Or(vec![
        difference("two"),
        difference("three"),
        difference("four"),
        Predicate::DiffEq(
            right.to_owned(),
            left.to_owned(),
            "two".to_owned(),
            "zero".to_owned(),
        ),
        Predicate::DiffEq(
            right.to_owned(),
            left.to_owned(),
            "three".to_owned(),
            "zero".to_owned(),
        ),
        Predicate::DiffEq(
            right.to_owned(),
            left.to_owned(),
            "four".to_owned(),
            "zero".to_owned(),
        ),
    ])
}

fn guard(predicate: Predicate, body: Search) -> Search {
    Search::Guard(predicate, Box::new(body))
}

fn without_smith_fletcher() -> Search {
    let answer = Search::Success(vec![
        AnswerTerm::Var("baker".to_owned()),
        AnswerTerm::Var("cooper".to_owned()),
        AnswerTerm::Var("fletcher".to_owned()),
        AnswerTerm::Var("miller".to_owned()),
        AnswerTerm::Var("smith".to_owned()),
    ]);
    let constraints = guard(
        Predicate::Ne(variable("baker"), integer(5)),
        guard(
            Predicate::Ne(variable("cooper"), integer(1)),
            guard(
                Predicate::Ne(variable("fletcher"), integer(1)),
                guard(
                    Predicate::Ne(variable("fletcher"), integer(5)),
                    guard(
                        Predicate::Gt(variable("miller"), variable("cooper")),
                        guard(not_adjacent("fletcher", "cooper"), answer),
                    ),
                ),
            ),
        ),
    );
    let distinct = [
        ("baker", "cooper"),
        ("baker", "fletcher"),
        ("baker", "miller"),
        ("baker", "smith"),
        ("cooper", "fletcher"),
        ("cooper", "miller"),
        ("cooper", "smith"),
        ("fletcher", "miller"),
        ("fletcher", "smith"),
        ("miller", "smith"),
    ];
    let mut body = constraints;
    for (left, right) in distinct.into_iter().rev() {
        body = guard(Predicate::Ne(variable(left), variable(right)), body);
    }
    body = Search::Set("zero".to_owned(), 0, Box::new(body));
    body = Search::Set("two".to_owned(), 2, Box::new(body));
    body = Search::Set("three".to_owned(), 3, Box::new(body));
    body = Search::Set("four".to_owned(), 4, Box::new(body));
    Search::ChooseRange {
        var: "baker".to_owned(),
        lo: 1,
        hi: 5,
        body: Box::new(Search::ChooseRange {
            var: "cooper".to_owned(),
            lo: 1,
            hi: 5,
            body: Box::new(Search::ChooseRange {
                var: "fletcher".to_owned(),
                lo: 1,
                hi: 5,
                body: Box::new(Search::ChooseRange {
                    var: "miller".to_owned(),
                    lo: 1,
                    hi: 5,
                    body: Box::new(Search::ChooseRange {
                        var: "smith".to_owned(),
                        lo: 1,
                        hi: 5,
                        body: Box::new(body),
                    }),
                }),
            }),
        }),
    }
}

#[test]
fn ex_4_38() {
    let outcome = SearchEngine::new().run(&without_smith_fletcher());
    assert_eq!(outcome.answers.len(), 5);
    assert!(outcome.answers.contains(&vec![
        AnswerValue::Int(3),
        AnswerValue::Int(2),
        AnswerValue::Int(4),
        AnswerValue::Int(5),
        AnswerValue::Int(1)
    ]));
}
