// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.24: a table whose key comparison is a predicate.

mod ex_3_24 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.24`.
        pub exercise: &'static str,
    }

    /// Exercise 3.24: a table whose key comparison is a predicate
    ///
    /// The solved entry point returns the exercise's answers as
    /// `(Option<i128>)`; the pending body reports [`Pending`].
    pub fn ex_3_24() -> Result<Option<i128>, Pending> {
        Err(Pending { exercise: "3.24" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_24() {
    let _ = ex_3_24::ex_3_24();
}
