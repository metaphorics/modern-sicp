// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.22: `let` is analyzed once as
//! part of a function body and runs for every call.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_1::{Plan, PlanStmt, analyze, run_analyzed_program};

const SOURCE: &str = "
fn twice() -> i64 {
    let left = 3;
    let right = 4;
    left + right
}

fn main() {
    println!(\"{}\", twice());
    println!(\"{}\", twice());
}
";

#[test]
fn ex_4_22() {
    let checked = support::checked(SOURCE).expect("source is admitted");
    let analyzed = analyze(&checked);
    let (_, plan) = analyzed
        .sema
        .funs
        .iter()
        .zip(&analyzed.bodies)
        .find(|(function, _)| function.name == "twice")
        .expect("twice is analyzed");
    let Plan::Seq { stmts, tail } = plan else {
        panic!("twice has a sequence body");
    };
    assert_eq!(stmts.len(), 2);
    assert!(
        stmts
            .iter()
            .all(|stmt| matches!(stmt, PlanStmt::Let { .. }))
    );
    assert!(matches!(tail.as_deref(), Some(Plan::Binary(..))));

    let first = run_analyzed_program(&analyzed);
    let second = run_analyzed_program(&analyzed);
    let direct = support::direct(SOURCE).expect("source runs");
    assert_eq!(first.stdout, "7\n7\n");
    assert_eq!(second.stdout, first.stdout);
    assert_eq!(direct.stdout, first.stdout);
}
