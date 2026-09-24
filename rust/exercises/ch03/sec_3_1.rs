// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.1.

mod ex_3_01 {

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.1`.
        pub exercise: &'static str,
    }

    /// Exercise 3.1: an accumulator keeps a running sum
    ///
    /// Returns the running sums after the first and the second call of
    /// an accumulator started at 5 and called with 10 each time.
    pub fn ex_3_01() -> Result<(i128, i128), Pending> {
        Err(Pending { exercise: "3.1" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_01() {
    assert_eq!(ex_3_01::ex_3_01(), Ok((15, 25)));
}
