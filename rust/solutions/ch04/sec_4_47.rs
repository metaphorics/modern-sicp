// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.47: Louis's recursive
//! verb-phrase parser changes the number of attempts.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, Search, SearchEngine};

fn attempts(program: &Search) -> usize {
    SearchEngine::new().run(program).effects.len()
}

fn original() -> Search {
    Search::Emit(
        "verb".to_owned(),
        Box::new(Search::Choose(vec![
            Search::Success(vec![AnswerTerm::Atom("verb".to_owned())]),
            Search::Emit(
                "extend".to_owned(),
                Box::new(Search::Success(vec![AnswerTerm::Atom(
                    "verb-phrase".to_owned(),
                )])),
            ),
        ])),
    )
}

fn louis() -> Search {
    Search::Emit(
        "verb".to_owned(),
        Box::new(Search::Choose(vec![
            Search::Emit(
                "extend".to_owned(),
                Box::new(Search::Success(vec![AnswerTerm::Atom(
                    "verb-phrase".to_owned(),
                )])),
            ),
            Search::Success(vec![AnswerTerm::Atom("verb".to_owned())]),
        ])),
    )
}

#[test]
fn ex_4_47() {
    assert!(attempts(&original()) <= attempts(&louis()));
}
