// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.19: the internal-definition
//! scoping debate, with each position's observable result.

/// Shared typed support for this exercise.
pub mod support;

fn ben_sequential() -> i64 {
    let mut a = 1;
    let b = a + 10;
    a = 5;
    a + b
}

fn alyssa_scan_out() -> Option<i64> {
    let a = None;
    let b = a.map(|value: i64| value + 10)?;
    let a = Some(5);
    a.zip(Some(b)).map(|(a, b)| a + b)
}

#[test]
fn ex_4_19() {
    assert_eq!(ben_sequential(), 16);
    assert_eq!(alyssa_scan_out(), None);
}
