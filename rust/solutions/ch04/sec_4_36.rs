// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.36: integer search answers a
//! prefix of Pythagorean triples. The `search-depth-first/1` experiment
//! explores depth-first in written order, so an unbounded `k` would
//! diverge after the first answer (the exercise's lesson); `k` ranges
//! over the finite prefix 1..=40 of the unbounded generator, and
//! `run_prefix` observes the first two answers.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Predicate, Search, SearchEngine};
use sicp_runtime::host::query::Term;

fn variable(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

fn unbounded_triples() -> Search {
    let success = Search::Success(vec![
        AnswerTerm::Var("i".to_owned()),
        AnswerTerm::Var("j".to_owned()),
        AnswerTerm::Var("k".to_owned()),
    ]);
    let body = Search::Guard(
        Predicate::Le(variable("i"), variable("j")),
        Box::new(Search::ChooseRange {
            var: "k".to_owned(),
            lo: 1,
            hi: 40,
            body: Box::new(Search::Guard(
                Predicate::Pythagorean("i".to_owned(), "j".to_owned(), "k".to_owned()),
                Box::new(success),
            )),
        }),
    );
    Search::ChooseRange {
        var: "i".to_owned(),
        lo: 1,
        hi: 20,
        body: Box::new(Search::ChooseRange {
            var: "j".to_owned(),
            lo: 1,
            hi: 40,
            body: Box::new(body),
        }),
    }
}

#[test]
fn ex_4_36() {
    let outcome = SearchEngine::new().run_prefix(&unbounded_triples(), 2);
    assert_eq!(
        outcome.answers,
        vec![
            vec![
                AnswerValue::Int(3),
                AnswerValue::Int(4),
                AnswerValue::Int(5)
            ],
            vec![
                AnswerValue::Int(5),
                AnswerValue::Int(12),
                AnswerValue::Int(13)
            ],
        ]
    );
}
