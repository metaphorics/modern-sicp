// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.8, one module and one ignored test.

mod ex_2_08 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.8`.
        pub exercise: &'static str,
    }

    /// Exercise 2.8: `sub-interval`
    ///
    /// Returns the lower and upper bounds of `[6, 8] - [1, 3]`.
    pub fn ex_2_08() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "2.8" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_08() {
    assert_eq!(ex_2_08::ex_2_08(), Ok((3.0, 7.0)));
}
