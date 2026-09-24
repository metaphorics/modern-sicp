// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.65: the natural logarithm of 2
//! as the series `1 - 1/2 + 1/3 - 1/4 + ...`, computed the way the text
//! computes pi. The summands are the alternating reciprocals; their
//! partial sums, the Euler transform of those partial sums, and the
//! recursively accelerated sequence (the first element of every row of
//! the tableau) give the three approximation sequences. The run pins
//! element 10 of the raw sums, element 5 of the Euler transform, and
//! element 5 of the accelerated sequence, and the errors against
//! `ln 2 = 0.6931471805599453` tell how rapidly each converges.

use ch03::sec_3_5::{
    Stream, accelerate_sequence, cons_stream, euler_transform, partial_sums, stream_map,
};

/// The alternating reciprocal summands of ln 2: `1, -1/2, 1/3, -1/4,
/// ...`, the `pi-summands` shape with step 1 instead of 2.
fn ln2_summands(n: f64) -> Stream<f64> {
    cons_stream(1.0 / n, move || stream_map(|x| -x, &ln2_summands(n + 1.0)))
}

mod ex_3_65 {
    use super::{accelerate_sequence, euler_transform, ln2_summands, partial_sums};
    use ch03::sec_3_5::stream_ref;

    /// Exercise 3.65: ln 2 approximation streams
    ///
    /// Answers element 10 of the raw partial sums, element 5 of the
    /// Euler transform, and element 5 of the recursively accelerated
    /// sequence, as `(raw_10, euler_5, accelerated_5)`.
    #[must_use]
    pub fn ex_3_65() -> (f64, f64, f64) {
        let sums = partial_sums(&ln2_summands(1.0));
        let raw_10 = stream_ref(&sums, 10);
        let euler_5 = stream_ref(&euler_transform(&sums), 5);
        let accelerated_5 = stream_ref(&accelerate_sequence(euler_transform, &sums), 5);
        (raw_10, euler_5, accelerated_5)
    }
}

/// The double nearest the natural logarithm of 2.
use std::f64::consts::LN_2;

#[test]
fn ex_3_65() {
    let (raw_10, euler_5, accelerated_5) = ex_3_65::ex_3_65();
    // Element 10 of the raw partial sums is 1 - 1/2 + ... + 1/11: the
    // alternating-series remainder is at most the next term, 1/12, and
    // the measured error is about 4.3e-2 -- roughly one digit.
    assert!((raw_10 - LN_2).abs() < 5.0e-2);
    // Element 5 of the Euler transform needs only 7 raw terms and is
    // already within about 2.5e-4 of ln 2 -- better than the raw sums'
    // element 10 by two orders of magnitude, at roughly 3 digits.
    assert!((euler_5 - LN_2).abs() < 1.0e-3);
    // Element 5 of the recursively accelerated sequence carries the
    // tableau's compounding: within about 1.7e-9, roughly 8 digits,
    // from the same 7 raw terms.
    assert!((accelerated_5 - LN_2).abs() < 1.0e-8);
    // The ordering the statement asks about: each acceleration stage
    // is strictly better than the one before it, by wide margins.
    assert!((raw_10 - LN_2).abs() > (euler_5 - LN_2).abs() * 100.0);
    assert!((euler_5 - LN_2).abs() > (accelerated_5 - LN_2).abs() * 1.0e4);
}
