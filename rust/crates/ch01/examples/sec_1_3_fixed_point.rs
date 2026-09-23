// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.3: finding fixed points of functions.

use ch01::sec_1_3::fixed_point;

fn main() {
    let cos_fixed = fixed_point(f64::cos, 1.0);
    println!("{cos_fixed}");
    // => 0.7390822985224024
    assert!((cos_fixed - 0.739_082_298_522_402_4).abs() < 1e-9);

    let solution = fixed_point(|y| y.sin() + y.cos(), 1.0);
    println!("{solution}");
    // => 1.2587315962971173
    assert!((solution - 1.258_731_596_297_117_3).abs() < 1e-9);
}
