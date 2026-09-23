// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.7: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_07 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.7`.
        pub exercise: &'static str,
    }

    /// Exercise 1.7: the book's `good-enough?`, an absolute tolerance of
    /// 0.001
    pub fn good_enough_absolute(_guess: f64, _x: f64) -> Result<bool, Pending> {
        Err(Pending { exercise: "1.7" })
    }

    /// The book's `sqrt`, driven by the absolute-tolerance test.
    pub fn sqrt_absolute(_x: f64) -> Result<f64, Pending> {
        Err(Pending { exercise: "1.7" })
    }

    /// The improved `sqrt`: stops when the guess stops changing by more
    /// than a small fraction of the guess.
    pub fn sqrt_relative(_x: f64) -> Result<f64, Pending> {
        Err(Pending { exercise: "1.7" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_07() {
    assert_eq!(ex_1_07::good_enough_absolute(0.03, 0.0001), Ok(true));
    let small = ex_1_07::sqrt_absolute(0.0001).unwrap_or(f64::NAN);
    assert!((small - 0.01).abs() > 0.005);
    let nine = ex_1_07::sqrt_relative(9.0).unwrap_or(f64::NAN);
    assert!((nine - 3.0).abs() < 1e-3);
    let tiny = ex_1_07::sqrt_relative(0.0001).unwrap_or(f64::NAN);
    assert!((tiny - 0.01).abs() < 1e-6);
    let big = ex_1_07::sqrt_relative(1.0e20).unwrap_or(f64::NAN);
    assert!((big - 1.0e10).abs() < 1.0e5);
}
