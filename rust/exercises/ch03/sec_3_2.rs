// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.2.

mod ex_3_02 {

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.2`.
        pub exercise: &'static str,
    }

    /// Exercise 3.2: a monitored procedure counts and resets
    ///
    /// Returns the result of the monitored square root at 100, the call
    /// count queried right after, and the count after a reset.
    pub fn ex_3_02() -> Result<(f64, u64, u64), Pending> {
        Err(Pending { exercise: "3.2" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_02() {
    let (root, count, after_reset) = ex_3_02::ex_3_02().expect("solved");
    assert!((root - 10.0).abs() < 1e-9);
    assert_eq!((count, after_reset), (1, 0));
}
