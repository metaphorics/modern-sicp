// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.50: `stream-map` generalized to
//! procedures over any number of streams, answered by the prefixes the
//! solution pins.

mod ex_3_50 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.50`.
        pub exercise: &'static str,
    }

    /// Exercise 3.50: multi-stream map completion
    ///
    /// Answers three pinned prefixes of the general map: the elementwise
    /// sum of two `integers` streams, the product of `integers`, `ones`,
    /// and `integers`, and the sum of `integers` with a three-element
    /// finite stream, which empties the result after three elements.
    #[expect(
        clippy::type_complexity,
        reason = "the pending report mirrors the solution's answer type"
    )]
    pub fn ex_3_50() -> Result<(Vec<i128>, Vec<i128>, Vec<i128>), Pending> {
        Err(Pending { exercise: "3.50" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_50() {
    let (two_summed, three_multiplied, short_empties) = ex_3_50::ex_3_50().expect("solved");
    // (integers + integers)[k] = 2(k + 1): the general map must agree
    // with the two-stream `add-streams` over the prefix.
    assert_eq!(two_summed, vec![2, 4, 6, 8, 10, 12]);
    // integers * ones * integers = n * 1 * n: one head call sees all
    // three heads at once.
    assert_eq!(three_multiplied, vec![1, 4, 9, 16, 25]);
    // One empty input empties the whole map: the finite stream runs out
    // at three elements, so only three sums come out.
    assert_eq!(short_empties, vec![2, 4, 6]);
}
