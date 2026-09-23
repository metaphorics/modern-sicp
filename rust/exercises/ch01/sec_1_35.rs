// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.35: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_35 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.35`.
        pub exercise: &'static str,
    }

    /// Exercise 1.35: the golden ratio as a fixed point
    ///
    /// Returns the fixed point of `x` mapped to `1 + 1 / x`.
    pub fn ex_1_35() -> Result<f64, Pending> {
        Err(Pending { exercise: "1.35" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_35() {
    let phi = ex_1_35::ex_1_35().unwrap_or(0.0);
    assert!((phi - 1.618_033_988_749_895).abs() < 1e-5);
}
