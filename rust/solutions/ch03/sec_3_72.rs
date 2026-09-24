// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The solution of exercise 3.72: numbers written as a sum of two
//! squares in three different ways, by the same ordered-pairs route as
//! exercise 3.71. Pairs `(i, j)` of integers ordered by the weight
//! `i*i + j*j` emit equal weights adjacently; the local grouping
//! combinator folds the ordered stream into runs of equal weight, and
//! the runs of length three or more are the numbers of the statement,
//! each carrying the three witness pairs that show how it is so written.

use std::cmp::Ordering;

use ch03::sec_3_5::{Stream, cons_stream, integers, stream_map};

/// The exercise-local `merge_weighted`: the ordered union of two streams
/// ordered by `weight`. A tie keeps BOTH elements -- the sides of
/// `weighted_pairs` are disjoint -- the first stream's element first.
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

/// The exercise-local `weighted_pairs` of exercise 3.70: pairs `(i, j)`
/// ordered by `weight`, built as the head pair plus the weighted merge
/// of the first stream's row with the recursion over the tails.
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

/// One maximal run of equal weight in an ordered pair stream: the
/// weight, how many pairs carry it, and those pairs in emission order.
type WeightRun = (i128, usize, Vec<(i128, i128)>);

/// The exercise-local grouping combinator: folds an ordered pair stream
/// into runs of equal weight, one output element per maximal run, each
/// run walked eagerly because a run is bounded by the number of
/// representations the weight has.
fn weight_runs<W>(s: &Stream<(i128, i128)>, weight: W) -> Stream<WeightRun>
where
    W: Fn(&(i128, i128)) -> i128 + Clone + 'static,
{
    if s.is_empty() {
        return Stream::Empty;
    }
    let head_weight = weight(s.head());
    let mut members = vec![*s.head()];
    let mut cursor = s.tail();
    while !cursor.is_empty() && weight(cursor.head()) == head_weight {
        members.push(*cursor.head());
        cursor = cursor.tail();
    }
    let front = cursor;
    let run = (head_weight, members.len(), members);
    cons_stream(run, move || weight_runs(&front, weight))
}

/// The statement's search: square-weighted ordered pairs, grouped into
/// equal-weight runs, keeping the runs of three or more -- the numbers
/// expressible as a sum of two squares in three different ways, with
/// the pairs that show how.
#[must_use]
fn three_square_runs() -> Vec<WeightRun> {
    let square_weight = |pair: &(i128, i128)| pair.0 * pair.0 + pair.1 * pair.1;
    let weighted = weighted_pairs(&integers(), &integers(), square_weight);
    weight_runs(&weighted, square_weight)
        .iter()
        .filter(|run| run.1 >= 3)
        .take(3)
        .collect()
}

/// The answer shape of the exercise: the qualifying weights, each with
/// the witness pairs of its representations, in emission order.
type WeightedWitnesses = (Vec<i128>, Vec<Vec<(i128, i128)>>);

mod ex_3_72 {
    use super::{WeightedWitnesses, three_square_runs};

    /// Exercise 3.72: sums of two squares thrice
    ///
    /// Answers the first three numbers with three two-square
    /// representations and, for each, the witness pairs that show how.
    #[must_use]
    pub fn ex_3_72() -> WeightedWitnesses {
        let runs = three_square_runs();
        let numbers = runs.iter().map(|run| run.0).collect();
        let witnesses = runs.into_iter().map(|(_, _, members)| members).collect();
        (numbers, witnesses)
    }
}

#[test]
fn ex_3_72() {
    // 325 = 1^2 + 18^2 = 6^2 + 17^2 = 10^2 + 15^2, 425 = 5^2 + 20^2 =
    // 8^2 + 19^2 = 13^2 + 16^2, and 650 = 5^2 + 25^2 = 11^2 + 23^2 =
    // 17^2 + 19^2: the first three weights with three representations.
    // Equal weights emit adjacently, the lower first coordinate first.
    assert_eq!(
        ex_3_72::ex_3_72(),
        (
            vec![325, 425, 650],
            vec![
                vec![(1, 18), (6, 17), (10, 15)],
                vec![(5, 20), (8, 19), (13, 16)],
                vec![(5, 25), (11, 23), (17, 19)],
            ],
        )
    );
}
