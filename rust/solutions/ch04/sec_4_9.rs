// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.9: typed iteration constructs
//! whose loop condition and state are explicit domain data.

/// Shared typed support for this exercise.
pub mod support;

enum Loop {
    While(fn(i64) -> bool, fn(i64) -> i64),
    Until(fn(i64) -> bool, fn(i64) -> i64),
}

fn run(loop_kind: &Loop, mut state: i64) -> i64 {
    loop {
        let keep_going = match loop_kind {
            Loop::While(condition, _) => condition(state),
            Loop::Until(condition, _) => !condition(state),
        };
        if !keep_going {
            return state;
        }
        state = match loop_kind {
            Loop::While(_, step) | Loop::Until(_, step) => step(state),
        };
    }
}

#[test]
fn ex_4_09() {
    let while_loop = Loop::While(|value| value < 5, |value| value + 1);
    let until_loop = Loop::Until(|value| value > 12, |value| value + 2);
    assert_eq!(run(&while_loop, 0), 5);
    assert_eq!(run(&until_loop, 3), 13);
}
