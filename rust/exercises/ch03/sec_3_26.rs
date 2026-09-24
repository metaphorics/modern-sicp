// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.26: a table as a binary tree ordered by key.

mod ex_3_26 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.26`.
        pub exercise: &'static str,
    }

    /// Exercise 3.26: a table as a binary tree ordered by key
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((Option<i128>, Vec<i128>))`; the pending body reports [`Pending`].
    pub fn ex_3_26() -> Result<(Option<i128>, Vec<i128>), Pending> {
        Err(Pending { exercise: "3.26" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_26() {
    let _ = ex_3_26::ex_3_26();
}
