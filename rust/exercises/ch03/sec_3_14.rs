// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.14: mystery turns a chain around in place.

mod ex_3_14 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.14`.
        pub exercise: &'static str,
    }

    /// Exercise 3.14: mystery turns a chain around in place
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((String, String))`; the pending body reports [`Pending`].
    pub fn ex_3_14() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "3.14" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_14() {
    let _ = ex_3_14::ex_3_14();
}
