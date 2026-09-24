// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.49: a scenario where the needed
//! resource set is unknown when the first lock is taken, so ordering
//! cannot save it.

mod ex_3_49 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.49`.
        pub exercise: &'static str,
    }

    /// Exercise 3.49: ordering avoidance fails scenario
    ///
    /// Answers whether each of the two chain-walking processes deadlocks
    /// holding its first account while waiting for the one it discovers
    /// only by reading that account.
    pub fn ex_3_49() -> Result<(bool, bool), Pending> {
        Err(Pending { exercise: "3.49" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_49() {
    assert_eq!(ex_3_49::ex_3_49(), Ok((true, true)));
}
