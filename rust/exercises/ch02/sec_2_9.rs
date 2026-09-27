// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.9, one module and one ignored test.

mod ex_2_09 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.9`.
        pub exercise: &'static str,
    }

    /// Exercise 2.9: the width of a sum is a function of the widths of
    /// the addends, but the width of a product is not
    ///
    /// Returns the width of `[2, 6] + [10, 14]`, the sum of the two
    /// addends' widths, and whether `[2, 6] * [10, 14]` and
    /// `[2, 6] * [100, 104]` (same factor, two same-width second
    /// factors with different centers) have the same width.
    pub fn ex_2_09() -> Result<(f64, f64, bool), Pending> {
        Err(Pending { exercise: "2.9" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_09() {
    assert_eq!(ex_2_09::ex_2_09(), Ok((4.0, 4.0, false)));
}
