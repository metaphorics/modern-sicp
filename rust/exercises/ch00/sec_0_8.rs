// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of section 0.8, one module and one
//! ignored test per exercise.

/// The pending scaffold of exercise 0.5: the stub and the
/// exercise-named test share one module so both carry the exercise's
/// name.
mod ex_0_05 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `0.5`.
        pub exercise: &'static str,
    }

    /// Exercise 0.5: sum of cubes, twice, then a checked average
    ///
    /// Sums `a^3 + (a+1)^3 + ... + b^3`, once as a recursive function and
    /// once as an iterator chain over `a..=b`; the two must agree.
    pub fn sum_cubes_recursive(_a: i64, _b: i64) -> Result<i128, Pending> {
        Err(Pending { exercise: "0.5" })
    }

    /// The same sum, as an iterator chain.
    pub fn sum_cubes_iterative(_a: i64, _b: i64) -> Result<i128, Pending> {
        Err(Pending { exercise: "0.5" })
    }

    /// The mean cube over `a..=b`, from the recursive sum, propagated
    /// through `ch00::sec_0_8::safe_div` with `?`.
    pub fn average_cube_recursive(
        _a: i64,
        _b: i64,
    ) -> Result<Result<i128, ch00::sec_0_8::ArithError>, Pending> {
        Err(Pending { exercise: "0.5" })
    }

    /// The same mean, from the iterative sum, propagated through
    /// `ch00::sec_0_8::safe_div` with `?`.
    pub fn average_cube_iterative(
        _a: i64,
        _b: i64,
    ) -> Result<Result<i128, ch00::sec_0_8::ArithError>, Pending> {
        Err(Pending { exercise: "0.5" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_0_05() {
    assert_eq!(ex_0_05::sum_cubes_recursive(1, 4), Ok(100));
    assert_eq!(ex_0_05::sum_cubes_iterative(1, 4), Ok(100));
    assert_eq!(ex_0_05::average_cube_recursive(1, 4), Ok(Ok(25)));
    assert_eq!(ex_0_05::average_cube_iterative(1, 4), Ok(Ok(25)));
    assert_eq!(
        ex_0_05::average_cube_recursive(5, 4),
        Ok(Err(ch00::sec_0_8::ArithError::DivideByZero))
    );
}
