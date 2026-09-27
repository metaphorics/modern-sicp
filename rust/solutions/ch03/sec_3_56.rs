// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.56: the Hamming (regular)
//! numbers as one self-referential stream. `S` begins with 1, and the
//! rest is the ordered merge of the stream's own multiples by 2, 3, and
//! 5: every element after 1 is an earlier element times 2, 3, or 5, and
//! every such product appears. The merge's equal-head branch advances
//! both sides at once, which is what collapses the duplicates -- 12 is
//! both 2 x 6 and 2^2 x 3 and is emitted once.

use ch03::sec_3_5::{Stream, cons_stream, merge, scale_stream, self_stream};

/// The book's exercise stream `S`: 1 consed onto the merge of the
/// stream's own scalings, `(merge (scale-stream S 2) (merge
/// (scale-stream S 3) (scale-stream S 5)))`. Each scaling reads the one
/// shared memoized spine, so no Hamming number is ever computed twice.
fn hammings() -> Stream<i128> {
    self_stream(|s| {
        let twos = s.clone();
        let threes = s.clone();
        let fives = s.clone();
        cons_stream(1, move || {
            let twos_tail = scale_stream(&twos.stream(), 2);
            let threes_and_fives = merge(
                &scale_stream(&threes.stream(), 3),
                &scale_stream(&fives.stream(), 5),
            );
            merge(&twos_tail, &threes_and_fives)
        })
    })
}

mod ex_3_56 {
    use super::hammings;

    /// Exercise 3.56: Hamming numbers via merge
    ///
    /// Answers the first 15 elements of the ascending, repetition-free
    /// stream of positive integers whose only prime factors are 2, 3,
    /// and 5.
    #[must_use]
    pub fn ex_3_56() -> Vec<i128> {
        hammings().iter().take(15).collect()
    }
}

#[test]
fn ex_3_56() {
    // 1 heads the stream; after it, the smallest not-yet-emitted product
    // is always at some merge head, so the stream is the sorted list of
    // 2^a 3^b 5^c products: 15 = 3 x 5 lands after 12 = 2^2 x 3 and
    // before 16 = 2^4.
    assert_eq!(
        ex_3_56::ex_3_56(),
        [1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24]
    );
    // The equal-head branch of `merge` drops each duplicate once, so the
    // stream never repeats or descends: strict increase over the first
    // 200 elements is the no-repetitions property of the construction.
    let walked: Vec<i128> = hammings().iter().take(200).collect();
    assert!(walked.windows(2).all(|pair| pair[0] < pair[1]));
}
