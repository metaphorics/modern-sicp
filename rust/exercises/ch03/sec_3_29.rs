// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.29: an or-gate built from an and-gate and inverters.

mod ex_3_29 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.29`.
        pub exercise: &'static str,
    }

    /// Exercise 3.29: an or-gate built from an and-gate and inverters
    ///
    /// The solved entry point returns the exercise's answers as
    /// `(u64)`; the pending body reports [`Pending`].
    pub fn ex_3_29() -> Result<u64, Pending> {
        Err(Pending { exercise: "3.29" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_29() {
    let _ = ex_3_29::ex_3_29();
}
