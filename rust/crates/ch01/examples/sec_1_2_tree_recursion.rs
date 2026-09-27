// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.2

//! Section 1.2.2: tree recursion and the counting-change example.

use ch01::sec_1_2::{count_change, fib, fib_iter};

fn main() {
    let fifth_fibonacci = fib(5);
    println!("{fifth_fibonacci}");
    // => 5
    assert_eq!(fifth_fibonacci, 5);

    let tenth_fibonacci = fib(10);
    println!("{tenth_fibonacci}");
    // => 55
    assert_eq!(tenth_fibonacci, 55);

    let tenth_fibonacci = fib_iter(10);
    println!("{tenth_fibonacci}");
    // => 55
    assert_eq!(tenth_fibonacci, 55);

    let ways_to_change_a_dollar = count_change(100);
    println!("{ways_to_change_a_dollar}");
    // => 292
    assert_eq!(ways_to_change_a_dollar, 292);
}
