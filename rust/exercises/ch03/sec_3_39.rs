// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.39: the outcomes that survive when
//! only the multiplication is serialized.

mod ex_3_39 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.39`.
        pub exercise: &'static str,
    }

    /// Exercise 3.39: which serialized outcomes remain
    ///
    /// Answers the sorted values left by the partially serialized
    /// program's forced interleavings.
    pub fn ex_3_39() -> Result<Vec<i128>, Pending> {
        Err(Pending { exercise: "3.39" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_39() {
    assert_eq!(ex_3_39::ex_3_39(), Ok(vec![100, 101, 121]));
}
