// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.70: `merge_weighted` and
//! `weighted_pairs`, with the sum-ordered, `2i + 3j + 5ij`-ordered, and
//! sum-divisibility-filtered prefixes the statement demands.

mod ex_3_70 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.70`.
        pub exercise: &'static str,
    }

    /// The pinned prefixes of exercise 3.70, all taken from real runs of
    /// the weighted machinery.
    #[derive(Debug, Clone, PartialEq)]
    pub struct WeightedPairsReport {
        /// The first 10 pairs of part (a), ordered by the sum `i + j`.
        pub sum_ordered: Vec<(i128, i128)>,
        /// The first 8 pairs of part (b), ordered by `2i + 3j + 5ij`.
        pub weighted_ordered: Vec<(i128, i128)>,
        /// The first 8 `(i, j, i + j)` triples whose sum no 2, 3, or 5
        /// divides, the filtered form of part (a)'s stream.
        pub sum_filtered: Vec<(i128, i128, i128)>,
    }

    /// Exercise 3.70: merge-weighted, weighted-pairs
    ///
    /// Answers the three pinned prefixes: the sum-ordered pairs, the
    /// `2i + 3j + 5ij`-ordered pairs over 2, 3, 5-coprime components,
    /// and the sum-divisibility-filtered triples.
    pub fn ex_3_70() -> Result<WeightedPairsReport, Pending> {
        Err(Pending { exercise: "3.70" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_70() {
    let report = ex_3_70::ex_3_70().expect("solved");
    // Part (a): each tie at a sum holds the row (1, x) side first.
    assert_eq!(
        report.sum_ordered,
        vec![
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 2),
            (1, 4),
            (2, 3),
            (1, 5),
            (2, 4),
            (3, 3),
            (1, 6),
        ]
    );
    // Part (b): the row weights 10, 58, 90, 106, 138, 152, 180, 222
    // surface before the (7, 7) weight 280 can.
    assert_eq!(
        report.weighted_ordered,
        vec![
            (1, 1),
            (1, 7),
            (1, 11),
            (1, 13),
            (1, 17),
            (1, 19),
            (1, 23),
            (1, 29)
        ]
    );
    // Part (c): sum 7 is the first survivor, then 11.
    assert_eq!(
        report.sum_filtered,
        vec![
            (1, 6, 7),
            (2, 5, 7),
            (3, 4, 7),
            (1, 10, 11),
            (2, 9, 11),
            (3, 8, 11),
            (4, 7, 11),
            (5, 6, 11),
        ]
    );
}
