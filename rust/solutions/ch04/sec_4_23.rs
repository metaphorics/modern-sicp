// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.23: compare the two sequence
//! analyzers over the same analyzed body and count the analysis and
//! execution work each shape performs.

/// Shared typed support for this exercise.
pub mod support;

use std::cell::Cell;

use ch04::sec_4_1::{Plan, PlanStmt, analyze, run_analyzed_program};

const SOURCE: &str = "
fn one() -> i64 {
    3 + 4
}

fn two() -> i64 {
    let ignored: i64 = 1;
    3 + 4
}

fn main() {
    println!(\"{}\", one());
    println!(\"{}\", one());
    println!(\"{}\", two());
    println!(\"{}\", two());
}
";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Work {
    analysis_units: usize,
}

#[derive(Debug)]
enum SequencePlan<'a> {
    Combined,
    Steps(Vec<&'a Plan>),
}

fn statement_plan(statement: &PlanStmt) -> &Plan {
    match statement {
        PlanStmt::Let { value, .. } | PlanStmt::Expr(value) => value,
    }
}

fn body_steps(body: &Plan) -> Vec<&Plan> {
    let Plan::Seq { stmts, tail } = body else {
        panic!("sequence analyzer expects a sequence body");
    };
    let mut steps: Vec<&Plan> = stmts.iter().map(statement_plan).collect();
    if let Some(tail) = tail {
        steps.push(tail.as_ref());
    }
    steps
}

fn analyze_text(calls: &Cell<usize>) -> (SequencePlan<'static>, Work) {
    calls.set(calls.get() + 1);
    (SequencePlan::Combined, Work { analysis_units: 1 })
}

fn analyze_alyssa<'a>(body: &'a Plan, calls: &Cell<usize>) -> (SequencePlan<'a>, Work) {
    let steps = body_steps(body);
    calls.set(calls.get() + steps.len());
    let count = steps.len();
    (
        SequencePlan::Steps(steps),
        Work {
            analysis_units: count,
        },
    )
}

fn execute(plan: &SequencePlan<'_>, runs: usize) -> usize {
    match plan {
        SequencePlan::Combined => 0,
        SequencePlan::Steps(steps) => steps.len() * runs,
    }
}

fn analyzed_body<'a>(analyzed: &'a ch04::sec_4_1::AnalyzedProgram, name: &str) -> &'a Plan {
    analyzed
        .sema
        .funs
        .iter()
        .zip(&analyzed.bodies)
        .find(|(function, _)| function.name == name)
        .map(|(_, body)| body)
        .expect("function is analyzed")
}

#[test]
fn ex_4_23() {
    let checked = support::checked(SOURCE).expect("source is admitted");
    let analyzed = analyze(&checked);
    let one = analyzed_body(&analyzed, "one");
    let two = analyzed_body(&analyzed, "two");

    let text_calls = Cell::new(0);
    let alyssa_calls = Cell::new(0);
    let (one_text, one_text_work) = analyze_text(&text_calls);
    let (one_alyssa, one_alyssa_work) = analyze_alyssa(one, &alyssa_calls);
    assert_eq!(one_text_work.analysis_units, 1);
    assert_eq!(one_alyssa_work.analysis_units, 1);
    assert_eq!(execute(&one_text, 2), 0);
    assert_eq!(execute(&one_alyssa, 2), 2);

    let (two_text, two_text_work) = analyze_text(&text_calls);
    let (two_alyssa, two_alyssa_work) = analyze_alyssa(two, &alyssa_calls);
    assert_eq!(two_text_work.analysis_units, 1);
    assert_eq!(two_alyssa_work.analysis_units, 2);
    assert_eq!(execute(&two_text, 2), 0);
    assert_eq!(execute(&two_alyssa, 2), 4);
    assert_eq!(text_calls.get(), 2);
    assert_eq!(alyssa_calls.get(), 3);
}

#[test]
fn ex_4_23a() {
    let checked = support::checked(SOURCE).expect("source is admitted");
    let analyzed = analyze(&checked);
    let one = analyzed_body(&analyzed, "one");
    let calls = Cell::new(0);
    let (plan, _) = analyze_text(&calls);
    assert_eq!(calls.get(), 1);
    assert_eq!(execute(&plan, 2), 0);

    let alyssa_calls = Cell::new(0);
    let (alyssa_plan, alyssa_work) = analyze_alyssa(one, &alyssa_calls);
    assert_eq!(alyssa_calls.get(), 1);
    assert_eq!(alyssa_work.analysis_units, 1);
    assert_eq!(execute(&alyssa_plan, 2), 2);

    let outcome = run_analyzed_program(&analyzed);
    assert_eq!(outcome.stdout, "7\n7\n7\n7\n");
}
