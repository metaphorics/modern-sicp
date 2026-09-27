// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.4.

mod ex_3_04 {

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.4`.
        pub exercise: &'static str,
    }

    /// Exercise 3.4: lockout after seven bad passwords
    ///
    /// Returns whether the cops were called after seven consecutive
    /// wrong-password attempts (they must not be) and after the eighth
    /// (they must be).
    pub fn ex_3_04() -> Result<(bool, bool), Pending> {
        Err(Pending { exercise: "3.4" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_04() {
    assert_eq!(ex_3_04::ex_3_04(), Ok((false, true)));
}
