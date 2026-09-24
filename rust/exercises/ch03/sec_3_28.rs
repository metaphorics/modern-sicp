// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.28: an or-gate as a primitive function box.

mod ex_3_28 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.28`.
        pub exercise: &'static str,
    }

    /// Exercise 3.28: an or-gate as a primitive function box
    ///
    /// The solved entry point returns the exercise's answers as
    /// `(u8)`; the pending body reports [`Pending`].
    pub fn ex_3_28() -> Result<u8, Pending> {
        Err(Pending { exercise: "3.28" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_28() {
    let _ = ex_3_28::ex_3_28();
}
