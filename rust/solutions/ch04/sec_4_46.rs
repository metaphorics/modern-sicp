// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.46: left-to-right operand
//! order is observable in the search effect log.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, Search, SearchEngine};

#[test]
fn ex_4_46() {
    let program = Search::Emit(
        "operator".to_owned(),
        Box::new(Search::Emit(
            "left".to_owned(),
            Box::new(Search::Emit(
                "right".to_owned(),
                Box::new(Search::Success(vec![AnswerTerm::Const(42)])),
            )),
        )),
    );
    let outcome = SearchEngine::new().run(&program);
    assert_eq!(outcome.effects, vec!["operator", "left", "right"]);
}
