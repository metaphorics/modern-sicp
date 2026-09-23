// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.3: square roots as a damped fixed-point search.

use ch01::sec_1_1::average;
use ch01::sec_1_3::fixed_point;

/// The book's second spelling of `sqrt`: the search for a fixed point of
/// `y` mapped to the average of `y` and `x / y`. The undamped spelling of
/// the prose oscillates forever and is shown only to make the point.
fn sqrt(x: f64) -> f64 {
    fixed_point(move |y| average(y, x / y), 1.0)
}

fn main() {
    let root = sqrt(9.0);
    println!("{root}");
    // => 3
    assert!((root - 3.0).abs() < 1e-9);

    let root = sqrt(16.0);
    println!("{root}");
    // => 4.000000000000051
    assert!((root - 4.0).abs() < 0.001);
}
