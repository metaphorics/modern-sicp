// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.35: `an-integer-between` over
//! typed range choices and a Pythagorean triple search.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Predicate, Search, SearchEngine, reference_model};
use sicp_runtime::host::query::Term;

fn variable(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

fn triples() -> Search {
    let success = Search::Success(vec![
        AnswerTerm::Var("i".to_owned()),
        AnswerTerm::Var("j".to_owned()),
        AnswerTerm::Var("k".to_owned()),
    ]);
    let ordered = Search::Guard(
        Predicate::Le(variable("i"), variable("j")),
        Box::new(Search::Guard(
            Predicate::Le(variable("j"), variable("k")),
            Box::new(Search::Guard(
                Predicate::Pythagorean("i".to_owned(), "j".to_owned(), "k".to_owned()),
                Box::new(success),
            )),
        )),
    );
    Search::ChooseRange {
        var: "i".to_owned(),
        lo: 1,
        hi: 20,
        body: Box::new(Search::ChooseRange {
            var: "j".to_owned(),
            lo: 1,
            hi: 20,
            body: Box::new(Search::ChooseRange {
                var: "k".to_owned(),
                lo: 1,
                hi: 30,
                body: Box::new(ordered),
            }),
        }),
    }
}

#[test]
fn ex_4_35() {
    let program = triples();
    let outcome = SearchEngine::new().run(&program);
    let reference = reference_model(&program);
    assert_eq!(outcome.answers, reference.answers);
    assert!(outcome.answers.contains(&vec![
        AnswerValue::Int(3),
        AnswerValue::Int(4),
        AnswerValue::Int(5)
    ]));
}
