// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.16: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_16 {
    use ch02::sec_2_1::Interval;
    use sicp_runtime::SchemeError;

    fn from_center_percent(center: f64, percent: f64) -> Result<Interval, SchemeError> {
        Interval::from_center_width(center, center.abs() * percent / 100.0)
    }

    /// This exercise's own `sub-interval`, reasoning identically to
    /// exercise 2.8's.
    ///
    /// # Errors
    /// Never, for `x` and `y` built by `Interval::new`.
    fn sub_interval(x: &Interval, y: &Interval) -> Result<Interval, SchemeError> {
        Interval::new(
            x.lower_bound() - y.upper_bound(),
            x.upper_bound() - y.lower_bound(),
        )
    }

    /// Exercise 2.16: equivalent algebraic expressions can give different
    /// answers
    ///
    /// No general interval-arithmetic package built from independent
    /// endpoint bookkeeping can avoid this: it does not track which
    /// occurrences of a variable in an expression name the same
    /// quantity, so it cannot cancel them the way ordinary algebra does.
    /// The simplest instance is `x - x`: exercises 2.14 and 2.15's `A / A`
    /// and `par1` versus `par2` are the same phenomenon on more elaborate
    /// expressions.
    ///
    /// Returns the lower and upper bounds of `x - x` for `x` at 10.0
    /// with 5 percent tolerance, the simplest case of the dependency
    /// problem underlying exercises 2.14 and 2.15.
    ///
    /// # Errors
    /// [`SchemeError`] when the fixed input below fails to build, which
    /// it does not.
    pub fn ex_2_16() -> Result<(f64, f64), SchemeError> {
        let x = from_center_percent(10.0, 5.0)?;
        let difference = sub_interval(&x, &x)?;
        Ok((difference.lower_bound(), difference.upper_bound()))
    }
}

#[test]
fn ex_2_16() {
    assert_eq!(ex_2_16::ex_2_16(), Ok((-1.0, 1.0)));
}
