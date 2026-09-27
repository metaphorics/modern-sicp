// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.7: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_07 {
    use ch01::sec_1_1::{abs, square};

    /// Exercise 1.7: the book's `good-enough?`, an absolute tolerance of
    /// 0.001
    ///
    /// Already true for a bad guess on a tiny radicand: `0.03` squared
    /// misses `0.0001` by less than the tolerance.
    pub fn good_enough_absolute(guess: f64, x: f64) -> bool {
        abs(square(guess) - x) < 0.001
    }

    /// The book's `sqrt`, driven by the absolute-tolerance test.
    ///
    /// Accurate near 1, badly wrong for small radicands; for very large
    /// ones it never terminates, because near the root the squares of
    /// successive guesses differ from `x` by more than 0.001 no matter
    /// how good the guesses are.
    pub fn sqrt_absolute(x: f64) -> f64 {
        fn iter(guess: f64, x: f64) -> f64 {
            if good_enough_absolute(guess, x) {
                guess
            } else {
                iter(f64::midpoint(guess, x / guess), x)
            }
        }
        iter(1.0, x)
    }

    /// The improved `sqrt`: stops when the guess stops changing by more
    /// than a small fraction of the guess.
    ///
    /// The end test compares successive guesses, so the tolerance scales
    /// with the radicand: small roots come out accurate, large ones
    /// terminate.
    pub fn sqrt_relative(x: f64) -> f64 {
        fn good_enough_relative(guess: f64, next: f64) -> bool {
            abs(next - guess) < 0.001 * guess
        }
        fn iter(guess: f64, x: f64) -> f64 {
            let next = f64::midpoint(guess, x / guess);
            if good_enough_relative(guess, next) {
                next
            } else {
                iter(next, x)
            }
        }
        iter(1.0, x)
    }
}

#[test]
fn ex_1_07() {
    assert!(ex_1_07::good_enough_absolute(0.03, 0.0001));
    let small = ex_1_07::sqrt_absolute(0.0001);
    assert!((small - 0.01).abs() > 0.005);
    let nine = ex_1_07::sqrt_relative(9.0);
    assert!((nine - 3.0).abs() < 1e-3);
    let tiny = ex_1_07::sqrt_relative(0.0001);
    assert!((tiny - 0.01).abs() < 1e-6);
    let big = ex_1_07::sqrt_relative(1.0e20);
    assert!((big - 1.0e10).abs() < 1.0e5);
}
