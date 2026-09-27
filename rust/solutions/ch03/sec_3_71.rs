// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The solution of exercise 3.71: the Ramanujan numbers through ordered
//! streams of pairs, the trick the statement credits to Charles
//! Leiserson. Pairs `(i, j)` of integers ordered by the weight
//! `i*i*i + j*j*j` emit equal weights adjacently, so a number written as
//! a sum of two cubes in two ways shows up as a run of equal weights in
//! the ordered stream. A local grouping combinator folds the ordered
//! stream into runs of equal weight; the runs of length two or more are
//! the Ramanujan numbers, each carrying its witness pairs.

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

/// The statement's search: cube-weighted ordered pairs, grouped into
/// equal-weight runs, keeping the runs of two or more -- the numbers
/// expressible as a sum of two cubes in two different ways, 1729 and
/// the next five the statement asks for.
#[must_use]
fn ramanujan_runs() -> Vec<WeightRun> {
    let cube_weight = |pair: &(i128, i128)| pair.0.pow(3) + pair.1.pow(3);
    let weighted = weighted_pairs(&integers(), &integers(), cube_weight);
    weight_runs(&weighted, cube_weight)
        .iter()
        .filter(|run| run.1 >= 2)
        .take(6)
        .collect()
}

/// The answer shape of the exercise: the qualifying weights, each with
/// the witness pairs of its representations, in emission order.
type WeightedWitnesses = (Vec<i128>, Vec<Vec<(i128, i128)>>);

mod ex_3_71 {
    use super::{WeightedWitnesses, ramanujan_runs};

    /// Exercise 3.71: Ramanujan numbers via weighted pairs
    ///
    /// Answers 1729 and the next five Ramanujan numbers, each with the
    /// witness pairs whose equal cube weight produced the run.
    #[must_use]
    pub fn ex_3_71() -> WeightedWitnesses {
        let runs = ramanujan_runs();
        let numbers = runs.iter().map(|run| run.0).collect();
        let witnesses = runs.into_iter().map(|(_, _, members)| members).collect();
        (numbers, witnesses)
    }
}

#[test]
fn ex_3_71() {
    // 1729 = 1^3 + 12^3 = 9^3 + 10^3, the taxicab exchange of the
    // statement's footnote; then 4104, 13832, 20683, 32832, 39312 are
    // the next five the statement asks for. Equal weights emit
    // adjacently, the lower first coordinate first.
    assert_eq!(
        ex_3_71::ex_3_71(),
        (
            vec![1729, 4104, 13832, 20683, 32832, 39312],
            vec![
                vec![(1, 12), (9, 10)],
                vec![(2, 16), (9, 15)],
                vec![(2, 24), (18, 20)],
                vec![(10, 27), (19, 24)],
                vec![(4, 32), (18, 30)],
                vec![(2, 34), (15, 33)],
            ],
        )
    );
}
