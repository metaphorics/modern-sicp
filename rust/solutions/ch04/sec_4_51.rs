// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.51: `Persist` survives
//! backtracking while `Set` is rolled back.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{Search, SearchEngine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Assignment {
    Trailed,
    Permanent,
}

fn assignment(kind: Assignment, name: &'static str, value: i64, body: Search) -> Search {
    match kind {
        Assignment::Trailed => Search::Set(name.to_owned(), value, Box::new(body)),
        Assignment::Permanent => Search::Persist(name.to_owned(), value, Box::new(body)),
    }
}

fn failed_probe(kind: Assignment) -> Search {
    Search::Choose(vec![
        assignment(kind, "count", 1, Search::Fail),
        Search::Fail,
    ])
}

#[test]
fn ex_4_51() {
    let mut trailed = SearchEngine::new();
    let trailed_outcome = trailed.run(&failed_probe(Assignment::Trailed));
    assert!(trailed_outcome.answers.is_empty());
    assert_eq!(trailed.binding("count"), None);

    let mut permanent = SearchEngine::new();
    let permanent_outcome = permanent.run(&failed_probe(Assignment::Permanent));
    assert!(permanent_outcome.answers.is_empty());
    assert_eq!(permanent.binding("count"), Some(1));
}
