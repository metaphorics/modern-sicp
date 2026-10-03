// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.53: `Persist` records the
//! failed attempt even when `if-fail` supplies the answer.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Predicate, Search, SearchEngine};
use sicp_runtime::host::query::Term;

fn program() -> Search {
    Search::IfFail {
        primary: Box::new(Search::Persist(
            "tried".to_owned(),
            1,
            Box::new(Search::Fail),
        )),
        fallback: Box::new(Search::Guard(
            Predicate::Eq(Term::Variable("tried".to_owned()), Term::Integer(1)),
            Box::new(Search::Success(vec![AnswerTerm::Const(42)])),
        )),
    }
}

#[test]
fn ex_4_53() {
    let mut engine = SearchEngine::new();
    let outcome = engine.run(&program());
    assert_eq!(outcome.answers, vec![vec![AnswerValue::Int(42)]]);
    assert_eq!(engine.binding("tried"), Some(1));
}
