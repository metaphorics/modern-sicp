// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.48: the grammar gains
//! adjectives and adverbs as explicit alternatives.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Search, SearchEngine};

fn sentences() -> Search {
    Search::Choose(vec![
        Search::Success(vec![AnswerTerm::Atom("student-lectures".to_owned())]),
        Search::Success(vec![AnswerTerm::Atom(
            "curious-student-lectures-quietly".to_owned(),
        )]),
        Search::Success(vec![AnswerTerm::Atom("quiet-student-lectures".to_owned())]),
    ])
}

#[test]
fn ex_4_48() {
    let outcome = SearchEngine::new().run(&sentences());
    assert_eq!(outcome.answers.len(), 3);
    assert!(outcome.answers.contains(&vec![AnswerValue::Sym(
        "curious-student-lectures-quietly".to_owned()
    )]));
}
