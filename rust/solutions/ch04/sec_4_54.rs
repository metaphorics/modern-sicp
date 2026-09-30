// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.54: `require` is a guard that
//! fails the current branch rather than a host-language conditional.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Predicate, Search, SearchEngine};
use sicp_runtime::host::query::Term;

fn require(condition: Predicate, body: Search) -> Search {
    Search::Guard(condition, Box::new(body))
}

#[test]
fn ex_4_54() {
    let program = Search::Choose(vec![
        require(
            Predicate::Eq(Term::Variable("x".to_owned()), Term::Integer(1)),
            Search::Success(vec![AnswerTerm::Const(1)]),
        ),
        require(
            Predicate::Eq(Term::Variable("x".to_owned()), Term::Integer(2)),
            Search::Success(vec![AnswerTerm::Const(2)]),
        ),
    ]);
    let outcome = SearchEngine::new().run(&Search::Set("x".to_owned(), 2, Box::new(program)));
    assert_eq!(outcome.answers, vec![vec![AnswerValue::Int(2)]]);
}
