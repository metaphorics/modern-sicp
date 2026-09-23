// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.12: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_12 {
    use ch02::sec_2_1::Interval;
    use sicp_runtime::SchemeError;

    /// Exercise 2.12's `make-center-percent`: a center and a percentage
    /// tolerance, built on `Interval::from_center_width` (the book's
    /// `make-center-width`, given in the main text).
    ///
    /// # Errors
    /// [`SchemeError::TypeMismatch`] when `percent` is negative.
    fn from_center_percent(center: f64, percent: f64) -> Result<Interval, SchemeError> {
        Interval::from_center_width(center, center.abs() * percent / 100.0)
    }

    /// Exercise 2.12's `percent`: the percentage tolerance of an interval
    /// around its center. The `center` selector this exercise's statement
    /// asks for is already `Interval::center`, from the main text.
    fn percent(interval: &Interval) -> f64 {
        100.0 * interval.width() / interval.center().abs()
    }

    /// Exercise 2.12: `make-center-percent` and `percent`
    ///
    /// Returns the lower bound, upper bound, and recovered percentage
    /// tolerance of the interval built from center `100.0` and tolerance
    /// `5.0` percent.
    ///
    /// # Errors
    /// [`SchemeError`] when the fixed inputs below fail to build an
    /// interval, which they do not.
    pub fn ex_2_12() -> Result<(f64, f64, f64), SchemeError> {
        let interval = from_center_percent(100.0, 5.0)?;
        Ok((
            interval.lower_bound(),
            interval.upper_bound(),
            percent(&interval),
        ))
    }
}

#[test]
fn ex_2_12() {
    assert_eq!(ex_2_12::ex_2_12(), Ok((95.0, 105.0, 5.0)));
}
