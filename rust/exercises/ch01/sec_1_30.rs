// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.30: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_30 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.30`.
        pub exercise: &'static str,
    }

    /// Exercise 1.30: `sum` as an iterative process
    ///
    /// Returns the iterative sum of the integers from 1 through 10.
    pub fn ex_1_30() -> Result<f64, Pending> {
        Err(Pending { exercise: "1.30" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_30() {
    let total = ex_1_30::ex_1_30().unwrap_or(0.0);
    assert!((total - 55.0).abs() < 1e-9);
}
