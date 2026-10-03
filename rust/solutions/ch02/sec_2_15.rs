// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.15: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_15 {
    use ch02::sec_2_1::{Interval, add_interval, div_interval, mul_interval};
    use sicp_runtime::SicpError;

    fn from_center_percent(center: f64, percent: f64) -> Result<Interval, SicpError> {
        Interval::from_center_width(center, center.abs() * percent / 100.0)
    }

    fn percent(interval: &Interval) -> f64 {
        100.0 * interval.width() / interval.center().abs()
    }

    fn par1(r1: &Interval, r2: &Interval) -> Interval {
        div_interval(&mul_interval(r1, r2), &add_interval(r1, r2))
    }

    fn par2(r1: &Interval, r2: &Interval) -> Result<Interval, SicpError> {
        let one = Interval::new(1.0, 1.0)?;
        let reciprocal_sum = add_interval(&div_interval(&one, r1), &div_interval(&one, r2));
        Ok(div_interval(&one, &reciprocal_sum))
    }

    /// Exercise 2.15: is Eva right that `par2` is a "better" program?
    ///
    /// Eva's diagnosis is that a formula which repeats a variable more
    /// times gives a wider (less certain) answer than an equivalent
    /// formula that repeats it fewer times, because each repetition is
    /// treated as an independent uncertain quantity. `par1` mentions
    /// `r1` and `r2` twice each; `par2` mentions each once, so this
    /// exercise predicts `par2` should report a smaller percentage
    /// tolerance.
    ///
    /// Returns whether `par2`'s percentage tolerance is strictly smaller
    /// than `par1`'s, for the same two resistors as exercise 2.14.
    ///
    /// # Errors
    /// [`SicpError`] when the fixed inputs below fail to build, which
    /// they do not.
    pub fn ex_2_15() -> Result<bool, SicpError> {
        let r1 = from_center_percent(10.0, 5.0)?;
        let r2 = from_center_percent(20.0, 5.0)?;
        let p1 = par1(&r1, &r2);
        let p2 = par2(&r1, &r2)?;
        Ok(percent(&p2) < percent(&p1))
    }
}

#[test]
fn ex_2_15() {
    assert_eq!(ex_2_15::ex_2_15(), Ok(true));
}
