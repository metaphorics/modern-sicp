// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.34: Louis's squarer from a bare multiplier is flawed.

mod ex_3_34 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.34`.
        pub exercise: &'static str,
    }

    /// Exercise 3.34: Louis's squarer from a bare multiplier is flawed
    ///
    /// The solved entry point returns the exercise's answers as
    /// `(Option<f64>)`; the pending body reports [`Pending`].
    pub fn ex_3_34() -> Result<Option<f64>, Pending> {
        Err(Pending { exercise: "3.34" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_34() {
    let _ = ex_3_34::ex_3_34();
}
