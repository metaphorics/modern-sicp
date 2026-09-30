// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.49: generation reuses the
//! grammar by choosing words instead of consuming input.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Search, SearchEngine};

fn generated() -> Search {
    Search::ChooseRange {
        var: "noun".to_owned(),
        lo: 1,
        hi: 3,
        body: Box::new(Search::ChooseRange {
            var: "verb".to_owned(),
            lo: 1,
            hi: 3,
            body: Box::new(Search::Success(vec![
                AnswerTerm::Var("noun".to_owned()),
                AnswerTerm::Var("verb".to_owned()),
            ])),
        }),
    }
}

#[test]
fn ex_4_49() {
    let outcome = SearchEngine::new().run(&generated());
    assert_eq!(outcome.answers.len(), 9);
    assert_eq!(
        outcome.answers[0],
        vec![AnswerValue::Int(1), AnswerValue::Int(1)]
    );
}
