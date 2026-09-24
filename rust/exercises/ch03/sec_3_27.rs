// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.27: memoized fib turns exponential into linear.

mod ex_3_27 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.27`.
        pub exercise: &'static str,
    }

    /// Exercise 3.27: memoized fib turns exponential into linear
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((i128, u64, u64, u64))`; the pending body reports [`Pending`].
    pub fn ex_3_27() -> Result<(i128, u64, u64, u64), Pending> {
        Err(Pending { exercise: "3.27" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_27() {
    let _ = ex_3_27::ex_3_27();
}
