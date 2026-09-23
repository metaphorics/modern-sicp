// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of section 0.2, one module and one
//! ignored test per exercise.

/// The pending scaffold of exercise 0.1: the stub and the
/// exercise-named test share one module so both carry the exercise's
/// name.
mod ex_0_01 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `0.1`.
        pub exercise: &'static str,
    }

    /// Exercise 0.1: factorial by checked multiplication
    ///
    /// Computes `n!` recursively with `checked_mul`, returning `None`
    /// instead of wrapping when the exact result would overflow `i128`.
    pub fn factorial(_n: u32) -> Result<Option<i128>, Pending> {
        Err(Pending { exercise: "0.1" })
    }

    /// The largest `n` for which [`factorial`] still returns `Some`.
    pub fn largest_factorial_n() -> Result<u32, Pending> {
        Err(Pending { exercise: "0.1" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_0_01() {
    assert_eq!(ex_0_01::factorial(5), Ok(Some(120)));
    assert_eq!(ex_0_01::factorial(34), Ok(None));
    assert_eq!(ex_0_01::largest_factorial_n(), Ok(33));
}

/// The pending scaffold of exercise 0.2: the stub and the
/// exercise-named test share one module so both carry the exercise's
/// name.
mod ex_0_02 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `0.2`.
        pub exercise: &'static str,
    }

    /// Exercise 0.2: change-counting, recursive and iterative
    ///
    /// Counts the ways to make `amount` cents from half-dollars,
    /// quarters, dimes, nickels, and pennies, by the book's recursive
    /// process (1.2.2).
    pub fn count_change_recursive(_amount: i64) -> Result<i64, Pending> {
        Err(Pending { exercise: "0.2" })
    }

    /// The same count, by a loop over an accumulator table indexed by
    /// amount, one pass per denomination.
    pub fn count_change_iterative(_amount: i64) -> Result<i64, Pending> {
        Err(Pending { exercise: "0.2" })
    }

    /// The maximum call depth [`count_change_recursive`] reaches while
    /// counting change for `amount`, to compare against the loop's
    /// constant depth.
    pub fn recursive_depth(_amount: i64) -> Result<u32, Pending> {
        Err(Pending { exercise: "0.2" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_0_02() {
    assert_eq!(ex_0_02::count_change_recursive(100), Ok(292));
    assert_eq!(ex_0_02::count_change_iterative(100), Ok(292));
    assert_eq!(ex_0_02::count_change_iterative(400), Ok(26_517));
    assert_eq!(
        ex_0_02::recursive_depth(400).map(|depth| depth > 400),
        Ok(true)
    );
}
