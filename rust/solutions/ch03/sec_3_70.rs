// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The solution of exercise 3.70: `merge_weighted` is the section's
//! `merge` re-ordered by a weighting function -- the cheaper side emits,
//! and on a weight tie both sides emit, the first stream first, since
//! the merged sides are disjoint. `weighted_pairs` is the statement's
//! generalization of `pairs`: the head pair `(s0, t0)`, then
//! `merge_weighted` of the row `(s0, x)` over the tail of `t` with
//! `weighted_pairs` over the two tails, which keeps every pair reachable
//! and emits the pairs in nondecreasing weight order. The report pins
//! part (a), the sum-ordered pairs; part (b), the `2i + 3j + 5ij`-ordered
//! pairs over components no 2, 3, or 5 divides; and the sum-ordered
//! stream filtered to sums no 2, 3, or 5 divides.

use std::cmp::Ordering;

use ch03::sec_3_5::{Stream, cons_stream, integers, stream_filter, stream_map};

/// The exercise-local `merge_weighted`: the ordered union of two streams
/// ordered by `weight`, like the book's `merge` but comparing weights.
/// A tie keeps BOTH elements -- the sides of `weighted_pairs` are
/// disjoint, so no pair is doubled -- the first stream's element first.
fn merge_weighted<A: Clone + 'static, W>(s1: &Stream<A>, s2: &Stream<A>, weight: W) -> Stream<A>
where
    W: Fn(&A) -> i128 + Clone + 'static,
{
    if s1.is_empty() {
        return s2.clone();
    }
    if s2.is_empty() {
        return s1.clone();
    }
    let x1 = s1.head().clone();
    let x2 = s2.head().clone();
    let front = s1.clone();
    let back = s2.clone();
    match (weight(&x1)).cmp(&weight(&x2)) {
        Ordering::Less => cons_stream(x1, move || merge_weighted(&front.tail(), &back, weight)),
        Ordering::Greater => cons_stream(x2, move || merge_weighted(&front, &back.tail(), weight)),
        Ordering::Equal => {
            let tied = x2;
            cons_stream(x1, move || {
                cons_stream(tied, move || {
                    merge_weighted(&front.tail(), &back.tail(), weight)
                })
            })
        }
    }
}

/// The exercise-local `weighted_pairs`: the pairs `(i, j)` with `i` from
/// `s` and `j` from `t`, ordered by `weight` per the statement. The head
/// pair, then the weighted merge of the first stream's row with the
/// recursion over the tails; the statement's footnote demand -- weight
/// increasing along a row and down a column -- is on the caller's
/// weighting function.
fn weighted_pairs<W>(s: &Stream<i128>, t: &Stream<i128>, weight: W) -> Stream<(i128, i128)>
where
    W: Fn(&(i128, i128)) -> i128 + Clone + 'static,
{
    let head = (*s.head(), *t.head());
    let row = {
        let first = *s.head();
        stream_map(move |x: &i128| (first, *x), &t.tail())
    };
    let front = s.clone();
    let back = t.clone();
    cons_stream(head, move || {
        let rest = weighted_pairs(&front.tail(), &back.tail(), weight.clone());
        merge_weighted(&row, &rest, weight)
    })
}

/// The statement's part (a): all pairs `(i, j)` with `i <= j`, ordered by
/// the sum `i + j` (the diagonal pairing of one `integers` with itself).
#[must_use]
fn sum_pairs() -> Stream<(i128, i128)> {
    weighted_pairs(&integers(), &integers(), |pair| pair.0 + pair.1)
}

/// The statement's part (b): pairs whose components no 2, 3, or 5
/// divides, ordered by the weight `2i + 3j + 5ij`.
#[must_use]
fn coprime_pairs() -> Stream<(i128, i128)> {
    let source = stream_filter(
        |n: &i128| n % 2 != 0 && n % 3 != 0 && n % 5 != 0,
        &integers(),
    );
    weighted_pairs(&source.clone(), &source, |pair| {
        2 * pair.0 + 3 * pair.1 + 5 * pair.0 * pair.1
    })
}

/// The sum-ordered pairs whose sum neither 2, 3, nor 5 divides, as the
/// `(i, j, i + j)` triples the sum order motivates.
#[must_use]
fn sum_filtered() -> Stream<(i128, i128, i128)> {
    let source = sum_pairs();
    stream_map(
        |pair: &(i128, i128)| (pair.0, pair.1, pair.0 + pair.1),
        &stream_filter(
            |pair: &(i128, i128)| {
                let sum = pair.0 + pair.1;
                sum % 2 != 0 && sum % 3 != 0 && sum % 5 != 0
            },
            &source,
        ),
    )
}

/// The pinned prefixes of exercise 3.70, all taken from real runs of the
/// weighted machinery above.
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

mod ex_3_70 {
    use super::{WeightedPairsReport, coprime_pairs, sum_filtered, sum_pairs};

    /// Exercise 3.70: merge-weighted, weighted-pairs
    ///
    /// Answers the three pinned prefixes: the sum-ordered pairs, the
    /// `2i + 3j + 5ij`-ordered pairs over 2, 3, 5-coprime components,
    /// and the sum-divisibility-filtered triples.
    #[must_use]
    pub fn ex_3_70() -> WeightedPairsReport {
        WeightedPairsReport {
            sum_ordered: sum_pairs().iter().take(10).collect(),
            weighted_ordered: coprime_pairs().iter().take(8).collect(),
            sum_filtered: sum_filtered().iter().take(8).collect(),
        }
    }
}

#[test]
fn ex_3_70() {
    // Part (a): nondecreasing sums; each tie at a sum holds one pair per
    // side of the merge, the row (1, x) side emitted first, so weight 4
    // is (1, 3) then (2, 2) and weight 6 is (1, 5), (2, 4), (3, 3).
    assert_eq!(
        ex_3_70::ex_3_70().sum_ordered,
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
    // Part (b): the components 1, 7, 11, 13, 17, ... carry the row
    // weights 10, 58, 90, 106, 138, 152, 180, 222 before the (7, 7)
    // weight 280 can surface; the recursion stays under the row.
    assert_eq!(
        ex_3_70::ex_3_70().weighted_ordered,
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
    // Part (c): sums 2..6 all carry a factor 2, 3, or 5, so 7 is the
    // first surviving sum, with its three pairs in merge order; the next
    // survivors are 11 and then 13.
    assert_eq!(
        ex_3_70::ex_3_70().sum_filtered,
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
