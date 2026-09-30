// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.8: named iteration as a
//! self-bound typed loop value.

/// Shared typed support for this exercise.
pub mod support;

fn named_fibonacci(term: i64) -> i64 {
    let mut previous = 0;
    let mut current = 1;
    for _ in 0..term {
        let next = previous + current;
        previous = current;
        current = next;
    }
    previous
}

#[test]
fn ex_4_08() {
    assert_eq!(named_fibonacci(10), 55);
}
