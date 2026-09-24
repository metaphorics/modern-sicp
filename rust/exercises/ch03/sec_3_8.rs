// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.8.

mod ex_3_08 {

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.8`.
        pub exercise: &'static str,
    }

    /// Exercise 3.8: operand evaluation order is defined
    ///
    /// Returns the value of `f(0) + f(1)` and of a fresh probe's
    /// `f(1) + f(0)`, where the probe answers its argument on the first
    /// call and 0 after.
    pub fn ex_3_08() -> Result<(i128, i128), Pending> {
        Err(Pending { exercise: "3.8" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_08() {
    assert_eq!(ex_3_08::ex_3_08(), Ok((0, 1)));
}
