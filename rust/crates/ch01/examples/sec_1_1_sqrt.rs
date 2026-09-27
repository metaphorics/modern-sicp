// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 1.1, listing 5: square roots by Newton's method.

use ch01::sec_1_1::sqrt;

fn main() {
    let root = sqrt(9.0);
    println!("{root}");
    // => 3.00009155413138
    assert!((root - 3.0).abs() < 0.001);

    let root = sqrt(100.0 + 37.0);
    println!("{root}");
    // => 11.704699917758145
    assert!((root * root - 137.0).abs() < 0.1);

    let root = sqrt(sqrt(2.0) + sqrt(3.0));
    println!("{root}");
    // => 1.7739279023207892
    assert!((root * root - (2.0f64.sqrt() + 3.0f64.sqrt())).abs() < 0.1);

    let square_of_root = sqrt(1000.0) * sqrt(1000.0);
    println!("{square_of_root}");
    // => 1000.000369924366
    assert!((square_of_root - 1000.0).abs() < 0.1);
}
