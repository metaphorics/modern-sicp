// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.4: abstractions and first-class procedures.

use ch01::sec_1_3::{average_damp_dyn, fixed_point_of_transform, newton_transform_dyn};

fn main() {
    let x = 9.0;
    let root = fixed_point_of_transform(Box::new(move |y| x / y), average_damp_dyn, 1.0);
    println!("{root}");
    // => 3
    assert!((root - 3.0).abs() < 1e-9);

    let root = fixed_point_of_transform(Box::new(|y| y * y - 9.0), newton_transform_dyn, 1.0);
    println!("{root}");
    // => 3.000000000000002
    assert!((root - 3.0).abs() < 0.00001);
}
