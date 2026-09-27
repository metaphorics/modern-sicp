// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.3: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_03 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.3`.
        pub exercise: &'static str,
    }

    /// Exercise 1.3: the sum of the squares of the two larger of three
    /// numbers
    pub fn sum_squares_two_larger(_a: i64, _b: i64, _c: i64) -> Result<i64, Pending> {
        Err(Pending { exercise: "1.3" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_03() {
    assert_eq!(ex_1_03::sum_squares_two_larger(1, 2, 3), Ok(13));
    assert_eq!(ex_1_03::sum_squares_two_larger(2, 2, 2), Ok(8));
    assert_eq!(ex_1_03::sum_squares_two_larger(-1, -2, -3), Ok(5));
}
