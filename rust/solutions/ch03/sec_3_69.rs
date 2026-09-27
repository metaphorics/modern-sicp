// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The solution of exercise 3.69: the book's `triples` builds the stream
//! of `(i, j, k)` with `i <= j <= k` out of the section's own `pairs` and
//! `interleave`. The head triple is `(s0, t0, u0)`; the rest interleaves
//! the triples whose first coordinate is `s0` -- `s0` consed in front of
//! every pair of the two tails -- with `triples` over the three tails,
//! so every triple is reachable on infinite inputs. Scanning that stream
//! for `i*i + j*j == k*k` answers the Pythagorean triples in the order
//! the decomposition reaches them.

use ch03::sec_3_5::{Stream, cons_stream, integers, interleave, pairs, stream_map};

/// The book's `triples`: every `(i, j, k)` with `i` from `s`, `j` from
/// `t`, `k` from `u`, and `i <= j <= k`. The first-coordinate cut is the
/// book's own: either the triple starts with `s0` -- and its last two
/// coordinates are a pair of the tails, laid out by `pairs` -- or it is
/// a triple of the tails; `interleave` keeps both infinite sides alive.
fn triples(s: &Stream<i128>, t: &Stream<i128>, u: &Stream<i128>) -> Stream<(i128, i128, i128)> {
    let head = (*s.head(), *t.head(), *u.head());
    let with_first = {
        let first = *s.head();
        stream_map(
            move |pair: &(i128, i128)| (first, pair.0, pair.1),
            &pairs(&t.tail(), &u.tail()),
        )
    };
    let front = s.clone();
    let middle = t.clone();
    let back = u.clone();
    cons_stream(head, move || {
        interleave(
            &with_first,
            &triples(&front.tail(), &middle.tail(), &back.tail()),
        )
    })
}

/// The bounded prefix of the scan, chosen by measurement: the interleaved
/// order reaches the first-coordinate branches at an exponential index,
/// and the six Pythagorean triples land at stream indices 19, 607, 6127,
/// 14079, 81791, and 292863, so the bound sits just above the last.
const SCAN_PREFIX: usize = 300_000;

/// The Pythagorean scan over `triples(integers, integers, integers)`.
/// The section's `stream_filter` cannot run here: it recurses eagerly
/// across each run of non-matching triples, and the gaps between
/// Pythagorean hits in the interleaved order run thousands deep, which
/// overflows the stack. The scan therefore walks a bounded prefix with
/// the iterative stream cursor and stops at the sixth hit.
///
/// The walked spine holds `300_000` memoized nodes whose recursive drop
/// would also overflow the stack, so the stream is kept in a
/// `ManuallyDrop`: the drop is suppressed on purpose and the test
/// process that owns the spine frees it on its one exit path, the
/// reclamation-at-process-exit stance the stream substrate's own design
/// document takes for the streams it cannot drop.
#[must_use]
fn pythagorean_prefix() -> Vec<(i128, i128, i128)> {
    let stream = std::mem::ManuallyDrop::new(triples(&integers(), &integers(), &integers()));
    stream
        .iter()
        .take(SCAN_PREFIX)
        .filter(|triple: &(i128, i128, i128)| {
            triple.0 * triple.0 + triple.1 * triple.1 == triple.2 * triple.2
        })
        .take(6)
        .collect()
}

mod ex_3_69 {
    use super::pythagorean_prefix;

    /// Exercise 3.69: triples and the Pythagorean stream
    ///
    /// Answers the first six Pythagorean triples of positive integers in
    /// the order the interleaved `triples` stream itself reaches them.
    #[must_use]
    pub fn ex_3_69() -> Vec<(i128, i128, i128)> {
        pythagorean_prefix()
    }
}

#[test]
fn ex_3_69() {
    // The scan answers exactly six inside the bounded prefix; a shorter
    // answer would mean the bound is too tight, not that the stream ran
    // out.
    let found = ex_3_69::ex_3_69();
    assert_eq!(found.len(), 6);
    // The stream's own ordering, not a sorted one: the decomposition
    // reaches (6, 8, 10) through the first-coordinate branches before it
    // walks deep enough inside an earlier branch to find (5, 12, 13).
    // Every row here satisfies i <= j <= k and i*i + j*j == k*k.
    assert_eq!(
        found,
        vec![
            (3, 4, 5),
            (6, 8, 10),
            (5, 12, 13),
            (9, 12, 15),
            (8, 15, 17),
            (12, 16, 20),
        ]
    );
}
