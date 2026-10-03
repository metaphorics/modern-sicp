// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.42: the liars puzzle as a
//! typed search over ranks with one true clause per girl.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Predicate, Search, SearchEngine};
use sicp_runtime::host::query::Term;

fn variable(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

fn integer(value: i64) -> Term {
    Term::Integer(value)
}

fn eq(left: &str, value: i64) -> Predicate {
    Predicate::Eq(variable(left), integer(value))
}

fn guard(predicate: Predicate, body: Search) -> Search {
    Search::Guard(predicate, Box::new(body))
}

fn negate(predicate: &Predicate) -> Predicate {
    match predicate {
        Predicate::Eq(left, right) => Predicate::Ne(left.clone(), right.clone()),
        Predicate::Ne(left, right) => Predicate::Eq(left.clone(), right.clone()),
        other => other.clone(),
    }
}

fn claim(binding: &str, predicate: &Predicate, body: Search) -> Search {
    Search::Choose(vec![
        guard(
            predicate.clone(),
            Search::Set(format!("{binding}_true"), 1, Box::new(body.clone())),
        ),
        guard(
            negate(predicate),
            Search::Set(format!("{binding}_true"), 0, Box::new(body)),
        ),
    ])
}

fn success() -> Search {
    Search::Success(vec![
        AnswerTerm::Var("betty".to_owned()),
        AnswerTerm::Var("ethel".to_owned()),
        AnswerTerm::Var("joan".to_owned()),
        AnswerTerm::Var("kitty".to_owned()),
        AnswerTerm::Var("mary".to_owned()),
    ])
}

fn liars() -> Search {
    let mut body = success();
    for binding in ["betty", "ethel", "joan", "kitty", "mary"] {
        body = guard(
            Predicate::SumEq(
                vec![
                    variable(&format!("{binding}_first_true")),
                    variable(&format!("{binding}_second_true")),
                ],
                1,
            ),
            body,
        );
    }
    for (binding, first, second) in [
        ("betty", eq("kitty", 2), eq("betty", 3)),
        ("ethel", eq("ethel", 1), eq("joan", 2)),
        ("joan", eq("joan", 3), eq("ethel", 5)),
        ("kitty", eq("kitty", 2), eq("mary", 4)),
        ("mary", eq("mary", 4), eq("betty", 1)),
    ]
    .into_iter()
    .rev()
    {
        body = claim(
            &format!("{binding}_first"),
            &first,
            claim(&format!("{binding}_second"), &second, body),
        );
    }
    for left in ["betty", "ethel", "joan", "kitty", "mary"] {
        for right in ["betty", "ethel", "joan", "kitty", "mary"] {
            if left < right {
                body = guard(Predicate::Ne(variable(left), variable(right)), body);
            }
        }
    }
    for name in ["betty", "ethel", "joan", "kitty", "mary"] {
        body = Search::ChooseRange {
            var: name.to_owned(),
            lo: 1,
            hi: 5,
            body: Box::new(body),
        };
    }
    body
}

#[test]
fn ex_4_42() {
    let outcome = SearchEngine::new().run(&liars());
    assert_eq!(
        outcome.answers,
        vec![vec![
            AnswerValue::Int(3),
            AnswerValue::Int(5),
            AnswerValue::Int(2),
            AnswerValue::Int(1),
            AnswerValue::Int(4)
        ]]
    );
}
