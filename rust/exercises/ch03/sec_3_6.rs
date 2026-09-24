// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.6.

mod ex_3_06 {

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.6`.
        pub exercise: &'static str,
    }

    /// Exercise 3.6: rand with generate and reset messages
    ///
    /// Returns whether three draws replay identically after a reset to
    /// the original state, and whether a reset to a different state
    /// produces a different continuation.
    pub fn ex_3_06() -> Result<(bool, bool), Pending> {
        Err(Pending { exercise: "3.6" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_06() {
    assert_eq!(ex_3_06::ex_3_06(), Ok((true, true)));
}
