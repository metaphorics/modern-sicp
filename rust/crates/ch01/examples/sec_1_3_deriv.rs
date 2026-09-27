// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.4: Newton's method from `deriv` and `fixed-point`.

use ch01::sec_1_1::square;
use ch01::sec_1_3::newtons_method;

fn main() {
    let derivative = ch01::sec_1_3::deriv(|x| x * x * x);
    println!("{}", derivative(5.0));
    // => 75.00014999664018
    assert!((derivative(5.0) - 75.0).abs() < 0.001);

    let root = newtons_method(move |y| square(y) - 9.0, 1.0);
    println!("{root}");
    // => 3.000000000000002
    assert!((root - 3.0).abs() < 0.00001);
}
