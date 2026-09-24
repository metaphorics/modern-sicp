// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.3.

mod ex_3_03 {

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.3`.
        pub exercise: &'static str,
    }

    /// Exercise 3.3: a password-protected account
    ///
    /// Returns the balance answered to a correct-password withdrawal of
    /// 40 from a 100 account, and the complaint text answered to a
    /// wrong-password deposit.
    pub fn ex_3_03() -> Result<(i128, String), Pending> {
        Err(Pending { exercise: "3.3" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_03() {
    assert_eq!(
        ex_3_03::ex_3_03(),
        Ok((60, "Incorrect password".to_string()))
    );
}
