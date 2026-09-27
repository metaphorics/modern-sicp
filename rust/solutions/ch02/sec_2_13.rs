// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.13: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_13 {
    use ch02::sec_2_1::{Interval, mul_interval};
    use sicp_runtime::SchemeError;

    /// This exercise's own copy of exercise 2.12's `make-center-percent`;
    /// exercise files are compiled as independent test binaries, so each
    /// one that needs a small helper defines it locally rather than
    /// sharing a module across files.
    fn from_center_percent(center: f64, percent: f64) -> Result<Interval, SchemeError> {
        Interval::from_center_width(center, center.abs() * percent / 100.0)
    }

    fn percent(interval: &Interval) -> f64 {
        100.0 * interval.width() / interval.center().abs()
    }

    /// Exercise 2.13: the percentage tolerance of a product, for small
    /// tolerances on positive numbers, is approximately the sum of the
    /// factors' percentage tolerances.
    ///
    /// Returns the exact percentage tolerance of the product of two
    /// intervals (centers `10.0` and `20.0`, tolerances `1.0` and `2.0`
    /// percent), and the approximating sum of the two factors'
    /// tolerances.
    ///
    /// # Errors
    /// [`SchemeError`] when the fixed inputs below fail to build an
    /// interval, which they do not.
    pub fn ex_2_13() -> Result<(f64, f64), SchemeError> {
        let a = from_center_percent(10.0, 1.0)?;
        let b = from_center_percent(20.0, 2.0)?;
        let product = mul_interval(&a, &b);
        Ok((percent(&product), percent(&a) + percent(&b)))
    }
}

#[test]
fn ex_2_13() {
    let (product_percent, approx_percent) = ex_2_13::ex_2_13().expect("fixed inputs never fail");
    assert!((product_percent - approx_percent).abs() < 0.01);
}
