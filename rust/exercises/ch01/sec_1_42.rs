// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.42: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_42 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.42`.
        pub exercise: &'static str,
    }

    /// Exercise 1.42: composition
    ///
    /// Returns `(compose square inc)(6)`, which is 49.
    pub fn ex_1_42() -> Result<f64, Pending> {
        Err(Pending { exercise: "1.42" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_42() {
    let value = ex_1_42::ex_1_42().unwrap_or(0.0);
    assert!((value - 49.0).abs() < 1e-9);
}
