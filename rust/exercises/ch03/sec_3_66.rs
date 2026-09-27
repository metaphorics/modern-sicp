// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.66: measuring at which stream
//! positions the book's `pairs` places the requested pairs of
//! `(pairs integers integers)`, and stating the general law.

mod ex_3_66 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.66`.
        pub exercise: &'static str,
    }

    /// Exercise 3.66: pairs ordering analysis
    ///
    /// Answers the measured stream position of each requested pair of
    /// `(pairs integers integers)`: a list of `((i, j), position)` for
    /// the first row's `(1, j)`, `j = 2..=8`; row 2's `(2, j)`,
    /// `j = 3..=6`; then `(3, 3)` and `(3, 4)`.
    #[expect(
        clippy::type_complexity,
        reason = "the pending report mirrors the solution's answer type"
    )]
    pub fn ex_3_66() -> Result<Vec<((i128, i128), usize)>, Pending> {
        Err(Pending { exercise: "3.66" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_66() {
    let measured = ex_3_66::ex_3_66().expect("solved");
    // The first row marches linearly, (1, j) at 2j - 3; row 2 is
    // spread four apart, (2, j) at 4j - 8; the diagonal doubles,
    // (k, k) at 2^k - 2. The general law for j > i is
    // 2^i (j - i) + 2^(i - 1) - 2.
    let expected: Vec<((i128, i128), usize)> = vec![
        ((1, 2), 1),
        ((1, 3), 3),
        ((1, 4), 5),
        ((1, 5), 7),
        ((1, 6), 9),
        ((1, 7), 11),
        ((1, 8), 13),
        ((2, 3), 4),
        ((2, 4), 8),
        ((2, 5), 12),
        ((2, 6), 16),
        ((3, 3), 6),
        ((3, 4), 10),
    ];
    assert_eq!(measured, expected);
}
