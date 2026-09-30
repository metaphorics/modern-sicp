// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.1

//! Section 4.1.7: separating syntactic analysis from execution.
//! `analyze` walks the checked program once and answers a plan tree of
//! execution procedures with every dispatch decision already made;
//! running the plan never re-dispatches on syntax, and the analyzed
//! evaluator's observable results match the direct evaluator exactly.

use ch04::eval_support::both_transcripts;
use ch04::sec_4_1::{admit, analyze, run, run_analyzed};

const FACTORIAL: &str = "\
fn factorial(n: i64) -> i64 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!(\"{}\", factorial(6));
}
";

const ASSIGNMENT: &str = "\
fn count_up(mut n: i64) -> i64 {
    n = n + 1;
    n
}

fn main() {
    println!(\"{}\", count_up(41));
}
";

fn main() {
    // Analysis answers execution procedures as typed plan data: every
    // dispatch decision is resolved when the plan is built.
    let checked = admit(FACTORIAL).expect("admitted");
    let _plan = analyze(&checked);

    // The analyzed run answers the factorial value.
    let outcome = run_analyzed(&checked);
    println!("{}", outcome.stdout);
    // => 720
    assert_eq!(outcome.stdout, "720\n");

    // Running the analyzed program again answers the same value, and
    // the direct evaluator agrees on every byte of the transcript.
    assert_eq!(run_analyzed(&checked).stdout, outcome.stdout);
    assert_eq!(run(&checked).stdout, outcome.stdout);
    let (direct, analyzed) = both_transcripts(FACTORIAL).expect("admitted");
    assert_eq!(direct, analyzed);

    // The analyzed evaluator covers the same subset: assignments and
    // sequences included.
    let outcome = run_analyzed(&admit(ASSIGNMENT).expect("admitted"));
    println!("{}", outcome.stdout);
    // => 42
    assert_eq!(outcome.stdout, "42\n");

    // The section's benchmark workload runs on both engines;
    // solutions/ch04/sec_4_1.rs times the two against each other.
}
