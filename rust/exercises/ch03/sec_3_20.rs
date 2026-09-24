// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.20: trace aliasing through the procedural pair.

mod ex_3_20 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.20`.
        pub exercise: &'static str,
    }

    /// Exercise 3.20: trace aliasing through the procedural pair
    ///
    /// The solved entry point returns the exercise's answers as
    /// `(i128, i128)`; the pending body reports [`Pending`].
    pub fn ex_3_20() -> Result<(i128, i128), Pending> {
        Err(Pending { exercise: "3.20" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_20() {
    let _ = ex_3_20::ex_3_20();
}
