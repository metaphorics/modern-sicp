// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution tests of section 0.8: one test per
//! exercise with its statement in a doc comment, and shared code in
//! the matching src module.

/// The reference solution of exercise 0.5: the stub and the
/// exercise-named test share one module so both carry the exercise's
/// name.
mod ex_0_05 {
    use ch00::sec_0_8::{ArithError, safe_div};

    /// Exercise 0.5: sum of cubes, twice, then a checked average
    ///
    /// Sums `a^3 + (a+1)^3 + ... + b^3` by recursion.
    pub fn sum_cubes_recursive(a: i64, b: i64) -> i128 {
        if a > b {
            0
        } else {
            i128::from(a).pow(3) + sum_cubes_recursive(a + 1, b)
        }
    }

    /// The same sum, as an iterator chain over `a..=b`.
    pub fn sum_cubes_iterative(a: i64, b: i64) -> i128 {
        (a..=b).map(|i| i128::from(i).pow(3)).sum()
    }

    /// The mean cube over `a..=b`, from the recursive sum, propagated
    /// through `safe_div` with `?`.
    ///
    /// # Errors
    /// [`ArithError::DivideByZero`] when `a..=b` is empty (`a > b`).
    pub fn average_cube_recursive(a: i64, b: i64) -> Result<i128, ArithError> {
        let total = sum_cubes_recursive(a, b);
        let average = safe_div(total, i128::from(b - a + 1))?;
        Ok(average)
    }

    /// The same mean, from the iterative sum, propagated through
    /// `safe_div` with `?`.
    ///
    /// # Errors
    /// [`ArithError::DivideByZero`] when `a..=b` is empty (`a > b`).
    pub fn average_cube_iterative(a: i64, b: i64) -> Result<i128, ArithError> {
        let total = sum_cubes_iterative(a, b);
        let average = safe_div(total, i128::from(b - a + 1))?;
        Ok(average)
    }
}

#[test]
fn ex_0_05() {
    assert_eq!(ex_0_05::sum_cubes_recursive(1, 4), 100);
    assert_eq!(ex_0_05::sum_cubes_iterative(1, 4), 100);
    assert_eq!(ex_0_05::average_cube_recursive(1, 4), Ok(25));
    assert_eq!(ex_0_05::average_cube_iterative(1, 4), Ok(25));
    assert_eq!(
        ex_0_05::average_cube_recursive(5, 4),
        Err(ch00::sec_0_8::ArithError::DivideByZero)
    );
}
