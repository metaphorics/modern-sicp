// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.67: the stream of all integer
//! pairs, both orders, built by mixing the first row and the first
//! column with two interleaves, and the measured positions proving
//! every requested pair appears.

mod ex_3_67 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.67`.
        pub exercise: &'static str,
    }

    /// Exercise 3.67: all pairs via extra interleave
    ///
    /// Answers the measured stream position of each requested pair of
    /// the web of all integer pairs: a list of `((i, j), position)`
    /// proving both orders appear, `(1, j)` and `(j, 1)` for
    /// `j = 2..=9`, plus `(2, 2)`.
    #[expect(
        clippy::type_complexity,
        reason = "the pending report mirrors the solution's answer type"
    )]
    pub fn ex_3_67() -> Result<Vec<((i128, i128), usize)>, Pending> {
        Err(Pending { exercise: "3.67" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_67() {
    let measured = ex_3_67::ex_3_67().expect("solved");
    // The row keeps the 3.66 stride, (1, j) at 2j - 3; the column is
    // the second strand of the second interleave, (j, 1) at 4j - 6;
    // the diagonal pair (2, 2) rides the web's own head strand at 4.
    // The farthest position any request needs is 30, for (9, 1).
    let expected: Vec<((i128, i128), usize)> = vec![
        ((1, 2), 1),
        ((1, 3), 3),
        ((1, 4), 5),
        ((1, 5), 7),
        ((1, 6), 9),
        ((1, 7), 11),
        ((1, 8), 13),
        ((1, 9), 15),
        ((2, 1), 2),
        ((3, 1), 6),
        ((4, 1), 10),
        ((5, 1), 14),
        ((6, 1), 18),
        ((7, 1), 22),
        ((8, 1), 26),
        ((9, 1), 30),
        ((2, 2), 4),
    ];
    assert_eq!(measured, expected);
}
