// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.31: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_31 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.31`.
        pub exercise: &'static str,
    }

    /// Exercise 1.31: the `product` abstraction
    ///
    /// Returns the factorial of 10 computed with `product` first, and the
    /// Wallis estimate of pi computed with `product` second.
    pub fn ex_1_31() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "1.31" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_31() {
    let (factorial_10, wallis_pi) = ex_1_31::ex_1_31().unwrap_or((0.0, 0.0));
    assert!((factorial_10 - 3_628_800.0).abs() < 1e-6);
    assert!((wallis_pi - std::f64::consts::PI).abs() < 0.01);
}
