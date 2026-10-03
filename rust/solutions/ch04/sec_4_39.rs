// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.39: restriction order changes
//! search work, not the final answers.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, Predicate, Search, SearchEngine};
use sicp_runtime::host::query::Term;

fn variable(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

fn guard(predicate: Predicate, body: Search) -> Search {
    Search::Guard(predicate, Box::new(body))
}

fn emit(tag: &'static str, body: Search) -> Search {
    Search::Emit(tag.to_owned(), Box::new(body))
}

fn success() -> Search {
    Search::Success(vec![
        AnswerTerm::Var("baker".to_owned()),
        AnswerTerm::Var("cooper".to_owned()),
    ])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Order {
    RestrictionsFirst,
    ChoicesFirst,
}

fn program(order: Order) -> Search {
    let restrictions_first = emit(
        "ne",
        guard(
            Predicate::Ne(variable("baker"), variable("cooper")),
            emit(
                "gt",
                guard(
                    Predicate::Gt(variable("cooper"), variable("baker")),
                    success(),
                ),
            ),
        ),
    );
    let choices_first = emit(
        "gt",
        guard(
            Predicate::Gt(variable("cooper"), variable("baker")),
            emit(
                "ne",
                guard(
                    Predicate::Ne(variable("baker"), variable("cooper")),
                    success(),
                ),
            ),
        ),
    );
    let body = match order {
        Order::RestrictionsFirst => emit("candidate", restrictions_first),
        Order::ChoicesFirst => emit("candidate", choices_first),
    };
    Search::ChooseRange {
        var: "baker".to_owned(),
        lo: 1,
        hi: 5,
        body: Box::new(Search::ChooseRange {
            var: "cooper".to_owned(),
            lo: 1,
            hi: 5,
            body: Box::new(body),
        }),
    }
}

#[test]
fn ex_4_39() {
    let first = SearchEngine::new().run(&program(Order::RestrictionsFirst));
    let second = SearchEngine::new().run(&program(Order::ChoicesFirst));
    assert_eq!(first.answers, second.answers);
    assert_ne!(first.effects, second.effects);
}
