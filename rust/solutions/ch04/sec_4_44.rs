// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.44: eight queens as explicit
//! row choices with column and diagonal guards.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Predicate, Search, SearchEngine};
use sicp_runtime::host::query::Term;

fn variable(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

fn guard(predicate: Predicate, body: Search) -> Search {
    Search::Guard(predicate, Box::new(body))
}

fn difference(left: &str, right: &str, amount: &str) -> Predicate {
    Predicate::DiffEq(
        left.to_owned(),
        right.to_owned(),
        amount.to_owned(),
        "zero".to_owned(),
    )
}

fn not_diagonal(left: &str, right: &str, distance: i64, board: i64) -> Predicate {
    let mut allowed = Vec::new();
    for amount in 1..board {
        if amount != distance {
            let name = format!("d{amount}");
            allowed.push(difference(left, right, &name));
            allowed.push(difference(right, left, &name));
        }
    }
    Predicate::Or(allowed)
}

fn queens(board: i64) -> Search {
    let mut body = Search::Success(
        (1..=board)
            .map(|row| AnswerTerm::Var(format!("c{row}")))
            .collect(),
    );
    for left in 1..=board {
        for right in (left + 1)..=board {
            let left_name = format!("c{left}");
            let right_name = format!("c{right}");
            body = guard(
                Predicate::Ne(variable(&left_name), variable(&right_name)),
                guard(
                    not_diagonal(&left_name, &right_name, right - left, board),
                    body,
                ),
            );
        }
    }
    for amount in 0..board {
        body = Search::Set(format!("d{amount}"), amount, Box::new(body));
    }
    for row in 1..=board {
        body = Search::ChooseRange {
            var: format!("c{row}"),
            lo: 1,
            hi: board,
            body: Box::new(body),
        };
    }
    body = Search::Set("zero".to_owned(), 0, Box::new(body));
    body
}

#[test]
fn ex_4_44() {
    let outcome = SearchEngine::new().run_prefix(&queens(4), 1);
    assert_eq!(
        outcome.answers,
        vec![vec![
            AnswerValue::Int(3),
            AnswerValue::Int(1),
            AnswerValue::Int(4),
            AnswerValue::Int(2)
        ]]
    );
}
