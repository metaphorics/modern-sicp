// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.75: Louis Reasoner's buggy
//! smoothing detector, translated verbatim, and the fixed structure the
//! hint asks for. Alyssa's plan extracts crossings from the signal
//! that averages each sense value with the previous raw value, so the
//! detector must compare each new average against the previous AVERAGE.
//! Louis feeds each average back as the next previous raw value, so
//! from the second cell on his "average" mixes the raw current value
//! with a contaminated running blend instead of the adjacent raw pair.

use ch03::sec_3_5::{Stream, cons_stream, self_stream, sign_change_detector};

/// The book's sense data, as in exercise 3.74: the thirteen printed
/// values extended by a flat tail of the last value 4, so the walk
/// never ends mid-way.
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

/// Louis Reasoner's version, translated verbatim: one `last_value`
/// argument; `avpt` is the average of the current head and that
/// previous value; the detector compares `avpt` against `last_value`;
/// and `avpt` itself is passed back as the next `last_value`.
#[must_use]
#[expect(
    clippy::manual_midpoint,
    reason = "the book's averaging formula stays as written"
)]
fn make_zero_crossings_louis(input_stream: &Stream<f64>, last_value: f64) -> Stream<f64> {
    let avpt = (*input_stream.head() + last_value) / 2.0;
    let source = input_stream.clone();
    cons_stream(sign_change_detector(avpt, last_value), move || {
        make_zero_crossings_louis(&source.tail(), avpt)
    })
}

/// The internal signal of Louis's version: the averages his detector
/// compares, the same verbatim recurrence with the `let`-bound `avpt`
/// lifted to the output so the test can pin the contamination itself.
#[must_use]
#[expect(
    clippy::manual_midpoint,
    reason = "the book's averaging formula stays as written"
)]
fn louis_averages(input_stream: &Stream<f64>, last_value: f64) -> Stream<f64> {
    let avpt = (*input_stream.head() + last_value) / 2.0;
    let source = input_stream.clone();
    cons_stream(avpt, move || louis_averages(&source.tail(), avpt))
}

/// The fix, same structure with the hint's extra argument: `avpt` is
/// still the average of the current head and the previous RAW value,
/// but the detector compares each new average against the previous
/// AVERAGE, and the recursion carries the raw head and the average
/// separately.
#[must_use]
#[expect(
    clippy::manual_midpoint,
    reason = "the book's averaging formula stays as written"
)]
fn make_zero_crossings_fixed(
    input_stream: &Stream<f64>,
    last_value: f64,
    last_avpt: f64,
) -> Stream<f64> {
    let avpt = (*input_stream.head() + last_value) / 2.0;
    let raw = *input_stream.head();
    let source = input_stream.clone();
    cons_stream(sign_change_detector(avpt, last_avpt), move || {
        make_zero_crossings_fixed(&source.tail(), raw, avpt)
    })
}

/// The first fourteen elements of `stream`, the pinned window of this
/// exercise's tests.
#[must_use]
fn prefix14(stream: &Stream<f64>) -> [f64; 14] {
    let mut window = [0.0; 14];
    for (slot, value) in window.iter_mut().zip(stream.iter()) {
        *slot = value;
    }
    window
}

/// Louis's crossings, the fixed form's, and his internal averages.
pub type CrossingReport = ([f64; 14], [f64; 14], [f64; 7]);

mod ex_3_75 {
    use super::{
        CrossingReport, louis_averages, make_zero_crossings_fixed, make_zero_crossings_louis,
        prefix14, sense_data,
    };

    /// Exercise 3.75: buggy smoothing detector fix
    ///
    /// Answers Louis's crossings on the sense data, the fixed form's
    /// crossings, and the first seven of Louis's internal averages.
    #[must_use]
    pub fn ex_3_75() -> CrossingReport {
        let sense = sense_data();
        let louis = prefix14(&make_zero_crossings_louis(&sense, 0.0));
        let fixed = prefix14(&make_zero_crossings_fixed(&sense, 0.0, 0.0));
        let averages: [f64; 7] = {
            let mut window = [0.0; 7];
            for (slot, value) in window.iter_mut().zip(louis_averages(&sense, 0.0).iter()) {
                *slot = value;
            }
            window
        };
        (louis, fixed, averages)
    }
}

#[test]
fn ex_3_75() {
    let (louis, fixed, averages) = ex_3_75::ex_3_75();

    // Louis's crossings on the sense data: -1 at index 6 and +1 at
    // index 11, one step later than Alyssa's raw answer of exercise
    // 3.74 (-1 at index 5, +1 at index 10), because he detects on a
    // lagging blend instead of the raw signal.
    for (got, want) in louis.iter().zip([
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    ]) {
        assert!(
            (got - want).abs() < 1e-12,
            "louis crossing: {got} vs {want}"
        );
    }

    // The fixed form answers the smoothed crossings: its averages are
    // the plan's adjacent-pair means, first negative at index 6
    // ((-0.1 + -2)/2 = -1.05 against 0.2 before) and first positive at
    // index 11 ((3 + 0.2)/2 = 1.6 against -0.15 before). On this clean
    // data the buggy positions happen to coincide; the averages below
    // are where the versions part ways.
    for (got, want) in fixed.iter().zip([
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    ]) {
        assert!(
            (got - want).abs() < 1e-12,
            "fixed crossing: {got} vs {want}"
        );
    }

    // Louis's averaging ladder: 0.5 = (1 + 0)/2 is the plan's first
    // average, but from the second cell on his averages stop being
    // adjacent-pair means -- the plan's second average is (2 + 1)/2 =
    // 1.5, his is (2 + 0.5)/2 = 1.25, averaging the raw value with his
    // own previous output. The blend leaks each old average forward
    // forever, so his "smoothing" is an exponential tail, and his first
    // comparison pits an average against the raw seed 0.
    for (got, want) in
        averages
            .iter()
            .zip([0.5, 1.25, 1.375, 1.187_5, 0.843_75, 0.371_875, -0.814_062_5])
    {
        assert!((got - want).abs() < 1e-12, "louis average: {got} vs {want}");
    }
}
