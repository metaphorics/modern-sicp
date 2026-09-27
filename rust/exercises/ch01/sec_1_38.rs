// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.38: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_38 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.38`.
        pub exercise: &'static str,
    }

    /// Exercise 1.38: Euler's expansion for `e`
    ///
    /// Returns the approximation of `e` from Euler's continued fraction
    /// at `k = 10`.
    pub fn ex_1_38() -> Result<f64, Pending> {
        Err(Pending { exercise: "1.38" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_38() {
    let e = ex_1_38::ex_1_38().unwrap_or(0.0);
    assert!((e - std::f64::consts::E).abs() < 1e-6);
}
