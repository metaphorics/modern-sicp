// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.32: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_32 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.32`.
        pub exercise: &'static str,
    }

    /// Exercise 1.32: the `accumulate` abstraction
    ///
    /// Returns the recursive spelling's sum of 1 through 10, its product
    /// for the factorial of 10, and then the loop spelling's values for
    /// the same two.
    pub fn ex_1_32() -> Result<(f64, f64, f64, f64), Pending> {
        Err(Pending { exercise: "1.32" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_32() {
    let (sum_rec, product_rec, sum_loop, product_loop) =
        ex_1_32::ex_1_32().unwrap_or((0.0, 0.0, 0.0, 0.0));
    assert!((sum_rec - 55.0).abs() < 1e-9);
    assert!((product_rec - 3_628_800.0).abs() < 1e-6);
    assert!((sum_loop - 55.0).abs() < 1e-9);
    assert!((product_loop - 3_628_800.0).abs() < 1e-6);
}
