// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.13: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_13 {
    /// `Fib(n)` by the linear iteration of 1.2.2, carried in `f64` so it
    /// stays exactly comparable with the golden-ratio approximation; the
    /// values stay exact integers up to `Fib(78)`, well past the range
    /// this exercise checks.
    fn fib_f64(n: u32) -> f64 {
        let (mut a, mut b, mut count) = (1.0_f64, 0.0_f64, n);
        while count != 0 {
            (a, b) = (a + b, a);
            count -= 1;
        }
        b
    }

    /// `phi^n / sqrt(5)`, the approximation the exercise proves is always
    /// within one half of `Fib(n)`.
    fn golden_approximation(n: u32) -> f64 {
        let phi = f64::midpoint(1.0, 5.0_f64.sqrt());
        phi.powi(i32::try_from(n).expect("n stays well under i32::MAX in this exercise"))
            / 5.0_f64.sqrt()
    }

    /// Exercise 1.13: `Fib(n)` is the closest integer to `phi^n / sqrt(5)`
    ///
    /// The exercise is a proof; the code companion verifies its numerical
    /// edge: it returns whether `Fib(n)` is the closest integer to
    /// `phi^n / sqrt(5)` for every `n` from 0 through 70, where the
    /// floating-point argument is still exact enough for the rounding
    /// claim to bite.
    pub fn ex_1_13() -> bool {
        (0..=70).all(|n| (fib_f64(n) - golden_approximation(n)).abs() < 0.5)
    }
}

#[test]
fn ex_1_13() {
    let holds = ex_1_13::ex_1_13();
    assert!(holds);
}
