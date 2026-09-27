// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.14: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_14 {
    use ch02::sec_2_1::{Interval, add_interval, div_interval, mul_interval};
    use sicp_runtime::SchemeError;

    fn from_center_percent(center: f64, percent: f64) -> Result<Interval, SchemeError> {
        Interval::from_center_width(center, center.abs() * percent / 100.0)
    }

    fn percent(interval: &Interval) -> f64 {
        100.0 * interval.width() / interval.center().abs()
    }

    /// Exercise 2.14's `par1`: `(r1 * r2) / (r1 + r2)`, the direct
    /// translation of the parallel-resistance formula.
    fn par1(r1: &Interval, r2: &Interval) -> Interval {
        div_interval(&mul_interval(r1, r2), &add_interval(r1, r2))
    }

    /// Exercise 2.14's `par2`: `1 / (1/r1 + 1/r2)`, the algebraically
    /// equivalent second formula that mentions each resistor's interval
    /// fewer times.
    ///
    /// # Errors
    /// [`SchemeError`] when the interval `1.0 / 1.0` fails to build,
    /// which it does not.
    fn par2(r1: &Interval, r2: &Interval) -> Result<Interval, SchemeError> {
        let one = Interval::new(1.0, 1.0)?;
        let reciprocal_sum = add_interval(&div_interval(&one, r1), &div_interval(&one, r2));
        Ok(div_interval(&one, &reciprocal_sum))
    }

    /// Exercise 2.14: repeated uncertain variables make algebraically
    /// equivalent formulas disagree
    ///
    /// Returns the percentage tolerance of `A / A` for `A` at 10.0 with
    /// 5 percent tolerance, then the percentage tolerances of `par1` and
    /// `par2` for two 5-percent-tolerance resistors of 10 and 20 ohms:
    /// neither matches the input tolerance exactly, and the two formulas
    /// for parallel resistance do not agree with each other either,
    /// because interval arithmetic treats every textual occurrence of a
    /// variable as an independent uncertain quantity.
    ///
    /// # Errors
    /// [`SchemeError`] when the fixed inputs below fail to build, which
    /// they do not.
    pub fn ex_2_14() -> Result<(f64, f64, f64), SchemeError> {
        let a = from_center_percent(10.0, 5.0)?;
        let a_over_a = div_interval(&a, &a);

        let r1 = from_center_percent(10.0, 5.0)?;
        let r2 = from_center_percent(20.0, 5.0)?;
        let p1 = par1(&r1, &r2);
        let p2 = par2(&r1, &r2)?;

        Ok((percent(&a_over_a), percent(&p1), percent(&p2)))
    }
}

#[test]
fn ex_2_14() {
    let (a_over_a_percent, par1_percent, par2_percent) =
        ex_2_14::ex_2_14().expect("fixed inputs never fail");
    assert!((a_over_a_percent - 9.975).abs() < 0.01);
    assert!((par1_percent - 14.901).abs() < 0.01);
    assert!((par2_percent - 5.0).abs() < 0.01);
}
