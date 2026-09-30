// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.9: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_09 {
    use ch02::sec_2_1::{Interval, add_interval, mul_interval};
    use sicp_runtime::SicpError;

    /// Exercise 2.9: the width of a sum is a function of the widths of
    /// the addends, but the width of a product is not
    ///
    /// Returns the width of `[2, 6] + [10, 14]`, the sum of the two
    /// addends' widths, and whether `[2, 6] * [10, 14]` and
    /// `[2, 6] * [100, 104]` (same factor, two same-width second factors
    /// with different centers) have the same width. `Interval`'s
    /// constructor enforces `lower <= upper`, so `width` is always
    /// well defined; this exercise is about the mathematics of that
    /// well-defined width, not about deciding the invariant.
    ///
    /// # Errors
    /// [`SicpError`] when the fixed bounds below fail to build an
    /// interval, which they do not.
    pub fn ex_2_09() -> Result<(f64, f64, bool), SicpError> {
        let a = Interval::new(2.0, 6.0)?;
        let b = Interval::new(10.0, 14.0)?;
        let c = Interval::new(100.0, 104.0)?;

        let sum_width = add_interval(&a, &b).width();
        let width_sum_of_addends = a.width() + b.width();

        let width_with_second_factor_b = mul_interval(&a, &b).width();
        let width_with_second_factor_c = mul_interval(&a, &c).width();
        let mul_widths_agree =
            (width_with_second_factor_b - width_with_second_factor_c).abs() < 1e-9;

        Ok((sum_width, width_sum_of_addends, mul_widths_agree))
    }
}

#[test]
fn ex_2_09() {
    assert_eq!(ex_2_09::ex_2_09(), Ok((4.0, 4.0, false)));
}
