// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.79: `solve-2nd` generalized to
//! `d^2 y/dt^2 = f(dy/dt, y)`. The two-integral loop of exercise 3.78
//! keeps its shape; the only change is the feedback element, where the
//! scaled adder `a dy + b y` becomes `f` mapped over the `dy`/`y`
//! pair. Two runs exercise it: `f = -y` must reproduce exercise 3.78's
//! oscillator, and `f = -0.1 dy - y` must show the damped run's
//! amplitude decaying period over period.

use ch03::sec_3_5::{Stream, integral_delayed, self_stream, stream_map2, stream_ref};

/// The book's generalized `solve-2nd`: the stream of `y` values of the
/// solution of `d^2 y/dt^2 = f(dy/dt, y)` with `y(0)` = `y0` and
/// `y'(0)` = `dy0`, integrated at step `dt`.
///
/// The network is exercise 3.78's with `f` in the feedback adder:
/// `y` integrates the delayed `dy`, `dy` integrates the delayed `ddy`,
/// and `ddy` is `f` over the two named streams. Each integrand thunk is
/// forced only after both definitions are complete, so the loop reads
/// finished streams. `f` is cloned into the tail thunk that keeps the
/// recursion going, hence the `Clone` bound.
#[must_use]
pub fn solve_2nd_general<F>(f: F, dt: f64, y0: f64, dy0: f64) -> Stream<f64>
where
    F: Fn(f64, f64) -> f64 + Clone + 'static,
{
    self_stream(move |y| {
        let dy = self_stream(move |dy_name| {
            let ddy = {
                let y = y.clone();
                let f = f.clone();
                move || {
                    let dy_stream = dy_name.stream();
                    let y_stream = y.stream();
                    stream_map2(move |dy, y| f(*dy, *y), &dy_stream, &y_stream)
                }
            };
            integral_delayed(ddy, dy0, dt)
        });
        integral_delayed(move || dy, y0, dt)
    })
}

/// One full oscillation of the damped run in samples: `2 pi / omega`
/// with `omega = sqrt(1 - 0.05^2)`, at `dt` = 0.01.
const SAMPLES_PER_PERIOD: usize = 629;

/// The peak `|y|` of each of the five one-period windows of the
/// damped run, window `w` covering samples
/// `[w * SAMPLES_PER_PERIOD, (w + 1) * SAMPLES_PER_PERIOD)`. One pass
/// of the stream cursor: the windows sit deep enough that per-index
/// `stream_ref` recursion is both quadratic and stack-hungry on a
/// test thread.
fn window_peaks(damped: &Stream<f64>) -> [f64; 5] {
    damped
        .iter()
        .enumerate()
        .take(5 * SAMPLES_PER_PERIOD)
        .fold([0.0; 5], |mut peaks, (i, y)| {
            let window = i / SAMPLES_PER_PERIOD;
            peaks[window] = peaks[window].max(y.abs());
            peaks
        })
}

mod ex_3_79 {
    use super::{stream_ref, window_peaks};

    /// Exercise 3.79: general second-order solver
    ///
    /// Answers two runs of [`super::solve_2nd_general`] at `dt` = 0.01,
    /// `y0` = 1, `dy0` = 0: the undamped oscillator `f(dy, y) = -y`
    /// sampled at `y` indices 0, 50, 157, 314, and the damped run
    /// `f(dy, y) = -0.1 dy - y` reduced to the peak `|y|` of each of
    /// five successive one-period windows.
    #[must_use]
    pub fn ex_3_79() -> ((f64, f64, f64, f64), [f64; 5]) {
        let undamped = super::solve_2nd_general(|_dy, y| -y, 0.01, 1.0, 0.0);
        let damped = super::solve_2nd_general(|dy, y| -0.1 * dy - y, 0.01, 1.0, 0.0);
        (
            (
                stream_ref(&undamped, 0),
                stream_ref(&undamped, 50),
                stream_ref(&undamped, 157),
                stream_ref(&undamped, 314),
            ),
            window_peaks(&damped),
        )
    }
}

#[test]
fn ex_3_79() {
    let (undamped, envelope) = ex_3_79::ex_3_79();
    let (y0, y50, y157, y314) = undamped;
    // `f = -y` is the equation of exercise 3.78, so the general solver
    // must answer its exact forward-Euler stream back.
    assert!((y0 - 1.0).abs() < 1e-12);
    assert!((y50 - 0.879_787_162_885_263).abs() < 1e-12);
    assert!((y157 - 0.000_855_344_724_179).abs() < 1e-12);
    assert!((y314 + 1.015_821_631_924_213).abs() < 1e-12);
    assert!((y0 - 0.0_f64.cos()).abs() < 1e-12);
    assert!((y50 - 0.5_f64.cos()).abs() < 0.01);
    assert!((y157 - 1.57_f64.cos()).abs() < 0.001);
    assert!((y314 - (314.0_f64 * 0.01).cos()).abs() < 0.05);
    // The damped run's window peaks, pinned exactly (tolerance 1e-9,
    // the rounding of the 12 printed digits), then the decay itself:
    // strictly shrinking amplitude period over period.
    for (peak, pinned) in envelope.into_iter().zip(PINNED_ENVELOPE) {
        assert!((peak - pinned).abs() < 1e-9);
    }
    assert!(envelope[0] > envelope[1]);
    assert!(envelope[1] > envelope[2]);
    assert!(envelope[2] > envelope[3]);
    assert!(envelope[3] > envelope[4]);
}

/// The five measured window peaks of the damped run. Each is about
/// 0.7534 of the one before: the e^(-0.05 t) envelope decay over one
/// period (0.730) times explicit Euler's slow amplitude pump per
/// window (about 1.032).
const PINNED_ENVELOPE: [f64; 5] = [
    1.0,
    0.753_455_690_398,
    0.567_693_464_718,
    0.427_728_838_755,
    0.322_271_300_395,
];
