// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.4: `&&` and `||` are Rust's
//! short-circuit Boolean forms, and their operands are values rather
//! than syntax trees.

/// Shared typed support for this exercise.
pub mod support;

const SHORT_CIRCUIT: &str = "
fn main() {
    let and_value = false && (1i64 / 0i64 == 1i64);
    let or_value = true || (1i64 / 0i64 == 1i64);
    println!(\"{} {}\", and_value, or_value);
}
";

#[test]
fn ex_4_04() {
    let outcome = support::direct(SHORT_CIRCUIT).expect("short-circuit source runs");
    assert_eq!(outcome.stdout, "false true\n");
    assert!(outcome.trap.is_none());
}
