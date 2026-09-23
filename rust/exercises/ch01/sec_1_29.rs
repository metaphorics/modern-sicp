// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.29: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_29 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.29`.
        pub exercise: &'static str,
    }

    /// Exercise 1.29: Simpson's Rule
    ///
    /// Returns the Simpson approximation of the integral of `cube` over
    /// 0 to 1 at `n = 100` first and `n = 1000` second.
    pub fn ex_1_29() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "1.29" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_29() {
    let (at_100, at_1000) = ex_1_29::ex_1_29().unwrap_or((0.0, 0.0));
    assert!((at_100 - 0.25).abs() < 1e-9);
    assert!((at_1000 - 0.25).abs() < 1e-11);
}
