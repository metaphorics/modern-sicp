// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.8: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_08 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.8`.
        pub exercise: &'static str,
    }

    /// Exercise 1.8: Newton's cube-root improvement step, `(x/y² + 2y) / 3`
    #[allow(dead_code)]
    pub fn cube_improve(_guess: f64, _x: f64) -> Result<f64, Pending> {
        Err(Pending { exercise: "1.8" })
    }

    /// The cube root of `x` by Newton's method.
    pub fn cube_root(_x: f64) -> Result<f64, Pending> {
        Err(Pending { exercise: "1.8" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_08() {
    let eight = ex_1_08::cube_root(8.0).unwrap_or(f64::NAN);
    assert!((eight - 2.0).abs() < 1e-5);
    let twenty_seven = ex_1_08::cube_root(27.0).unwrap_or(f64::NAN);
    assert!((twenty_seven - 3.0).abs() < 1e-5);
    let thousandth = ex_1_08::cube_root(0.001).unwrap_or(f64::NAN);
    assert!((thousandth - 0.1).abs() < 1e-7);
    let big = ex_1_08::cube_root(1.0e12).unwrap_or(f64::NAN);
    assert!((big - 1.0e4).abs() < 1.0);
}
