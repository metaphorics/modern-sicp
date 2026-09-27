// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.76: smoothing as a reusable
//! component. `smooth` maps the average over the stream and its own
//! tail, so each output element is the mean of two successive input
//! elements, and the zero-crossing detector runs unchanged on the
//! conditioned signal -- the extractor no longer knows how the input
//! was smoothed, Eva Lu Ator's modularity point.

use ch03::sec_3_5::{Stream, cons_stream, make_zero_crossings, self_stream, stream_map2};

/// The book's sense data, as in exercise 3.74: the thirteen printed
/// values extended by a flat tail of the last value 4, so the walks
/// never end mid-way.
#[must_use]
fn sense_data() -> Stream<f64> {
    let printed: [f64; 13] = [
        1.0, 2.0, 1.5, 1.0, 0.5, -0.1, -2.0, -3.0, -2.0, -0.5, 0.2, 3.0, 4.0,
    ];
    let tail = self_stream(|fours| cons_stream(4.0, move || fours.stream()));
    printed.iter().rev().fold(tail, |rest, value| {
        cons_stream(*value, move || rest.clone())
    })
}

/// The book's `smooth`: output element k averages input elements k and
/// k+1 (the stream and its own tail under the two-stream map), so a
/// finite input answers a stream one element shorter.
#[must_use]
fn smooth(s: &Stream<f64>) -> Stream<f64> {
    #[expect(
        clippy::manual_midpoint,
        reason = "the statement's plain averaging of two successive elements stays as written"
    )]
    stream_map2(|a: &f64, b: &f64| (*a + *b) / 2.0, s, &s.tail())
}

/// The first `N` elements of `stream`.
#[must_use]
fn prefix<const N: usize>(stream: &Stream<f64>) -> [f64; N] {
    let mut window = [0.0; N];
    for (slot, value) in window.iter_mut().zip(stream.iter()) {
        *slot = value;
    }
    window
}

mod ex_3_76 {
    use super::{make_zero_crossings, prefix, sense_data, smooth};

    /// Exercise 3.76: smooth as reusable combinator
    ///
    /// Answers the first thirteen smoothed sense values and the first
    /// fourteen crossings of the smoothed signal, the section's own
    /// detector run unchanged on the smoothed stream with last value 0.
    #[must_use]
    pub fn ex_3_76() -> ([f64; 13], [f64; 14]) {
        let smoothed = smooth(&sense_data());
        let crossings = make_zero_crossings(&smoothed, 0.0);
        (prefix(&smoothed), prefix(&crossings))
    }
}

#[test]
fn ex_3_76() {
    let (smoothed, crossings) = ex_3_76::ex_3_76();

    // The smoothed signal: each cell is the mean of the raw cell and
    // its successor -- 1.5 = (1 + 2)/2, ..., -1.05 = (-0.1 + -2)/2.
    for (got, want) in smoothed.iter().zip([
        1.5, 1.75, 1.25, 0.75, 0.2, -1.05, -2.5, -2.5, -1.25, -0.15, 1.6, 3.5, 4.0,
    ]) {
        assert!((got - want).abs() < 1e-12, "smoothed: {got} vs {want}");
    }

    // The two real crossings survive and no spurious crossing appears:
    // -1 at index 5 (smoothed 0.2 = (0.5 + -0.1)/2 before, -1.05 =
    // (-0.1 + -2)/2 at) and +1 at index 10 (smoothed -0.15 before, 1.6
    // at). The indices match the raw answer of 3.74, and run one
    // earlier than the 3.75 fixed form's 6 and 11, because smooth's
    // element k already mixes raw k with k+1 while the fixed form's
    // k-th average pairs k with k-1: the same averaged values, one
    // index apart. The detector itself is the section's untouched
    // make_zero_crossings -- smoothing stayed a component.
    for (got, want) in crossings.iter().zip([
        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
    ]) {
        assert!((got - want).abs() < 1e-12, "crossing: {got} vs {want}");
    }
}
