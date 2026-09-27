// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.40: every value a concurrent
//! squaring and cubing can leave in `x`, and what serialization keeps.

mod ex_3_40 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.40`.
        pub exercise: &'static str,
    }

    /// Exercise 3.40: all values of concurrent x squared
    ///
    /// Answers the sorted possible values of the unserialized race and
    /// the values serialization leaves.
    pub fn ex_3_40() -> Result<(Vec<i128>, Vec<i128>), Pending> {
        Err(Pending { exercise: "3.40" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_40() {
    assert_eq!(
        ex_3_40::ex_3_40(),
        Ok((vec![100, 1000, 10_000, 100_000, 1_000_000], vec![1_000_000]))
    );
}
