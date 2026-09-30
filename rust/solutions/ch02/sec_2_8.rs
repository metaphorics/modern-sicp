// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.8: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_08 {
    use ch02::sec_2_1::Interval;
    use sicp_runtime::SicpError;

    /// Exercise 2.8: `sub-interval`, reasoning analogous to Alyssa's
    /// `add-interval`: the smallest possible difference is `x`'s lower
    /// bound minus `y`'s upper bound, and the largest is `x`'s upper
    /// bound minus `y`'s lower bound. For any two well-formed intervals
    /// this is itself well formed, since `x.lower - y.upper <= x.upper -
    /// y.lower` whenever `x.lower <= x.upper` and `y.lower <= y.upper`.
    ///
    /// # Errors
    /// Never, for `x` and `y` built by `Interval::new`; the `Result` only
    /// reflects [`Interval::new`]'s own signature.
    fn sub_interval(x: &Interval, y: &Interval) -> Result<Interval, SicpError> {
        Interval::new(
            x.lower_bound() - y.upper_bound(),
            x.upper_bound() - y.lower_bound(),
        )
    }

    /// Exercise 2.8: `sub-interval`
    ///
    /// Returns the lower and upper bounds of `[6, 8] - [1, 3]`.
    ///
    /// # Errors
    /// [`SicpError`] when the fixed bounds below fail to build an
    /// interval, which they do not.
    pub fn ex_2_08() -> Result<(f64, f64), SicpError> {
        let x = Interval::new(6.0, 8.0)?;
        let y = Interval::new(1.0, 3.0)?;
        let diff = sub_interval(&x, &y)?;
        Ok((diff.lower_bound(), diff.upper_bound()))
    }
}

#[test]
fn ex_2_08() {
    assert_eq!(ex_2_08::ex_2_08(), Ok((3.0, 7.0)));
}
