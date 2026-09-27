// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.25: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_25 {
    use sicp_runtime::SchemeError;

    /// `fast-expt`, checked: every multiplication can overflow `i128`,
    /// and the caller finds out instead of silently wrapping.
    fn fast_expt_checked(b: i128, n: u64) -> Result<i128, SchemeError> {
        if n == 0 {
            return Ok(1);
        }
        if n.is_multiple_of(2) {
            let half = fast_expt_checked(b, n / 2)?;
            half.checked_mul(half).ok_or(SchemeError::Overflow)
        } else {
            let rest = fast_expt_checked(b, n - 1)?;
            b.checked_mul(rest).ok_or(SchemeError::Overflow)
        }
    }

    /// Alyssa's `expmod`: compute the full power first, and reduce
    /// afterward. Unlike the book's `expmod`, the intermediate value is
    /// the entire `base^exp`, not a residue below `m`.
    fn alyssa_expmod(base: i128, exp: u64, m: i128) -> Option<i128> {
        fast_expt_checked(base, exp).ok().map(|power| power % m)
    }

    /// Exercise 1.25: Alyssa's `expmod` and where its width runs out
    ///
    /// Returns `Some` of Alyssa's checked result for a small case where
    /// `fast_expt` still fits `i128`, and `None` for a prime-sized case
    /// where it overflows. She is not correct: the book's `expmod` of
    /// 1.2.6 never lets its intermediates leave the neighborhood of `m`,
    /// while her version needs the full `base^exp` to exist first, and
    /// for a witness `base` near a million-sized prime `n` that value
    /// has millions of digits long before `checked_mul` gives up.
    pub fn ex_1_25() -> (Option<i128>, Option<i128>) {
        let small = alyssa_expmod(3, 5, 7);
        let large = alyssa_expmod(1_000_002, 1_000_003, 1_000_003);
        (small, large)
    }
}

#[test]
fn ex_1_25() {
    let (small, large) = ex_1_25::ex_1_25();
    assert_eq!(small, Some(5));
    assert_eq!(large, None);
}
