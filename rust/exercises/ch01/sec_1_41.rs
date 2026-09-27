// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.41: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_41 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.41`.
        pub exercise: &'static str,
    }

    /// Exercise 1.41: the `double` procedure
    ///
    /// Returns the value the book's `(((double (double double)) inc) 5)`
    /// computes, rebuilt with as many nestings of `double` around `inc`
    /// as the book's self-application builds.
    pub fn ex_1_41() -> Result<f64, Pending> {
        Err(Pending { exercise: "1.41" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_41() {
    let value = ex_1_41::ex_1_41().unwrap_or(0.0);
    assert!((value - 21.0).abs() < 1e-9);
}
