// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.40: assignment counts and
//! interleaved restriction pruning in the multiple-dwelling search.

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

fn guard(predicate: Predicate, body: Search) -> Search {
    Search::Guard(predicate, Box::new(body))
}

fn emit(tag: &'static str, body: Search) -> Search {
    Search::Emit(tag.to_owned(), Box::new(body))
}

fn choose(name: &str, body: Search) -> Search {
    Search::ChooseRange {
        var: name.to_owned(),
        lo: 1,
        hi: 5,
        body: Box::new(body),
    }
}

fn difference(left: &str, right: &str, amount: &str) -> Predicate {
    Predicate::DiffEq(
        left.to_owned(),
        right.to_owned(),
        amount.to_owned(),
        "zero".to_owned(),
    )
}

fn not_adjacent(left: &str, right: &str) -> Predicate {
    Predicate::Or(vec![
        difference(left, right, "two"),
        difference(left, right, "three"),
        difference(left, right, "four"),
        difference(right, left, "two"),
        difference(right, left, "three"),
        difference(right, left, "four"),
    ])
}

fn answer() -> Search {
    Search::Success(vec![
        AnswerTerm::Var("baker".to_owned()),
        AnswerTerm::Var("cooper".to_owned()),
        AnswerTerm::Var("fletcher".to_owned()),
        AnswerTerm::Var("miller".to_owned()),
        AnswerTerm::Var("smith".to_owned()),
    ])
}

fn all_assignments() -> Search {
    choose(
        "baker",
        choose(
            "cooper",
            choose(
                "fletcher",
                choose("miller", choose("smith", emit("assignment", answer()))),
            ),
        ),
    )
}

fn distinct_assignments() -> Search {
    let smith = guard(
        Predicate::Ne(variable("smith"), variable("miller")),
        guard(
            Predicate::Ne(variable("smith"), variable("fletcher")),
            guard(
                Predicate::Ne(variable("smith"), variable("cooper")),
                guard(
                    Predicate::Ne(variable("smith"), variable("baker")),
                    emit("assignment", answer()),
                ),
            ),
        ),
    );
    let miller = guard(
        Predicate::Ne(variable("miller"), variable("fletcher")),
        guard(
            Predicate::Ne(variable("miller"), variable("cooper")),
            guard(
                Predicate::Ne(variable("miller"), variable("baker")),
                choose("smith", smith),
            ),
        ),
    );
    let fletcher = guard(
        Predicate::Ne(variable("fletcher"), variable("cooper")),
        guard(
            Predicate::Ne(variable("fletcher"), variable("baker")),
            choose("miller", miller),
        ),
    );
    let cooper = guard(
        Predicate::Ne(variable("cooper"), variable("baker")),
        choose("fletcher", fletcher),
    );
    choose("baker", choose("cooper", cooper))
}

fn efficient_dwelling() -> Search {
    let smith = guard(
        not_adjacent("smith", "fletcher"),
        guard(
            Predicate::Ne(variable("smith"), variable("miller")),
            guard(
                Predicate::Ne(variable("smith"), variable("fletcher")),
                guard(
                    Predicate::Ne(variable("smith"), variable("cooper")),
                    guard(
                        Predicate::Ne(variable("smith"), variable("baker")),
                        emit("assignment", answer()),
                    ),
                ),
            ),
        ),
    );
    let miller = guard(
        Predicate::Gt(variable("miller"), variable("cooper")),
        guard(
            Predicate::Ne(variable("miller"), variable("fletcher")),
            guard(
                Predicate::Ne(variable("miller"), variable("cooper")),
                guard(
                    Predicate::Ne(variable("miller"), variable("baker")),
                    choose("smith", smith),
                ),
            ),
        ),
    );
    let fletcher = guard(
        Predicate::Ne(variable("fletcher"), integer(5)),
        guard(
            Predicate::Ne(variable("fletcher"), integer(1)),
            guard(
                not_adjacent("fletcher", "cooper"),
                guard(
                    Predicate::Ne(variable("fletcher"), variable("cooper")),
                    guard(
                        Predicate::Ne(variable("fletcher"), variable("baker")),
                        choose("miller", miller),
                    ),
                ),
            ),
        ),
    );
    let cooper = guard(
        Predicate::Ne(variable("cooper"), integer(1)),
        guard(
            Predicate::Ne(variable("cooper"), variable("baker")),
            choose("fletcher", fletcher),
        ),
    );
    let baker = guard(
        Predicate::Ne(variable("baker"), integer(5)),
        choose("cooper", cooper),
    );
    Search::Set(
        "zero".to_owned(),
        0,
        Box::new(Search::Set(
            "two".to_owned(),
            2,
            Box::new(Search::Set(
                "three".to_owned(),
                3,
                Box::new(Search::Set(
                    "four".to_owned(),
                    4,
                    Box::new(choose("baker", baker)),
                )),
            )),
        )),
    )
}

#[test]
fn ex_4_40() {
    let all = SearchEngine::new().run(&all_assignments());
    let distinct = SearchEngine::new().run(&distinct_assignments());
    let efficient = SearchEngine::new().run(&efficient_dwelling());
    assert_eq!(all.effects.len(), 5usize.pow(5));
    assert_eq!(distinct.effects.len(), 5 * 4 * 3 * 2);
    assert_eq!(efficient.effects.len(), efficient.answers.len());
    assert_eq!(
        efficient.answers,
        vec![vec![
            AnswerValue::Int(3),
            AnswerValue::Int(2),
            AnswerValue::Int(4),
            AnswerValue::Int(5),
            AnswerValue::Int(1)
        ]]
    );
}
