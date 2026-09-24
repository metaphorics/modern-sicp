// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.23: a deque with constant-time operations at both ends.

mod ex_3_23 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.23`.
        pub exercise: &'static str,
    }

    /// Exercise 3.23: a deque with constant-time operations at both ends
    ///
    /// The solved entry point returns the exercise's answers as
    /// `(Vec<String>)`; the pending body reports [`Pending`].
    pub fn ex_3_23() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "3.23" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_23() {
    let _ = ex_3_23::ex_3_23();
}
