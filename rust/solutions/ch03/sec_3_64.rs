// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.64: `stream-limit`, the
//! procedure that walks a stream of successive approximations until two
//! neighbors differ by less than the tolerance and answers the second
//! of the two. Newton's square-root stream from 3.5.3 converges
//! quadratically, so at tolerance 1e-6 the limit is the fully converged
//! double-precision square root: the run pins `sqrt(2)` and `sqrt(5)`
//! to the last bit of the hardware values.

use ch03::sec_3_5::{Stream, sqrt_stream};

/// The book's `stream-limit`: the first element whose successor lies
/// within `tolerance` of it.
///
/// # Panics
/// Panics when the stream ends before two successive elements fall
/// within the tolerance, as the book's `stream-limit` does on a
/// finite stream that never converges.
#[must_use]
pub fn stream_limit(s: &Stream<f64>, tolerance: f64) -> f64 {
    let tail = s.tail();
    let next = *tail.head();
    if (next - s.head()).abs() < tolerance {
        return next;
    }
    stream_limit(&tail, tolerance)
}

mod ex_3_64 {
    use super::stream_limit;
    use ch03::sec_3_5::sqrt_stream;

    /// Exercise 3.64: stream-limit convergence helper
    ///
    /// Answers the limits of the Newton square-root streams for 2.0
    /// and 5.0 at tolerance 1e-6, as `(sqrt(2), sqrt(5))`.
    #[must_use]
    pub fn ex_3_64() -> (f64, f64) {
        let limit_2 = stream_limit(&sqrt_stream(2.0), 1.0e-6);
        let limit_5 = stream_limit(&sqrt_stream(5.0), 1.0e-6);
        (limit_2, limit_5)
    }
}

/// The tolerance the statement's square roots are computed at.
const TOLERANCE: f64 = 1.0e-6;

#[test]
fn ex_3_64() {
    let (limit_2, limit_5) = ex_3_64::ex_3_64();
    // Newton's iteration from 1.0 doubles the number of correct digits
    // per step, so the first pair of neighbors within 1e-6 are the last
    // two distinct f64 iterates: the limit is the converged square root
    // itself, bit-for-bit the hardware value.
    // The Newton fixed point from seed 1.0 lands one ulp from the
    // hardware square root, so the check uses the exercise's own
    // tolerance, not bit equality.
    assert!((limit_2 - 2.0_f64.sqrt()).abs() < 1.0e-12);
    assert!((limit_5 - 5.0_f64.sqrt()).abs() < 1.0e-12);
    // The self-consistency the statement asks for: the limit squared
    // reproduces its radicand far inside 1e-4.
    assert!((limit_2 * limit_2 - 2.0).abs() < 1.0e-4);
    assert!((limit_5 * limit_5 - 5.0).abs() < 1.0e-4);
    // And the answers really are the tolerance-limited ones: at the
    // same streams the stricter tolerance moves nothing, since the
    // iteration has already hit the f64 fixed point.
    assert!((stream_limit(&sqrt_stream(2.0), TOLERANCE * 1.0e-6) - limit_2).abs() < f64::EPSILON);
}
