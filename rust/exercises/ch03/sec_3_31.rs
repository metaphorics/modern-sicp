// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.31: why attaching runs each action once immediately.

mod ex_3_31 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.31`.
        pub exercise: &'static str,
    }

    /// Exercise 3.31: why attaching runs each action once immediately
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((bool, bool))`; the pending body reports [`Pending`].
    pub fn ex_3_31() -> Result<(bool, bool), Pending> {
        Err(Pending { exercise: "3.31" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_31() {
    let _ = ex_3_31::ex_3_31();
}
