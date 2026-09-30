// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.34, one module and one test.

mod ex_2_34 {
    use ch02::sec_2_2::accumulate;
    use sicp_runtime::SicpError;

    /// Exercise 2.34: `horner-eval`
    ///
    /// Horner's rule is a right fold over the coefficient sequence:
    /// each step takes the coefficients accumulated so far (the value
    /// of the polynomial over the higher terms), multiplies by `x`, and
    /// adds this coefficient. The sequence runs from `a_0` through
    /// `a_n`, so the fold reaches `a_n` first, exactly the order the
    /// nested formula needs.
    ///
    /// # Errors
    /// [`SicpError::Overflow`] when an intermediate product or sum
    /// leaves the `i128` range, which is this edition's arithmetic
    /// contract for exact integers.
    fn horner_eval(x: i128, coefficient_sequence: &[i128]) -> Result<i128, SicpError> {
        accumulate(
            |this_coeff, acc: Result<i128, SicpError>| {
                let higher_terms = acc?;
                let shifted = higher_terms.checked_mul(x).ok_or(SicpError::Overflow)?;
                this_coeff.checked_add(shifted).ok_or(SicpError::Overflow)
            },
            Ok(0),
            coefficient_sequence,
        )
    }

    /// Exercise 2.34: Horner's rule
    ///
    /// Returns `1 + 3x + 5x^3 + x^5` evaluated at `x = 2` by the
    /// `accumulate`-based `horner_eval` over `(1 3 0 5 0 1)`.
    pub fn ex_2_34() -> i128 {
        horner_eval(2, &[1, 3, 0, 5, 0, 1]).expect("book-scale values fit i128")
    }
}

#[test]
fn ex_2_34() {
    assert_eq!(ex_2_34::ex_2_34(), 79);
}
