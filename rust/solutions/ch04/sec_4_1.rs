// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.1: Rust fixes operand order,
//! so the host-language exercise pins the evaluator's observable order
//! instead of asking the reader to choose one.

/// Shared typed support for this exercise.
pub mod support;

const ORDERED: &str = "
fn emit(tag: i64, value: i64) -> i64 {
    println!(\"{}\", tag);
    value
}

fn add(left: i64, right: i64) -> i64 {
    left + right
}

fn main() {
    let result = add(emit(1, 1), emit(2, 2));
    println!(\"{}\", result);
}
";

const EXPLICIT: &str = "
fn emit(tag: i64, value: i64) -> i64 {
    println!(\"{}\", tag);
    value
}

fn main() {
    let left = emit(1, 1);
    let right = emit(2, 2);
    println!(\"{}\", left + right);
}
";

#[test]
fn ex_4_01() {
    let ordered = support::both(ORDERED).expect("ordered source runs");
    let explicit = support::both(EXPLICIT).expect("explicit source runs");
    assert_eq!(ordered.0, "1\n2\n3\n");
    assert_eq!(ordered.1, "1\n2\n3\n");
    assert_eq!(explicit.0, ordered.0);
}
