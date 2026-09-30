// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.21: recursion without a
//! top-level definition, by passing the procedure to itself.

/// Shared typed support for this exercise.
pub mod support;

/// A factorial procedure passed as ordinary data: the wrapper keeps the
/// self-application well-typed without naming the top-level function.
#[derive(Debug, Clone, Copy)]
struct Fact(fn(Fact, i64) -> i64);

/// A Fibonacci procedure passed as ordinary data, threading its
/// accumulator state through the same self-passing discipline.
#[derive(Debug, Clone, Copy)]
struct Fib(fn(Fib, i64, i64, i64) -> i64);

fn factorial(proc: Fact, n: i64) -> i64 {
    if n == 0 { 1 } else { n * (proc.0)(proc, n - 1) }
}

fn fibonacci(proc: Fib, n: i64, previous: i64, current: i64) -> i64 {
    if n == 0 {
        previous
    } else {
        (proc.0)(proc, n - 1, current, previous + current)
    }
}

struct Mutual {
    even: fn(i64, &Mutual) -> bool,
    odd: fn(i64, &Mutual) -> bool,
}

fn even_step(n: i64, procedures: &Mutual) -> bool {
    if n == 0 {
        true
    } else {
        (procedures.odd)(n - 1, procedures)
    }
}

fn odd_step(n: i64, procedures: &Mutual) -> bool {
    if n == 0 {
        false
    } else {
        (procedures.even)(n - 1, procedures)
    }
}

#[test]
fn ex_4_21() {
    assert_eq!(factorial(Fact(factorial), 10), 3_628_800);
    assert_eq!(fibonacci(Fib(fibonacci), 10, 0, 1), 55);

    let procedures = Mutual {
        even: even_step,
        odd: odd_step,
    };
    assert!((procedures.even)(10, &procedures));
    assert!((procedures.odd)(7, &procedures));
}
