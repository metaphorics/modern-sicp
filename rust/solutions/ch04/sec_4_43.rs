// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.43: the yacht puzzle as typed
//! search over father, daughter, and yacht-name assignments.

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

fn distinct(prefix: &str, body: Search) -> Search {
    let mut result = body;
    for left in 1..=5 {
        for right in (left + 1)..=5 {
            result = guard(
                Predicate::Ne(
                    variable(&format!("{prefix}{left}")),
                    variable(&format!("{prefix}{right}")),
                ),
                result,
            );
        }
    }
    result
}

fn fixed_facts(body: Search, told: Told) -> Search {
    let mut result = body;
    result = guard(eq("d4", 5), result);
    result = guard(eq("y1", 3), result);
    result = guard(eq("y3", 4), result);
    result = guard(eq("y4", 2), result);
    result = guard(eq("y2", 5), result);
    if let Told::Yes = told {
        result = guard(eq("d1", 1), result);
    }
    for father in 1..=5 {
        result = guard(
            Predicate::Ne(
                variable(&format!("d{father}")),
                variable(&format!("y{father}")),
            ),
            result,
        );
    }
    result = distinct("d", result);
    distinct("y", result)
}

fn gabrielle_father(body: &Search) -> Search {
    let mut alternatives = Vec::new();
    for father in 1..=5 {
        alternatives.push(guard(
            eq(&format!("d{father}"), 2),
            guard(
                Predicate::Eq(variable(&format!("y{father}")), variable("d5")),
                body.clone(),
            ),
        ));
    }
    Search::Choose(alternatives)
}

fn lorna_father() -> Search {
    let names = ["moore", "downing", "hall", "barnacle", "parker"];
    let mut alternatives = Vec::new();
    for (index, name) in names.iter().enumerate() {
        let father = i64::try_from(index + 1).expect("five fathers");
        alternatives.push(guard(
            eq(&format!("d{}", index + 1), 3),
            Search::Set(
                "lorna_father".to_owned(),
                father,
                Box::new(Search::Success(vec![AnswerTerm::Atom((*name).to_owned())])),
            ),
        ));
    }
    Search::Choose(alternatives)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Told {
    Yes,
    No,
}

fn build(told: Told) -> Search {
    let body = lorna_father();
    let body = gabrielle_father(&body);
    let body = fixed_facts(body, told);
    let mut result = body;
    for prefix in ["d", "y"] {
        for index in (1..=5).rev() {
            result = Search::ChooseRange {
                var: format!("{prefix}{index}"),
                lo: 1,
                hi: 5,
                body: Box::new(result),
            };
        }
    }
    result
}

fn told_program() -> Search {
    build(Told::Yes)
}

fn untold_program() -> Search {
    build(Told::No)
}

#[test]
fn ex_4_43() {
    let told = SearchEngine::new().run(&told_program());
    assert_eq!(
        told.answers,
        vec![vec![AnswerValue::Sym("downing".to_owned())]]
    );
    let untold = SearchEngine::new().run(&untold_program());
    assert_eq!(untold.answers.len(), 2);
    assert!(
        untold
            .answers
            .contains(&vec![AnswerValue::Sym("downing".to_owned())])
    );
    assert!(
        untold
            .answers
            .contains(&vec![AnswerValue::Sym("parker".to_owned())])
    );
}
