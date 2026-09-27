// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.17: count distinct pairs once each with a history.

mod ex_3_17 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.17`.
        pub exercise: &'static str,
    }

    /// Exercise 3.17: count distinct pairs once each with a history
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((u64, u64, u64))`; the pending body reports [`Pending`].
    pub fn ex_3_17() -> Result<(u64, u64, u64), Pending> {
        Err(Pending { exercise: "3.17" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_17() {
    let _ = ex_3_17::ex_3_17();
}
