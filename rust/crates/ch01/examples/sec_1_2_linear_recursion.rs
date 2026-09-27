// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.2

//! Section 1.2.1: the two factorial processes, linear recursion and
//! linear iteration.

use ch01::sec_1_2::{fact_iter, factorial, factorial_iter};

fn main() {
    let six_factorial = factorial(6);
    println!("{six_factorial}");
    // => 720
    assert_eq!(six_factorial, 720);

    let six_factorial = factorial_iter(6);
    println!("{six_factorial}");
    // => 720
    assert_eq!(six_factorial, 720);

    let six_factorial = fact_iter(1, 1, 6);
    println!("{six_factorial}");
    // => 720
    assert_eq!(six_factorial, 720);
}
