// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.10, one module and one ignored
//! test.

mod ex_2_10 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.10`.
        pub exercise: &'static str,
    }

    /// Exercise 2.10: `div-interval` signals an error when the divisor
    /// spans zero
    ///
    /// Returns the bounds of `[6, 8] / [2, 4]`, and whether dividing by
    /// `[-1, 1]` (which spans zero) is rejected.
    pub fn ex_2_10() -> Result<((f64, f64), bool), Pending> {
        Err(Pending { exercise: "2.10" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_10() {
    assert_eq!(ex_2_10::ex_2_10(), Ok(((1.5, 4.0), true)));
}
