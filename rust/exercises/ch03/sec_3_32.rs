// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.32: agenda segments must run actions first in, first out.

mod ex_3_32 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.32`.
        pub exercise: &'static str,
    }

    /// Exercise 3.32: agenda segments must run actions first in, first out
    ///
    /// The solved entry point returns the exercise's answers as
    /// `(u8)`; the pending body reports [`Pending`].
    pub fn ex_3_32() -> Result<u8, Pending> {
        Err(Pending { exercise: "3.32" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_32() {
    let _ = ex_3_32::ex_3_32();
}
