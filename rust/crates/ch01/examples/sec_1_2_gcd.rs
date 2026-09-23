// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.2

//! Section 1.2.5: Euclid's Algorithm.

use ch01::sec_1_2::gcd;

fn main() {
    let common_divisor = gcd(206, 40);
    println!("{common_divisor}");
    // => 2
    assert_eq!(common_divisor, 2);
}
