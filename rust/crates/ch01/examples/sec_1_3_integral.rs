// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.1: numerical integration with `sum`.

use ch01::sec_1_3::{cube, integral};

fn main() {
    println!("{}", integral(&cube, 0.0, 1.0, 0.01));
    // => 0.24998750000000042
    assert!((integral(&cube, 0.0, 1.0, 0.01) - 0.25).abs() < 0.0001);

    println!("{}", integral(&cube, 0.0, 1.0, 0.001));
    // => 0.249999875000001
    assert!((integral(&cube, 0.0, 1.0, 0.001) - 0.25).abs() < 0.00001);
}
