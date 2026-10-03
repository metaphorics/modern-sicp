// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.52: fallback runs only when
//! the primary search yields no answers.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Search, SearchEngine, reference_model};

fn if_fail(primary: Search, fallback: Search) -> Search {
    Search::IfFail {
        primary: Box::new(primary),
        fallback: Box::new(fallback),
    }
}

#[test]
fn ex_4_52() {
    let program = if_fail(Search::Fail, Search::Success(vec![AnswerTerm::Const(42)]));
    let expected = vec![vec![AnswerValue::Int(42)]];
    assert_eq!(SearchEngine::new().run(&program).answers, expected);
    assert_eq!(reference_model(&program).answers, expected);
}

#[test]
fn ex_4_52_primary_answers_skip_the_fallback() {
    let program = if_fail(
        Search::Choose(vec![
            Search::Success(vec![AnswerTerm::Const(8)]),
            Search::Success(vec![AnswerTerm::Const(12)]),
        ]),
        Search::Success(vec![AnswerTerm::Const(0)]),
    );
    let expected = vec![vec![AnswerValue::Int(8)], vec![AnswerValue::Int(12)]];
    assert_eq!(SearchEngine::new().run(&program).answers, expected);
    assert_eq!(reference_model(&program).answers, expected);
}
