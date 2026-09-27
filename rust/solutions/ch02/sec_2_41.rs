// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.41, one module and one test.

mod ex_2_41 {
    /// Exercise 2.41: ordered triples of distinct positive integers up
    /// to `n` that sum to `s`. Three nested range enumerations produce
    /// every ordered triple, the distinctness test and the sum test
    /// filter, and the survivors are the answer; this is 2.40's
    /// `unique-pairs` pattern one dimension deeper.
    fn ordered_triples(n: i64, s: i64) -> Vec<[i64; 3]> {
        (1..=n)
            .flat_map(|i| (1..=n).flat_map(move |j| (1..=n).map(move |k| [i, j, k])))
            .filter(|t| t[0] != t[1] && t[1] != t[2] && t[0] != t[2])
            .filter(|t| t[0] + t[1] + t[2] == s)
            .collect()
    }

    /// Exercise 2.41: ordered triples summing to s
    ///
    /// Returns the number of ordered triples of distinct positive
    /// integers up to `n = 4` that sum to `s = 9`, together with one
    /// such triple. Exactly the numbers 2, 3, and 4 fit, in all six
    /// orders.
    pub fn ex_2_41() -> (usize, [i64; 3]) {
        let triples = ordered_triples(4, 9);
        let first = triples.first().copied().unwrap_or([0, 0, 0]);
        (triples.len(), first)
    }
}

#[test]
fn ex_2_41() {
    assert_eq!(ex_2_41::ex_2_41(), (6, [2, 3, 4]));
}
