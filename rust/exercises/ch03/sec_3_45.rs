// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.45: the double serialization that
//! makes Louis's account exchange deadlock with itself.

mod ex_3_45 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.45`.
        pub exercise: &'static str,
    }

    /// Exercise 3.45: double serialization deadlocks
    ///
    /// Answers whether each of the two opposite-order processes deadlocks
    /// waiting for the serializer it already holds.
    pub fn ex_3_45() -> Result<(bool, bool), Pending> {
        Err(Pending { exercise: "3.45" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_45() {
    assert_eq!(ex_3_45::ex_3_45(), Ok((true, true)));
}
