// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.37: expression-style constraint combinators.

mod ex_3_37 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.37`.
        pub exercise: &'static str,
    }

    /// Exercise 3.37: expression-style constraint combinators
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((f64, f64))`; the pending body reports [`Pending`].
    pub fn ex_3_37() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "3.37" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_37() {
    let _ = ex_3_37::ex_3_37();
}
