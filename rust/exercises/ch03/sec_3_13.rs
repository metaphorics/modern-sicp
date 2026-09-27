// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.13: make-cycle closes the chain back on itself.

mod ex_3_13 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.13`.
        pub exercise: &'static str,
    }

    /// Exercise 3.13: make-cycle closes the chain back on itself
    ///
    /// The solved entry point returns the exercise's answers as
    /// `(bool)`; the pending body reports [`Pending`].
    pub fn ex_3_13() -> Result<bool, Pending> {
        Err(Pending { exercise: "3.13" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_13() {
    let _ = ex_3_13::ex_3_13();
}
