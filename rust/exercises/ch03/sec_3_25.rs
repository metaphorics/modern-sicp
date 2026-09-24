// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.25: a table keyed by lists of arbitrary length.

mod ex_3_25 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.25`.
        pub exercise: &'static str,
    }

    /// Exercise 3.25: a table keyed by lists of arbitrary length
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((Option<i128>, Option<i128>, Option<i128>))`; the pending body reports [`Pending`].
    #[allow(
        clippy::type_complexity,
        reason = "three parallel Option<i128> lookups, not a structural nesting problem"
    )]
    pub fn ex_3_25() -> Result<(Option<i128>, Option<i128>, Option<i128>), Pending> {
        Err(Pending { exercise: "3.25" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_25() {
    let _ = ex_3_25::ex_3_25();
}
