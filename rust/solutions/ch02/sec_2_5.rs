// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.5: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_05 {
    use sicp_runtime::SchemeError;

    /// Exercise 2.5's `cons`: `2^a * 3^b`. This edition's numbers
    /// convention keeps every exact integer in `i128` with checked
    /// arithmetic; 2 and 3 are coprime, so no encodable pair of small
    /// `a` and `b` values used in this exercise comes near overflowing
    /// it.
    ///
    /// # Errors
    /// [`SchemeError::Overflow`] when `2^a * 3^b` does not fit `i128`.
    fn cons(a: u32, b: u32) -> Result<i128, SchemeError> {
        let two_to_a = 2i128.checked_pow(a).ok_or(SchemeError::Overflow)?;
        let three_to_b = 3i128.checked_pow(b).ok_or(SchemeError::Overflow)?;
        two_to_a
            .checked_mul(three_to_b)
            .ok_or(SchemeError::Overflow)
    }

    /// Counts how many times `factor` divides `z`.
    fn count_factor(mut z: i128, factor: i128) -> u32 {
        let mut count = 0;
        while z % factor == 0 {
            z /= factor;
            count += 1;
        }
        count
    }

    /// Exercise 2.5's `car`: the power of 2 dividing `z`.
    fn car(z: i128) -> u32 {
        count_factor(z, 2)
    }

    /// Exercise 2.5's `cdr`: the power of 3 dividing `z`.
    fn cdr(z: i128) -> u32 {
        count_factor(z, 3)
    }

    /// Exercise 2.5: pairs of nonnegative integers as `2^a * 3^b`
    ///
    /// Returns `car` and `cdr` of the pair `(3, 2)` recovered from its
    /// `2^a * 3^b` encoding.
    ///
    /// # Errors
    /// [`SchemeError::Overflow`] when `cons` overflows, which does not
    /// arise from encoding `(3, 2)`.
    pub fn ex_2_05() -> Result<(u32, u32), SchemeError> {
        let z = cons(3, 2)?;
        Ok((car(z), cdr(z)))
    }
}

#[test]
fn ex_2_05() {
    assert_eq!(ex_2_05::ex_2_05(), Ok((3, 2)));
}
