// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.42: whether serializing once
//! outside the dispatch changes what concurrency is allowed.

mod ex_3_42 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.42`.
        pub exercise: &'static str,
    }

    /// Exercise 3.42: serialize once outside dispatch
    ///
    /// Answers the final balance of Ben's once-serialized account and of
    /// the text's per-call account after the same concurrent deposits
    /// and one final withdrawal.
    pub fn ex_3_42() -> Result<(i128, i128), Pending> {
        Err(Pending { exercise: "3.42" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_42() {
    assert_eq!(ex_3_42::ex_3_42(), Ok((1075, 1075)));
}
