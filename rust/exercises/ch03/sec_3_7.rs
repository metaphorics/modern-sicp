// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.7.

mod ex_3_07 {

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.7`.
        pub exercise: &'static str,
    }

    /// Exercise 3.7: a joint account shares one balance
    ///
    /// Returns the balance a joint-account withdrawal of 40 from a 100
    /// account answers, the balance the original account then reports,
    /// and whether a joint opened with the wrong password is refused.
    pub fn ex_3_07() -> Result<(i128, i128, bool), Pending> {
        Err(Pending { exercise: "3.7" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_07() {
    assert_eq!(ex_3_07::ex_3_07(), Ok((60, 60, true)));
}
