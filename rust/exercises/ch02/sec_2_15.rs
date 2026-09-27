// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.15, one module and one ignored
//! test.

mod ex_2_15 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.15`.
        pub exercise: &'static str,
    }

    /// Exercise 2.15: is Eva right that `par2` is a "better" program?
    ///
    /// Returns whether `par2`'s percentage tolerance is strictly smaller
    /// than `par1`'s, for the same two resistors as exercise 2.14.
    pub fn ex_2_15() -> Result<bool, Pending> {
        Err(Pending { exercise: "2.15" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_15() {
    assert_eq!(ex_2_15::ex_2_15(), Ok(true));
}
