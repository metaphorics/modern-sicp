// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.30: a ripple-carry adder over n full-adders.

mod ex_3_30 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.30`.
        pub exercise: &'static str,
    }

    /// Exercise 3.30: a ripple-carry adder over n full-adders
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((u8, u8))`; the pending body reports [`Pending`].
    pub fn ex_3_30() -> Result<(u8, u8), Pending> {
        Err(Pending { exercise: "3.30" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_30() {
    let _ = ex_3_30::ex_3_30();
}
