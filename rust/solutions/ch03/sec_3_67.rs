// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.67: the stream of all pairs of
//! the integers, `(i, j)` and `(j, i)` alike, without the `i <= j`
//! condition. The head pair is consed on; then the first row, the first
//! column, and the recursive web of both tails are mixed with two
//! interleaves, so the extra stream the hint asks for is the transposed
//! first column. A 2000-pair prefix is walked and the position of each
//! requested pair is measured: the row `(1, j)` marches every other
//! position (`2j - 3`), the column `(j, 1)` every fourth (`4j - 6`),
//! and the farthest request, `(9, 1)`, lands at 30.

use ch03::sec_3_5::{Stream, cons_stream, interleave, stream_map};

/// The length of the measured prefix; far past the 30th position, where
/// the farthest requested pair lands.
const PREFIX: usize = 2_000;

/// The web of pairs of 3.67: the head pair, then the first row
/// `(s0, t1), (s0, t2), ...` interleaved with the first column
/// `(s1, t0), (s2, t0), ...` interleaved with the recursive web of both
/// tails.
fn pairs_web(s: &Stream<i128>, t: &Stream<i128>) -> Stream<(i128, i128)> {
    let head_pair = (*s.head(), *t.head());
    let first = *s.head();
    let second = *t.head();
    let row = stream_map(move |x: &i128| (first, *x), &t.tail());
    let column = stream_map(move |x: &i128| (*x, second), &s.tail());
    let front = s.clone();
    let back = t.clone();
    cons_stream(head_pair, move || {
        interleave(
            &row,
            &interleave(&column, &pairs_web(&front.tail(), &back.tail())),
        )
    })
}

/// The position of `needle` in the recorded prefix, if the prefix
/// reached it.
#[must_use]
fn position_in(prefix: &[(i128, i128)], needle: (i128, i128)) -> Option<usize> {
    prefix.iter().position(|&pair| pair == needle)
}

/// The pairs whose presence answers the statement: the first row's
/// `(1, j)` and the first column's `(j, 1)` for `j = 2..=9`, then the
/// diagonal pair `(2, 2)`.
#[must_use]
fn requested_pairs() -> Vec<(i128, i128)> {
    let mut requested: Vec<(i128, i128)> = (2..=9).map(|j| (1, j)).collect();
    requested.extend((2..=9).map(|j| (j, 1)));
    requested.push((2, 2));
    requested
}

mod ex_3_67 {
    use super::{PREFIX, pairs_web, position_in, requested_pairs};
    use ch03::sec_3_5::integers;

    /// Exercise 3.67: all pairs via extra interleave
    ///
    /// Answers the measured stream position of each requested pair of
    /// the web of all integer pairs: a list of `((i, j), position)`
    /// proving both orders appear, `(1, j)` and `(j, 1)` for
    /// `j = 2..=9`, plus `(2, 2)`.
    #[must_use]
    pub fn ex_3_67() -> Vec<((i128, i128), usize)> {
        let prefix: Vec<(i128, i128)> = pairs_web(&integers(), &integers())
            .iter()
            .take(PREFIX)
            .collect();
        requested_pairs()
            .into_iter()
            .filter_map(|needle| position_in(&prefix, needle).map(|position| (needle, position)))
            .collect()
    }
}

#[test]
fn ex_3_67() {
    let measured = ex_3_67::ex_3_67();
    // Both orders appear, and the diagonal too. The row keeps the
    // 3.66 stride: (1, j) at 2j - 3. The column is the second strand of
    // a second interleave, so (j, 1) lands at 4j - 6: (2, 1) at 2,
    // (3, 1) at 6, ... (9, 1) at 30. (2, 2) rides the web's own head
    // strand at 4.
    let mut expected: Vec<((i128, i128), i128)> = (2..=9).map(|j| ((1, j), 2 * j - 3)).collect();
    expected.extend((2..=9).map(|j| ((j, 1), 4 * j - 6)));
    expected.push(((2, 2), 4));
    let measured_positions: Vec<((i128, i128), i128)> = measured
        .iter()
        .map(|&(pair, position)| {
            let widened = i128::try_from(position).expect("measured position is bounded by PREFIX");
            (pair, widened)
        })
        .collect();
    assert_eq!(measured_positions, expected);
    // Completeness: all seventeen requests were found, and the farthest
    // position any of them needed is 30 -- (9, 1).
    assert_eq!(measured.len(), 17);
    let farthest = measured.iter().map(|(_, position)| *position).max();
    assert_eq!(farthest, Some(30));
}
