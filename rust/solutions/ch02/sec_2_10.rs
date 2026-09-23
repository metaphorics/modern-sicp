// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.10: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_10 {
    use ch02::sec_2_1::{Interval, mul_interval};
    use sicp_runtime::SchemeError;

    /// Exercise 2.10: `div-interval`, checking that the divisor does not
    /// span zero before dividing, and signaling
    /// [`SchemeError::DivisionByZero`] when it does, instead of
    /// `ch02::sec_2_1::div_interval`'s undefined numeric result.
    ///
    /// # Errors
    /// [`SchemeError::DivisionByZero`] when `y`'s bounds straddle or
    /// touch zero. [`SchemeError::TypeMismatch`] when the reciprocal
    /// interval fails to build, which cannot happen once the zero check
    /// above passes.
    fn div_interval_checked(x: &Interval, y: &Interval) -> Result<Interval, SchemeError> {
        if y.lower_bound() <= 0.0 && y.upper_bound() >= 0.0 {
            return Err(SchemeError::DivisionByZero);
        }
        let reciprocal_y = Interval::new(1.0 / y.upper_bound(), 1.0 / y.lower_bound())?;
        Ok(mul_interval(x, &reciprocal_y))
    }

    /// Exercise 2.10: `div-interval` signals an error when the divisor
    /// spans zero
    ///
    /// Returns the bounds of `[6, 8] / [2, 4]`, and whether dividing by
    /// `[-1, 1]` (which spans zero) is rejected.
    ///
    /// # Errors
    /// [`SchemeError`] when the fixed bounds below fail to build an
    /// interval, or when the valid division is rejected, neither of
    /// which happens for these inputs.
    pub fn ex_2_10() -> Result<((f64, f64), bool), SchemeError> {
        let x = Interval::new(6.0, 8.0)?;
        let y = Interval::new(2.0, 4.0)?;
        let quotient = div_interval_checked(&x, &y)?;

        let spans_zero = Interval::new(-1.0, 1.0)?;
        let rejected = div_interval_checked(&x, &spans_zero).is_err();

        Ok(((quotient.lower_bound(), quotient.upper_bound()), rejected))
    }
}

#[test]
fn ex_2_10() {
    assert_eq!(ex_2_10::ex_2_10(), Ok(((1.5, 4.0), true)));
}
