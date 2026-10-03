// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.3

//! Section 4.3.3: evaluating explicit search choices as the named
//! experiment `search-undo-trails/1`. A choice frame carries the
//! alternatives a trial has left; the undo trail restores every
//! trailed assignment the search unwinds through, so a failed branch
//! leaves no reversible state behind. Persistent assignments remain
//! visible across the unwind.

use ch04::sec_4_3::{AnswerTerm, AnswerValue, Search, SearchEngine};
use sicp_runtime::host::query::{Predicate, Term};

/// Renders one answer component for the transcript.
fn render(value: &AnswerValue) -> String {
    match value {
        AnswerValue::Int(value) => value.to_string(),
        AnswerValue::Sym(name) => name.clone(),
    }
}

/// This binary's transcript printer.
fn show(label: &str, parts: &[String]) {
    println!("{label}: {parts:?}");
}

/// A two-trial search whose second trial fails its guard: the first
/// answer and the state left after exhaustion. `per_trial` is the
/// assignment form each trial runs.
fn two_trials(per_trial: fn(String, i64, Box<Search>) -> Search) -> Search {
    let answer = Search::Success(vec![AnswerTerm::Var(String::from("x"))]);
    let trials: Vec<Search> = [1, 2]
        .into_iter()
        .map(|x| {
            let guarded = Search::Guard(
                Predicate::Eq(Term::Variable(String::from("x")), Term::Integer(1)),
                Box::new(answer.clone()),
            );
            Search::Set(
                String::from("x"),
                x,
                Box::new(per_trial(String::from("count"), x, Box::new(guarded))),
            )
        })
        .collect();
    Search::Set(String::from("count"), 0, Box::new(Search::Choose(trials)))
}

fn main() {
    // The trailed assignment: trial one answers, trial two fails its
    // guard and unwinds, and every assignment the search crossed is
    // restored. The final binding is therefore the baseline again.
    let mut engine = SearchEngine::new();
    let outcome = engine.run(&two_trials(Search::Set));
    let rendered: Vec<String> = outcome.answers.iter().flatten().map(render).collect();
    println!("reversible assignment answer: {}", rendered.join(" "));
    // => reversible assignment answer: 1
    assert_eq!(outcome.answers, [[AnswerValue::Int(1)]]);
    assert_eq!(engine.binding("count"), Some(0));
    show(
        "reversible assignment trail",
        &[String::from("exhausted"), String::from("0")],
    );
    // => reversible assignment trail: ["exhausted", "0"]

    // Persistent assignment does not enter the undo trail. Both trials
    // write; the failed one's write survives the unwind, so the final
    // binding contains the second trial's value.
    let mut engine = SearchEngine::new();
    let outcome = engine.run(&two_trials(Search::Persist));
    assert_eq!(outcome.answers, [[AnswerValue::Int(1)]]);
    assert_eq!(engine.binding("count"), Some(2));
    show(
        "persistent assignment trail",
        &[String::from("exhausted"), String::from("2")],
    );
    // => persistent assignment trail: ["exhausted", "2"]

    // The driver protocol: asking past the last answer reports
    // exhaustion rather than inventing one, and an empty choice fails
    // immediately.
    let one_answer = Search::Success(vec![AnswerTerm::Const(1)]);
    let past_the_end = SearchEngine::new().run_prefix(&one_answer, 2).answers.len() == 1;
    let empty_fails = SearchEngine::new()
        .run(&Search::Choose(Vec::new()))
        .answers
        .is_empty();
    let driver = vec![past_the_end, empty_fails];
    println!("driver protocol: {driver:?}");
    // => driver protocol: [true, true]
    assert_eq!(driver, vec![true, true]);
}
