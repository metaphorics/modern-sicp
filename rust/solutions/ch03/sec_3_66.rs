// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.66: the order in which the
//! book's `pairs` places the pairs of `(pairs integers integers)`. The
//! stream is walked over a 4000-pair prefix and the position of each
//! requested pair is measured. The measurements state the general law
//! exactly: the diagonal `(k, k)` sits at position `2^k - 2`, and a
//! strict upper-triangle pair `(i, j)` sits at position
//! `2^i (j - i) + 2^(i - 1) - 2` -- the first row marches linearly
//! (`(1, j)` at `2j - 3`, so `(1, 100)` needs only 197 pairs), while
//! each row's elements are spread `2^i` apart, doubling per row. The
//! test checks the law against every one of the 4000 measured pairs.

use ch03::sec_3_5::{integers, pairs};

/// The length of the measured prefix; large enough to contain every
/// pair the exercise names with room to verify the general law.
const PREFIX: usize = 4_000;

/// The position of `needle` in the recorded prefix, if the prefix
/// reached it.
#[must_use]
fn position_in(prefix: &[(i128, i128)], needle: (i128, i128)) -> Option<usize> {
    prefix.iter().position(|&pair| pair == needle)
}

/// The pairs whose measured positions answer the statement, in the
/// order the statement names them: the first row's `(1, j)` for
/// `j = 2..=8`, row 2's `(2, j)` for `j = 3..=6`, then `(3, 3)` and
/// `(3, 4)`.
#[must_use]
fn requested_pairs() -> Vec<(i128, i128)> {
    let mut requested: Vec<(i128, i128)> = (2..=8).map(|j| (1, j)).collect();
    requested.extend((3..=6).map(|j| (2, j)));
    requested.push((3, 3));
    requested.push((3, 4));
    requested
}

/// The measured prefix of `(pairs integers integers)`.
#[must_use]
fn measured_prefix() -> Vec<(i128, i128)> {
    pairs(&integers(), &integers())
        .iter()
        .take(PREFIX)
        .collect()
}

mod ex_3_66 {
    use super::{measured_prefix, position_in, requested_pairs};

    /// Exercise 3.66: pairs ordering analysis
    ///
    /// Answers the measured stream position of each requested pair of
    /// `(pairs integers integers)`: a list of `((i, j), position)`.
    /// Every requested pair lies well inside the measured prefix -- the
    /// farthest, `(2, 6)`, sits at position 16 -- so the list is
    /// complete; the test pins its exact content.
    #[must_use]
    pub fn ex_3_66() -> Vec<((i128, i128), usize)> {
        let prefix = measured_prefix();
        requested_pairs()
            .into_iter()
            .filter_map(|needle| position_in(&prefix, needle).map(|position| (needle, position)))
            .collect()
    }
}

#[test]
fn ex_3_66() {
    let measured = ex_3_66::ex_3_66();
    let measured_positions: Vec<((i128, i128), i128)> = measured
        .iter()
        .map(|&(pair, position)| {
            let widened = i128::try_from(position).expect("measured position is bounded by PREFIX");
            (pair, widened)
        })
        .collect();
    // The first row marches linearly: (1, j) at 2j - 3, one row
    // element every other position -- so (1, 100) at 197, not the
    // astronomically far position a doubling rule would suggest.
    let mut expected: Vec<((i128, i128), i128)> = (2..=8).map(|j| ((1, j), 2 * j - 3)).collect();
    // Row 2 is spread four apart: (2, j) at 4j - 8.
    expected.extend((3..=6).map(|j| ((2, j), 4 * j - 8)));
    // The diagonal doubles: (3, 3) at 2^3 - 2 = 6, and (3, 4), the
    // first off-diagonal of row 3, at 2^3 + 4 - 2 = 10.
    expected.push(((3, 3), 6));
    expected.push(((3, 4), 10));
    assert_eq!(measured_positions, expected);
    // The general law, checked against every measured pair: the
    // diagonal (k, k) at 2^k - 2, and (i, j) for j > i at
    // 2^i (j - i) + 2^(i - 1) - 2. Each row's stride doubles with i,
    // which is why (99, 100) sits near 3 x 2^98 - 2 and (100, 100) at
    // 2^100 - 2 -- effectively unreachable -- while (1, 100) is at 197.
    for (position, (i, j)) in measured_prefix().into_iter().enumerate() {
        let position = position as i128;
        let law = if j == i {
            (1_i128 << i) - 2
        } else {
            (1_i128 << i) * (j - i) + (1_i128 << (i - 1)) - 2
        };
        assert_eq!(position, law, "law broke at ({i}, {j})");
    }
    // Completeness of the answer list, pinned: all thirteen requests
    // were found inside the prefix.
    assert_eq!(measured.len(), 13);
}
