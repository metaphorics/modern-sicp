// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.78: query answers feed an
//! explicit search program.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, Search, SearchEngine};
use ch04::sec_4_4::{Database, qeval};
use support::{answer_text, atom, fact, relation, var};

fn items() -> Database {
    let mut database = Database::new();
    database.assert(fact("item", vec![atom("a")]));
    database.assert(fact("item", vec![atom("b")]));
    database
}

#[test]
fn ex_4_78() {
    let query = qeval(&items(), &relation("item", vec![var("answer")]));
    let alternatives: Vec<Search> = query
        .answers
        .iter()
        .map(|answer| Search::Success(vec![AnswerTerm::Atom(answer_text(answer, "answer"))]))
        .collect();
    let outcome = SearchEngine::new().run(&Search::Choose(alternatives));
    assert_eq!(outcome.answers.len(), 2);
}
