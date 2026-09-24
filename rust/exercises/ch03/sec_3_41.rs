// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.41: what a balance reader observes
//! during a withdrawal, serialized read or not.

mod ex_3_41 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.41`.
        pub exercise: &'static str,
    }

    /// Exercise 3.41: should balance reads serialize
    ///
    /// Answers the sorted observations of the text's account and of
    /// Ben's serialized-balance account during one withdrawal.
    pub fn ex_3_41() -> Result<(Vec<i128>, Vec<i128>), Pending> {
        Err(Pending { exercise: "3.41" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_41() {
    assert_eq!(ex_3_41::ex_3_41(), Ok((vec![50, 100], vec![50, 100])));
}
