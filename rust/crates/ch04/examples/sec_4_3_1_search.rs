// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.3

//! Section 4.3.1: choice and search, as the named experiment
//! `search-depth-first/1`. A choice point lists alternatives in written
//! order, and the search answers each success depth-first; an unbounded
//! generator is observed through a finite answer prefix.

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Search, SearchEngine, reference_model};
use sicp_runtime::host::query::{Predicate, Term};

fn var(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

fn answer_var(name: &str) -> AnswerTerm {
    AnswerTerm::Var(name.to_owned())
}

fn int(value: &AnswerValue) -> Option<i64> {
    match value {
        AnswerValue::Int(number) => Some(*number),
        AnswerValue::Sym(_) => None,
    }
}

fn choose(name: &str, values: &[i64], body: &Search) -> Search {
    Search::Choose(
        values
            .iter()
            .map(|value| Search::Set(name.to_owned(), *value, Box::new(body.clone())))
            .collect(),
    )
}

fn run(program: &Search) -> ch04::sec_4_3::SearchOutcome {
    let outcome = SearchEngine::new().run(program);
    assert_eq!(outcome, reference_model(program));
    outcome
}

fn prime(number: i64) -> bool {
    if number < 2 {
        return false;
    }
    let mut divisor = 2;
    while divisor * divisor <= number {
        if number % divisor == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}

fn main() {
    // Two nested choices enumerate six ordered pairs; every alternative
    // resumes the same continuation with its own bindings.
    let pairs = choose(
        "x",
        &[1, 2, 3],
        &choose(
            "y",
            &[1, 2],
            &Search::Success(vec![answer_var("x"), answer_var("y")]),
        ),
    );
    let outcome = run(&pairs);
    assert_eq!(
        outcome.answers,
        [
            [AnswerValue::Int(1), AnswerValue::Int(1)],
            [AnswerValue::Int(1), AnswerValue::Int(2)],
            [AnswerValue::Int(2), AnswerValue::Int(1)],
            [AnswerValue::Int(2), AnswerValue::Int(2)],
            [AnswerValue::Int(3), AnswerValue::Int(1)],
            [AnswerValue::Int(3), AnswerValue::Int(2)],
        ]
    );

    // A guarded choice filters candidate number pairs by their sum.
    // This keeps the worked outputs 3+20, 3+110, and 8+35, in the
    // order the two choice lists generate them.
    let allowed_prime_sums = Predicate::Or(
        [23, 43, 113]
            .into_iter()
            .map(|sum| Predicate::SumEq(vec![var("a"), var("b")], sum))
            .collect(),
    );
    let prime_pairs = choose(
        "a",
        &[3, 8],
        &choose(
            "b",
            &[20, 110, 35],
            &Search::Guard(
                allowed_prime_sums,
                Box::new(Search::Success(vec![answer_var("a"), answer_var("b")])),
            ),
        ),
    );
    let outcome = run(&prime_pairs);
    assert_eq!(
        outcome.answers,
        [
            [AnswerValue::Int(3), AnswerValue::Int(20)],
            [AnswerValue::Int(3), AnswerValue::Int(110)],
            [AnswerValue::Int(8), AnswerValue::Int(35)],
        ]
    );
    assert!(outcome.answers.iter().all(|pair| {
        int(&pair[0])
            .zip(int(&pair[1]))
            .is_some_and(|(a, b)| prime(a + b))
    }));

    // An unbounded generator has infinitely many alternatives. With
    // `a` fixed at 30, the guard accepts prime sums; the first two
    // values of `b` are 11 (sum 41) and 13 (sum 43).
    let prime_sum = Predicate::Or(
        [41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
            .into_iter()
            .map(|sum| Predicate::SumEq(vec![var("a"), var("b")], sum))
            .collect(),
    );
    let unbounded = Search::Set(
        String::from("a"),
        30,
        Box::new(Search::ChooseFrom {
            var: String::from("b"),
            start: 11,
            body: Box::new(Search::Guard(
                prime_sum,
                Box::new(Search::Success(vec![answer_var("a"), answer_var("b")])),
            )),
        }),
    );
    let outcome = SearchEngine::new().run_prefix(&unbounded, 2);
    assert_eq!(
        outcome.answers,
        [
            [AnswerValue::Int(30), AnswerValue::Int(11)],
            [AnswerValue::Int(30), AnswerValue::Int(13)],
        ]
    );
}
