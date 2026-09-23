// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.39: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_39 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.39`.
        pub exercise: &'static str,
    }

    /// Exercise 1.39: Lambert's continued fraction for the tangent
    ///
    /// Returns `tan_cf(pi / 6, 20)`, whose true value is `1 / sqrt(3)`.
    pub fn ex_1_39() -> Result<f64, Pending> {
        Err(Pending { exercise: "1.39" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_39() {
    let tangent = ex_1_39::ex_1_39().unwrap_or(0.0);
    let expected = 1.0 / 3.0f64.sqrt();
    assert!((tangent - expected).abs() < 1e-5);
}
