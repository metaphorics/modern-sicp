// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.1: the three range procedures.

use ch01::sec_1_3::{pi_sum, sum_cubes, sum_integers};

fn main() {
    println!("{}", sum_integers(1.0, 10.0));
    // => 55
    assert!((sum_integers(1.0, 10.0) - 55.0).abs() < 1e-9);

    println!("{}", sum_cubes(1.0, 10.0));
    // => 3025
    assert!((sum_cubes(1.0, 10.0) - 3025.0).abs() < 1e-9);

    let eight_times_pi_sum = 8.0 * pi_sum(1.0, 1000.0);
    println!("{eight_times_pi_sum}");
    // => 3.139592655589783
    assert!((eight_times_pi_sum - 3.139_592_655_589_783).abs() < 1e-9);
}
