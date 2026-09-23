// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.43: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_43 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.43`.
        pub exercise: &'static str,
    }

    /// Exercise 1.43: repeated application
    ///
    /// Returns `(repeated square 2)(5)`, which is 625.
    pub fn ex_1_43() -> Result<f64, Pending> {
        Err(Pending { exercise: "1.43" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_43() {
    let value = ex_1_43::ex_1_43().unwrap_or(0.0);
    assert!((value - 625.0).abs() < 1e-9);
}
