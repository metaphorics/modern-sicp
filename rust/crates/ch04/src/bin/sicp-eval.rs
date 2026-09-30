// SPDX-License-Identifier: GPL-3.0-only
//! The edition teaching driver for the section 4 engines and the
//! named experiment/query cases. Stdout is exactly one JSON object
//! `{"termination": ..., "stdout": ...}`; diagnostics go to stderr.

use std::fmt::Write as _;

use ch04::sec_4_1::{run_analyzed, run_source};
use ch04::sec_4_2::{self as lazy, LazyOutcome, Mode};
use ch04::sec_4_3::{self as search, SearchOutcome};
use ch04::sec_4_4::{self as query, Substitution};

fn escape(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            control if (control as u32) < 0x20 => {
                // `String`'s fmt::Write never fails, so the ignored
                // result is safe.
                let _ = write!(out, "\\u{:04x}", control as u32);
            }
            other => out.push(other),
        }
    }
    out
}

fn emit(termination: &str, transcript: &str) {
    println!(
        "{{\"termination\":\"{}\",\"stdout\":\"{}\"}}",
        termination,
        escape(transcript)
    );
}

fn outcome_lines(outcome: &sicp_runtime::host::ops::RunOutcome) -> (&'static str, String) {
    if let Some(trap) = &outcome.trap {
        eprintln!("{trap:?}");
        ("error", outcome.stdout.clone())
    } else {
        ("value", outcome.stdout.clone())
    }
}

fn lazy_lines(prefix: &str, mode: &str, outcome: &LazyOutcome) -> String {
    let value = outcome
        .value
        .map_or_else(|| "-".to_owned(), |value| value.to_string());
    format!(
        "{prefix}mode={mode} value={value} effects={} rendered={}",
        outcome.effects.join(","),
        outcome.rendered.join(",")
    )
}

fn lazy_case(case: &str) -> lazy::LazyExpr {
    match case {
        "lazy/01-non-strict-application" => lazy::lazy_non_strict(),
        "lazy/02-delay-force" => lazy::lazy_delay_force(),
        "lazy/03-church-pairs" => lazy::lazy_church_pairs(),
        "lazy/04-lazy-list" => lazy::lazy_list(),
        other => {
            eprintln!("unknown lazy case: {other}");
            std::process::exit(2);
        }
    }
}

fn search_case(case: &str) -> search::Search {
    match case {
        "amb/01-amb-basics" => search::search_basics(),
        "amb/02-prime-sum-pair" => search::search_prime_sum(),
        "amb/03-multiple-dwelling" => search::search_dwelling(),
        "amb/04-pythagorean-triples" => search::search_pythagorean(),
        other => {
            eprintln!("unknown search case: {other}");
            std::process::exit(2);
        }
    }
}

fn query_case(case: &str) -> (query::Database, query::Query) {
    match case {
        "query/01-basic-personnel" => query::query_personnel(),
        "query/02-compound-queries" => query::query_compound(),
        "query/03-rules" => query::query_rules(),
        "query/04-append-form" => query::query_append(),
        other => {
            eprintln!("unknown query case: {other}");
            std::process::exit(2);
        }
    }
}

fn search_lines(outcome: &SearchOutcome) -> String {
    let mut lines = Vec::new();
    for answer in &outcome.answers {
        let parts: Vec<String> = answer
            .iter()
            .map(|value| match value {
                search::AnswerValue::Int(value) => value.to_string(),
                search::AnswerValue::Sym(value) => value.clone(),
            })
            .collect();
        lines.push(format!("answer={}", parts.join(",")));
    }
    lines.push(format!("effects={}", outcome.effects.join(",")));
    lines.join("\n")
}

fn render_term(term: &query::Term) -> String {
    match term {
        query::Term::Variable(name) => format!("?{name}"),
        query::Term::Integer(value) => value.to_string(),
        query::Term::Text(value) => format!("\"{value}\""),
        query::Term::Atom(value) => value.clone(),
        query::Term::Pair(_, _) => {
            let mut items = Vec::new();
            let mut cursor = term;
            loop {
                match cursor {
                    query::Term::Pair(left, right) => {
                        items.push(render_term(left));
                        cursor = right;
                    }
                    query::Term::Empty => break,
                    other => {
                        items.push(format!(". {}", render_term(other)));
                        break;
                    }
                }
            }
            format!("({})", items.join(" "))
        }
        query::Term::Empty => "()".to_owned(),
    }
}

/// The variables a query mentions, in order of first appearance.
fn query_variables(query: &query::Query, names: &mut Vec<String>) {
    fn term_variables(term: &query::Term, names: &mut Vec<String>) {
        match term {
            query::Term::Variable(name) if !names.contains(name) => names.push(name.clone()),
            query::Term::Pair(left, right) => {
                term_variables(left, names);
                term_variables(right, names);
            }
            _ => {}
        }
    }
    match query {
        query::Query::Unify(left, right) => {
            term_variables(left, names);
            term_variables(right, names);
        }
        query::Query::Relation { arguments, .. } | query::Query::Value(_, arguments) => {
            for argument in arguments {
                term_variables(argument, names);
            }
        }
        query::Query::And(subs) | query::Query::Or(subs) => {
            for sub in subs {
                query_variables(sub, names);
            }
        }
        query::Query::Not(sub) | query::Query::Unique(sub) | query::Query::UniqueBy(_, sub) => {
            query_variables(sub, names);
        }
    }
}

/// A term with every bound variable replaced by its value, as the
/// query system instantiates a query with an answer frame. A variable
/// still unbound is renamed canonically (`?base#k`, numbered by first
/// appearance in the answer), since the numbers rule application
/// assigns are bookkeeping, not part of the answer.
fn instantiate(term: &query::Term, frame: &Substitution, unbound: &mut Vec<String>) -> query::Term {
    match term {
        query::Term::Variable(name) => match frame.get(name) {
            Some(bound) => instantiate(bound, frame, unbound),
            None if !name.contains('#') => term.clone(),
            None => {
                if !unbound.contains(name) {
                    unbound.push(name.clone());
                }
                let position = unbound
                    .iter()
                    .position(|seen| seen == name)
                    .map_or(0, |at| at + 1);
                let base = name.split('#').next().unwrap_or(name);
                query::Term::Variable(format!("{base}#{position}"))
            }
        },
        query::Term::Pair(left, right) => query::Term::Pair(
            Box::new(instantiate(left, frame, unbound)),
            Box::new(instantiate(right, frame, unbound)),
        ),
        other => other.clone(),
    }
}

/// One line per answer: the query's variables, by name, instantiated
/// with the answer frame.
fn query_lines(program: &query::Query, answers: &[Substitution]) -> String {
    let mut names = Vec::new();
    query_variables(program, &mut names);
    names.sort();
    let mut lines = Vec::new();
    for answer in answers {
        let mut unbound = Vec::new();
        let rendered: Vec<String> = names
            .iter()
            .map(|name| {
                let value = instantiate(&query::Term::Variable(name.clone()), answer, &mut unbound);
                format!("{name}={}", render_term(&value))
            })
            .collect();
        lines.push(format!("answer={}", rendered.join(";")));
    }
    lines.join("\n")
}

/// The evaluator thread's stack. The direct and analyzed evaluators
/// recurse on the host stack once per guest call and nested expression,
/// so a guest recursion as deep as the native program's own (the
/// deep-recursion lesson) needs far more than the main thread's default.
/// The reservation is virtual; only the depth actually used is touched.
const EVALUATOR_STACK_BYTES: usize = 1 << 30;

fn main() {
    let evaluator = std::thread::Builder::new()
        .name("evaluator".to_owned())
        .stack_size(EVALUATOR_STACK_BYTES)
        .spawn(run)
        .unwrap_or_else(|error| {
            eprintln!("cannot start the evaluator thread: {error}");
            std::process::exit(2);
        });
    if evaluator.join().is_err() {
        std::process::exit(101);
    }
}

fn run() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: sicp-eval <engine> <case> <source>");
        std::process::exit(2);
    }
    let (engine, case, source) = (args[1].as_str(), args[2].as_str(), args[3].as_str());
    match engine {
        "direct" | "analyzed" => {
            let text = match std::fs::read_to_string(source) {
                Ok(text) => text,
                Err(error) => {
                    eprintln!("cannot read {source}: {error}");
                    std::process::exit(2);
                }
            };
            let outcome = match engine {
                "direct" => run_source(&text),
                _ => match sicp_runtime::host::admit(&text) {
                    Ok(program) => Ok(run_analyzed(&program)),
                    Err(diag) => Err(diag),
                },
            };
            match outcome {
                Ok(outcome) => {
                    let (termination, transcript) = outcome_lines(&outcome);
                    emit(termination, &transcript);
                }
                Err(diag) => {
                    eprintln!("{diag:?}");
                    emit("rejected", "");
                }
            }
        }
        "lazy" => {
            let expr = lazy_case(case);
            let mut lines = Vec::new();
            for (name, mode) in [
                ("lazy-recompute/1", Mode::Recompute),
                ("lazy-memo/1", Mode::Memo),
            ] {
                let outcome = lazy::LazyEngine::new(mode).run(&expr);
                lines.push(lazy_lines("", name, &outcome));
            }
            emit("value", &lines.join("\n"));
        }
        "search" => {
            let program = search_case(case);
            let outcome = search::SearchEngine::new().run(&program);
            emit("value", &search_lines(&outcome));
        }
        "query" => {
            let (database, program) = query_case(case);
            let outcome = query::qeval(&database, &program);
            emit("value", &query_lines(&program, &outcome.answers));
        }
        "reference" => {
            let mut lines = Vec::new();
            if case.starts_with("lazy/") {
                let expr = lazy_case(case);
                for (name, mode) in [
                    ("lazy-recompute/1", Mode::Recompute),
                    ("lazy-memo/1", Mode::Memo),
                ] {
                    let outcome = lazy::reference_model(mode, &expr);
                    lines.push(lazy_lines("", name, &outcome));
                }
            } else if case.starts_with("amb/") {
                let program = search_case(case);
                let outcome = search::reference_model(&program);
                lines.push(search_lines(&outcome));
            } else if case.starts_with("query/") {
                let (database, program) = query_case(case);
                let answers = query::reference_answers(&database, &program);
                lines.push(query_lines(&program, &answers));
            } else {
                eprintln!("reference engine has no model for {case}");
                std::process::exit(2);
            }
            emit("value", &lines.join("\n"));
        }
        other => {
            eprintln!("unknown engine: {other}");
            std::process::exit(2);
        }
    }
}
