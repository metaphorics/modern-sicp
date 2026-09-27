// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.40: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_40 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.40`.
        pub exercise: &'static str,
    }

    /// Exercise 1.40: `cubic` for Newton's method
    ///
    /// Returns the zero that `newtons_method` finds for the cubic
    /// `x^3 + x^2 - 2`, whose root is 1.
    pub fn ex_1_40() -> Result<f64, Pending> {
        Err(Pending { exercise: "1.40" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_40() {
    let zero = ex_1_40::ex_1_40().unwrap_or(0.0);
    assert!((zero - 1.0).abs() < 1e-5);
}
