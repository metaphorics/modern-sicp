// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.13, one module and one ignored
//! test.

mod ex_2_13 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.13`.
        pub exercise: &'static str,
    }

    /// Exercise 2.13: the percentage tolerance of a product, for small
    /// tolerances on positive numbers
    ///
    /// Returns the exact percentage tolerance of the product of two
    /// intervals (centers `10.0` and `20.0`, tolerances `1.0` and `2.0`
    /// percent), and the approximating sum of the two factors'
    /// tolerances.
    pub fn ex_2_13() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "2.13" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_13() {
    let (product_percent, approx_percent) = ex_2_13::ex_2_13().expect("solved");
    assert!((product_percent - approx_percent).abs() < 0.01);
}
