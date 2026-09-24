// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.82: exercise 3.5's Monte
//! Carlo integration as a stream of ever-better estimates. The
//! seeded word stream is paired up two words at a time and scaled
//! into the rectangle, the experiment's outcomes feed the section's
//! `monte-carlo`, and the running fraction is scaled by the
//! rectangle's area. No trial-count argument, no assignment: the
//! only "state" is the word stream itself, and more trials are just
//! a longer prefix. The confidence question -- how many trials until
//! the estimate is this good -- is answered by walking the estimate
//! stream, which is what the book means by looking farther into it.

use std::f64::consts::PI;

use ch03::sec_3_5::{
    Stream, map_successive_pairs, monte_carlo_stream, random_numbers, scale_stream, stream_map,
};

/// The book's `estimate-integral` of exercise 3.5, streaming: answers
/// the stream of successively better estimates of the area picked out
/// by `experiment` inside the rectangle `[x1, x2] x [y1, y2]`, one
/// estimate per random point.
#[must_use]
pub fn estimate_integral<F>(experiment: F, x1: f64, x2: f64, y1: f64, y2: f64) -> Stream<f64>
where
    F: Fn(f64, f64) -> bool + Clone + 'static,
{
    // Two words per point, the book's `random-in-range` applied to
    // each half of the pair.
    let points = map_successive_pairs(
        move |r1: &u64, r2: &u64| (word_in_range(*r1, x1, x2), word_in_range(*r2, y1, y2)),
        &random_numbers(),
    );
    let experiments = stream_map(move |p: &(f64, f64)| experiment(p.0, p.1), &points);
    let area = (x2 - x1) * (y2 - y1);
    scale_stream(&monte_carlo_stream(&experiments, 0, 0), area)
}

/// The book's `random-in-range` over one generator word: scales the
/// word into `[low, high)` as a fraction of the whole word space.
fn word_in_range(word: u64, low: f64, high: f64) -> f64 {
    #[expect(
        clippy::cast_precision_loss,
        reason = "one u64 -> f64 rounding per word is this edition's spelling of a unit-interval draw; the estimates sit far inside their asserted bounds"
    )]
    let fraction = word as f64 / u64::MAX as f64;
    low + fraction * (high - low)
}

/// The book's circle-in-square experiment: the unit circle centered at
/// (1, 1) over the (0..2)^2 square, whose area ratio is pi over 4.
fn circle_in_square() -> Stream<f64> {
    estimate_integral(
        |x, y| (x - 1.0) * (x - 1.0) + (y - 1.0) * (y - 1.0) <= 1.0,
        0.0,
        2.0,
        0.0,
        2.0,
    )
}

/// Walks `estimates` until one lands within `bound` of `target` and
/// answers the 1-based trial count that produced it: the streaming
/// answer to how many trials this confidence costs.
///
/// # Panics
/// Panics when the estimate stream ends before the bound is met,
/// which the seeded circle run never reaches.
fn first_within(estimates: &Stream<f64>, target: f64, bound: f64) -> usize {
    for (trial, estimate) in estimates.iter().enumerate() {
        if (estimate - target).abs() < bound {
            return trial + 1;
        }
    }
    panic!("estimate stream ended before the bound was met");
}

/// The estimate `n` trials in, walked with the stream cursor: the pins
/// sit deep enough that `stream_ref`'s recursion would overrun a test
/// thread's stack, and the cursor forces memoized tails iteratively.
fn sample(estimates: &Stream<f64>, n: usize) -> f64 {
    estimates
        .iter()
        .nth(n)
        .expect("the estimate stream is endless")
}

mod ex_3_82 {
    use super::{PI, circle_in_square, first_within, sample};

    /// Exercise 3.82: streaming Monte Carlo integration
    ///
    /// Answers, for [`super::circle_in_square`]: the estimate stream's
    /// samples at 100, 1000, and 10000 trials, and the 1-based trial
    /// count where the estimate first lands within 0.1 of pi.
    ///
    /// The 10000-cell spine is forgotten rather than dropped: unwinding
    /// it recurses cell by cell and overruns a test thread's stack, and
    /// the runtime already leaks self-referential spines by design,
    /// reclaimed at process exit -- which for a test is immediate.
    #[must_use]
    pub fn ex_3_82() -> ([f64; 3], usize) {
        let estimates = circle_in_square();
        let answer = (
            [
                sample(&estimates, 99),
                sample(&estimates, 999),
                sample(&estimates, 9_999),
            ],
            first_within(&estimates, PI, 0.1),
        );
        std::mem::forget(estimates);
        answer
    }
}

#[test]
fn ex_3_82() {
    let (estimates, trial) = ex_3_82::ex_3_82();
    // Exact rationals of the seeded chain: 81 of the first 100 pairs
    // land in the circle, 765 of 1000, 7866 of 10000, each fraction
    // times the square's area 4.
    assert!((estimates[0] - 3.24).abs() < 1e-12);
    assert!((estimates[1] - 3.06).abs() < 1e-12);
    assert!((estimates[2] - 3.146_4).abs() < 1e-12);
    // The confidence walk's first hit: trial 21 is the 16th success,
    // an estimate of 4*16/21 = 3.0476 inside 0.1 of pi, and no
    // earlier estimate is (the 20th, 15 of 20, is 3.0 exactly).
    assert_eq!(trial, 21);
}

/// The same walk at the tighter 0.01 bound answers its own
/// deterministic count, 111: confidence on demand, with no
/// trial-count argument anywhere in the program.
#[test]
fn tighter_bound_walks_farther() {
    let estimates = circle_in_square();
    assert_eq!(first_within(&estimates, PI, 0.01), 111);
}
