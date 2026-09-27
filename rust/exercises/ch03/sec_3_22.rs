// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.22: build the queue as a message-passing closure.

mod ex_3_22 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.22`.
        pub exercise: &'static str,
    }

    /// Exercise 3.22: build the queue as a message-passing closure
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((String, Vec<String>))`; the pending body reports [`Pending`].
    pub fn ex_3_22() -> Result<(String, Vec<String>), Pending> {
        Err(Pending { exercise: "3.22" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_22() {
    let _ = ex_3_22::ex_3_22();
}
