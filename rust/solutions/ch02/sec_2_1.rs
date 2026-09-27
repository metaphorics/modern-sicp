// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solutions of exercise 2.1 and its tailored addition
//! 2.1a, one module and one test per exercise.

mod ex_2_01 {
    use ch02::sec_2_1::Rational;

    /// Exercise 2.1: a sign-normalizing `make-rat`
    ///
    /// Builds on `Rational::new`'s `gcd` reduction, then moves the sign
    /// of the result onto the numerator, so a rational number is
    /// negative if and only if its numerator is negative.
    fn make_rat(num: i128, den: i128) -> (i128, i128) {
        let reduced = Rational::new(num, den).expect("den is never zero in this exercise's cases");
        let (n, d) = (reduced.numer(), reduced.denom());
        if d < 0 { (-n, -d) } else { (n, d) }
    }

    /// Exercise 2.1: a sign-normalizing `make-rat`
    ///
    /// Returns the reduced `(numerator, denominator)` for each of the
    /// four sign combinations of `1 / 2`, in the order `(+, +)`,
    /// `(+, -)`, `(-, -)`, `(-, +)`, with the sign normalized onto the
    /// numerator in every case.
    pub fn ex_2_01() -> [(i128, i128); 4] {
        [
            make_rat(1, 2),
            make_rat(1, -2),
            make_rat(-1, -2),
            make_rat(-1, 2),
        ]
    }
}

#[test]
fn ex_2_01() {
    assert_eq!(ex_2_01::ex_2_01(), [(1, 2), (-1, 2), (1, 2), (-1, 2)]);
}

/// Exercise 2.1a (this edition): the first product in a chain of
/// self-multiplications of a rational number to overflow `i128`.
mod ex_2_01a {
    use ch02::sec_2_1::Rational;
    use sicp_runtime::SchemeError;

    /// Exercise 2.1a: repeatedly squares `Rational::new(99, 100)` and
    /// returns the 1-indexed attempt number of the first squaring that
    /// overflows `i128`.
    ///
    /// # Errors
    /// A [`SchemeError`] other than [`SchemeError::Overflow`], which does
    /// not arise from squaring `99/100`.
    pub fn ex_2_01a() -> Result<usize, SchemeError> {
        let mut product = Rational::new(99, 100)?;
        let mut attempts = 0usize;
        loop {
            attempts += 1;
            match product.mul(&product) {
                Ok(next) => product = next,
                Err(SchemeError::Overflow) => return Ok(attempts),
                Err(other) => return Err(other),
            }
        }
    }
}

#[test]
fn ex_2_01a() {
    assert_eq!(ex_2_01a::ex_2_01a(), Ok(5));
}
