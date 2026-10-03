// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.74: Eva Lu Ator's rewrite of
//! the zero-crossing detector with the general two-stream map of
//! exercise 3.50. The crossings are the sign-change detector mapped
//! over the sense data and the same data shifted one cell into the
//! past, the shift seeded with the initial previous value 0 -- no
//! explicit recursion anywhere.

use ch03::sec_3_5::{Stream, cons_stream, self_stream, sign_change_detector, stream_map2};

/// The book's sense data: 1 2 1.5 1 0.5 -0.1 -2 -3 -2 -0.5 0.2 3 4 ...
/// On the page the signal continues indefinitely, so the port extends
/// the thirteen printed values with a flat tail of the last value 4;
/// the crossing walk then never ends mid-way and every later
/// comparison settles at 0.
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

/// Alyssa's zero-crossing stream in Eva Lu Ator's form: the detector
/// mapped over the sense data and the same data delayed by one cell,
/// prefixed with the initial zero. The book's first-order
/// `sign-change-detector` is lifted over the map's reference arguments.
#[must_use]
fn zero_crossings(sense: &Stream<f64>) -> Stream<f64> {
    let delayed = {
        let again = sense.clone();
        cons_stream(0.0, move || again)
    };
    stream_map2(
        |current: &f64, previous: &f64| sign_change_detector(*current, *previous),
        sense,
        &delayed,
    )
}

/// The first fourteen elements of `stream`, the pinned window of this
/// exercise's test.
#[must_use]
fn prefix14(stream: &Stream<f64>) -> [f64; 14] {
    let mut window = [0.0; 14];
    for (slot, value) in window.iter_mut().zip(stream.iter()) {
        *slot = value;
    }
    window
}

mod ex_3_74 {
    use super::{prefix14, sense_data, zero_crossings};

    /// Exercise 3.74: zero crossings via stream-map
    ///
    /// Answers the first fourteen crossings of the book's sense data,
    /// the delayed stream seeded with previous value 0.
    #[must_use]
    pub fn ex_3_74() -> [f64; 14] {
        prefix14(&zero_crossings(&sense_data()))
    }
}

#[test]
fn ex_3_74() {
    let crossings = ex_3_74::ex_3_74();
    // The book's own rows: the raw signal turns negative at -0.1
    // (index 5) and back positive at 0.2 (index 10), so the crossing
    // stream is 0 everywhere but -1 at index 5 and +1 at index 10; the
    // flat 4s tail keeps every later comparison settled at 0.
    for (got, want) in crossings.iter().zip([
        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
    ]) {
        assert!((got - want).abs() < 1e-12, "crossing: {got} vs {want}");
    }
}
