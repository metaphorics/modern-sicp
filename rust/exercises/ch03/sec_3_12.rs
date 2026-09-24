// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.12: append! mutates the tail of a shared chain.

mod ex_3_12 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.12`.
        pub exercise: &'static str,
    }

    /// Exercise 3.12: append! mutates the tail of a shared chain
    ///
    /// The solved entry point returns the exercise's answers as
    /// `(Vec<String>)`; the pending body reports [`Pending`].
    pub fn ex_3_12() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "3.12" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_12() {
    let _ = ex_3_12::ex_3_12();
}
