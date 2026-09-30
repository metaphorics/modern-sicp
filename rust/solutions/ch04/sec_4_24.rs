// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.24: compare direct execution
//! with a retained analyzed plan over the same checked workload.

/// Shared typed support for this exercise.
pub mod support;

use std::time::Instant;

use ch04::sec_4_1::{analyze, run, run_analyzed_program};

const SOURCE: &str = "
fn fib(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    println!(\"{}\", fib(13));
}
";

const WARMUP: usize = 2;
const RUNS: usize = 5;

#[test]
fn ex_4_24() {
    let checked = support::checked(SOURCE).expect("source is admitted");
    let analyzed = analyze(&checked);

    for _ in 0..WARMUP {
        let direct = run(&checked);
        let planned = run_analyzed_program(&analyzed);
        assert_eq!(direct.stdout, planned.stdout);
    }

    let direct_start = Instant::now();
    let mut direct_stdout = String::new();
    for _ in 0..RUNS {
        direct_stdout = run(&checked).stdout;
    }
    let direct_ns = direct_start.elapsed().as_nanos();

    let analyzed_start = Instant::now();
    let mut analyzed_stdout = String::new();
    for _ in 0..RUNS {
        analyzed_stdout = run_analyzed_program(&analyzed).stdout;
    }
    let analyzed_ns = analyzed_start.elapsed().as_nanos();

    assert_eq!(direct_stdout, "233\n");
    assert_eq!(analyzed_stdout, direct_stdout);
    assert!(direct_ns > 0);
    assert!(analyzed_ns > 0);
}
