// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.69: `triples` over three infinite
//! streams through the section's `pairs` and `interleave`, filtered to
//! the Pythagorean triples.

mod ex_3_69 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.69`.
        pub exercise: &'static str,
    }

    /// Exercise 3.69: triples and the Pythagorean stream
    ///
    /// Answers the first six Pythagorean triples of positive integers in
    /// the order the interleaved `triples` stream itself reaches them.
    pub fn ex_3_69() -> Result<Vec<(i128, i128, i128)>, Pending> {
        Err(Pending { exercise: "3.69" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_69() {
    let triples = ex_3_69::ex_3_69().expect("solved");
    // The stream's own ordering, not a sorted one: the decomposition
    // reaches (6, 8, 10) through the first-coordinate branches before it
    // walks deep enough inside an earlier branch to find (5, 12, 13).
    assert_eq!(
        triples,
        vec![
            (3, 4, 5),
            (6, 8, 10),
            (5, 12, 13),
            (9, 12, 15),
            (8, 15, 17),
            (12, 16, 20),
        ]
    );
}
