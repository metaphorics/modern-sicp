// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.5, one module and one ignored test.

mod ex_2_05 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.5`.
        pub exercise: &'static str,
    }

    /// Exercise 2.5: pairs of nonnegative integers as `2^a * 3^b`
    ///
    /// Returns the two exponents of the pair `(3, 2)` recovered from
    /// its `2^a * 3^b` encoding, using the pair's first/second selectors.
    pub fn ex_2_05() -> Result<(u32, u32), Pending> {
        Err(Pending { exercise: "2.5" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_05() {
    assert_eq!(ex_2_05::ex_2_05(), Ok((3, 2)));
}
