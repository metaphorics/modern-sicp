// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.4: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_04 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.4`.
        pub exercise: &'static str,
    }

    /// Adds two numbers: one of the two operator functions the branch may
    /// yield.
    #[allow(dead_code)]
    pub fn add(a: i64, b: i64) -> i64 {
        a + b
    }

    /// Subtracts the second number from the first: the other operator
    /// function.
    #[allow(dead_code)]
    pub fn sub(a: i64, b: i64) -> i64 {
        a - b
    }

    /// Exercise 1.4: `a + |b|`, built by letting a branch choose the
    /// operator function
    pub fn a_plus_abs_b(_a: i64, _b: i64) -> Result<i64, Pending> {
        Err(Pending { exercise: "1.4" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_04() {
    assert_eq!(ex_1_04::a_plus_abs_b(1, 5), Ok(6));
    assert_eq!(ex_1_04::a_plus_abs_b(1, -5), Ok(6));
    assert_eq!(ex_1_04::a_plus_abs_b(2, 0), Ok(2));
}
