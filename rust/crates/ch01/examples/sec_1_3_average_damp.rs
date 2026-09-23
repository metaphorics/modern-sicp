// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.4: average damping as a function that returns functions.

use ch01::sec_1_1::square;
use ch01::sec_1_3::{average_damp, cube_root, fixed_point};

/// The book's reformulated `sqrt`: a fixed-point search over the
/// average-damped `y` mapped to `x / y`.
fn sqrt(x: f64) -> f64 {
    fixed_point(average_damp(move |y| x / y), 1.0)
}

fn main() {
    let damped_square = average_damp(square);
    println!("{}", damped_square(10.0));
    // => 55
    assert!((damped_square(10.0) - 55.0).abs() < 1e-9);

    let root = sqrt(9.0);
    println!("{root}");
    // => 3
    assert!((root - 3.0).abs() < 1e-9);

    let root = cube_root(27.0);
    println!("{root}");
    // => 2.9999972321057697
    assert!((root - 3.0).abs() < 0.001);
}
