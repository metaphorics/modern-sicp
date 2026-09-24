// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.35: a squarer as a new primitive constraint.

mod ex_3_35 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.35`.
        pub exercise: &'static str,
    }

    /// Exercise 3.35: a squarer as a new primitive constraint
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((Option<f64>, Option<f64>, bool))`; the pending body reports [`Pending`].
    pub fn ex_3_35() -> Result<(Option<f64>, Option<f64>, bool), Pending> {
        Err(Pending { exercise: "3.35" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_35() {
    let _ = ex_3_35::ex_3_35();
}
