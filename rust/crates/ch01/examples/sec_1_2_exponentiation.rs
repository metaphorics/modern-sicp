// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.2

//! Section 1.2.4: exponentiation, from linear recursion to successive
//! squaring.

use ch01::sec_1_2::{expt, expt_iter, fast_expt};

/// The shape of `fast-expt`'s recursion, counting only the
/// multiplications each step would perform, never the value itself: `2^n`
/// for `n` in the thousands has far more digits than any fixed-width
/// integer holds, but the number of multiplications the algorithm needs
/// stays a small, exactly countable number.
fn fast_expt_multiplication_count(n: u64) -> u64 {
    if n == 0 {
        0
    } else if n.is_multiple_of(2) {
        1 + fast_expt_multiplication_count(n / 2)
    } else {
        1 + fast_expt_multiplication_count(n - 1)
    }
}

fn main() {
    let tenth_power = expt(2, 10);
    println!("{tenth_power}");
    // => 1024
    assert_eq!(tenth_power, 1024);

    let tenth_power = expt_iter(2, 10);
    println!("{tenth_power}");
    // => 1024
    assert_eq!(tenth_power, 1024);

    let tenth_power = fast_expt(2, 10);
    println!("{tenth_power}");
    // => 1024
    assert_eq!(tenth_power, 1024);

    let multiplications = fast_expt_multiplication_count(1000);
    println!("{multiplications}");
    // => 15
    assert_eq!(multiplications, 15);
}
