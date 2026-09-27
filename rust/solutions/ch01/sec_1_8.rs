// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.8: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_08 {
    use ch01::sec_1_1::{abs, square};

    /// Exercise 1.8: Newton's cube-root improvement step, `(x/y² + 2y) / 3`
    pub fn cube_improve(guess: f64, x: f64) -> f64 {
        (x / square(guess) + 2.0 * guess) / 3.0
    }

    /// The cube root of `x` by Newton's method
    ///
    /// The same shape as the square-root program, with `cube_improve` in
    /// place of `improve` and the end test of exercise 1.7 (a fraction
    /// of the guess) in place of the absolute tolerance, which fails at
    /// the extremes the same way.
    pub fn cube_root(x: f64) -> f64 {
        fn good_enough_cube(guess: f64, next: f64) -> bool {
            abs(next - guess) < 0.001 * guess
        }
        fn iter(guess: f64, x: f64) -> f64 {
            let next = cube_improve(guess, x);
            if good_enough_cube(guess, next) {
                next
            } else {
                iter(next, x)
            }
        }
        iter(1.0, x)
    }
}

#[test]
fn ex_1_08() {
    let eight = ex_1_08::cube_root(8.0);
    assert!((eight - 2.0).abs() < 1e-5);
    let twenty_seven = ex_1_08::cube_root(27.0);
    assert!((twenty_seven - 3.0).abs() < 1e-5);
    let thousandth = ex_1_08::cube_root(0.001);
    assert!((thousandth - 0.1).abs() < 1e-7);
    let big = ex_1_08::cube_root(1.0e12);
    assert!((big - 1.0e4).abs() < 1.0);
}
