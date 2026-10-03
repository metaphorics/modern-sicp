// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.37: Ben's generator chooses only
//! `i` and `j`, computes the hypotenuse, and so explores fewer candidates
//! than the 4.35 program that also searches for `k`.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_3::{AnswerTerm, Predicate, Search, SearchEngine};
use sicp_runtime::host::query::Term;

fn variable(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

fn emit(tag: &'static str, body: Search) -> Search {
    Search::Emit(tag.to_owned(), Box::new(body))
}

fn success() -> Search {
    Search::Success(vec![
        AnswerTerm::Var("i".to_owned()),
        AnswerTerm::Var("j".to_owned()),
        AnswerTerm::Var("k".to_owned()),
    ])
}

const LOW: i64 = 1;
const PAIR_HIGH: i64 = 10;
const HYPOTENUSE_HIGH: i64 = 15;

/// The 4.35 program: three nested choices, the `i <= j <= k` ordering
/// guards where the book places them, then the Pythagorean requirement.
fn book_order() -> Search {
    let body = Search::Guard(
        Predicate::Le(variable("i"), variable("j")),
        Box::new(Search::Guard(
            Predicate::Le(variable("j"), variable("k")),
            Box::new(emit(
                "candidate",
                Search::Guard(
                    Predicate::Pythagorean("i".to_owned(), "j".to_owned(), "k".to_owned()),
                    Box::new(success()),
                ),
            )),
        )),
    );
    Search::ChooseRange {
        var: "i".to_owned(),
        lo: LOW,
        hi: PAIR_HIGH,
        body: Box::new(Search::ChooseRange {
            var: "j".to_owned(),
            lo: LOW,
            hi: PAIR_HIGH,
            body: Box::new(Search::ChooseRange {
                var: "k".to_owned(),
                lo: LOW,
                hi: HYPOTENUSE_HIGH,
                body: Box::new(body),
            }),
        }),
    }
}

/// The integer square root of `square`, when it is exact.
fn exact_root(square: i64) -> Option<i64> {
    let mut root = 0;
    while root * root < square {
        root += 1;
    }
    (root * root == square).then_some(root)
}

/// Ben's program: only `i <= j` is chosen; `k` is computed from
/// `i*i + j*j` and kept when exact and within the bound.
fn ben() -> Search {
    let mut pairs = Vec::new();
    for i in LOW..=PAIR_HIGH {
        for j in i..=PAIR_HIGH {
            let outcome = match exact_root(i * i + j * j).filter(|k| *k <= HYPOTENUSE_HIGH) {
                Some(k) => Search::Set("k".to_owned(), k, Box::new(success())),
                None => Search::Fail,
            };
            pairs.push(Search::Set(
                "i".to_owned(),
                i,
                Box::new(Search::Set(
                    "j".to_owned(),
                    j,
                    Box::new(emit("candidate", outcome)),
                )),
            ));
        }
    }
    Search::Choose(pairs)
}

#[test]
fn ex_4_37() {
    let book = SearchEngine::new().run(&book_order());
    let ben = SearchEngine::new().run(&ben());
    assert_eq!(book.answers, ben.answers);
    assert!(!ben.answers.is_empty());
    assert!(ben.effects.len() < book.effects.len());
}
