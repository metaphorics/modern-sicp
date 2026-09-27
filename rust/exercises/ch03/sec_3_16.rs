// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.16: count-pairs counts shared pairs more than once.

mod ex_3_16 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.16`.
        pub exercise: &'static str,
    }

    /// Exercise 3.16: count-pairs counts shared pairs more than once
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((u64, u64, u64, bool))`; the pending body reports [`Pending`].
    pub fn ex_3_16() -> Result<(u64, u64, u64, bool), Pending> {
        Err(Pending { exercise: "3.16" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_16() {
    let _ = ex_3_16::ex_3_16();
}
