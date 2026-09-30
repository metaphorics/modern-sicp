// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.50: `ramb` chooses among
//! alternatives in a seeded random order held by the search engine.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, Search, SearchEngine};

fn alternatives() -> Search {
    Search::Choose(vec![
        Search::Success(vec![AnswerTerm::Atom("first".to_owned())]),
        Search::Success(vec![AnswerTerm::Atom("second".to_owned())]),
        Search::Success(vec![AnswerTerm::Atom("third".to_owned())]),
        Search::Success(vec![AnswerTerm::Atom("fourth".to_owned())]),
        Search::Success(vec![AnswerTerm::Atom("fifth".to_owned())]),
    ])
}

#[test]
fn ex_4_50() {
    let first = SearchEngine::with_seed(7).run_seeded(&alternatives());
    let again = SearchEngine::with_seed(7).run_seeded(&alternatives());
    let different = SearchEngine::with_seed(8).run_seeded(&alternatives());

    assert_eq!(first.answers, again.answers);
    assert_eq!(first.answers.len(), 5);
    assert_ne!(first.answers, different.answers);
    assert!(
        first
            .answers
            .iter()
            .all(|answer| different.answers.contains(answer))
    );
}

#[test]
fn ex_4_50a() {
    let first = SearchEngine::with_seed(2026).run_seeded(&alternatives());
    let again = SearchEngine::with_seed(2026).run_seeded(&alternatives());
    let other = SearchEngine::with_seed(2028).run_seeded(&alternatives());
    assert_eq!(first.answers, again.answers);
    assert_eq!(first.answers.len(), 5);
    assert!(
        first
            .answers
            .iter()
            .all(|answer| other.answers.contains(answer))
    );
    assert_ne!(first.answers, other.answers);
}
