// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.1: the `sum` abstraction.

use ch01::sec_1_3::{cube, identity, inc, sum};

/// The book's `sum-cubes` restated as a call to `sum`.
fn sum_cubes(a: f64, b: f64) -> f64 {
    sum(&cube, a, &inc, b)
}

/// The book's `sum-integers` restated as a call to `sum`.
fn sum_integers(a: f64, b: f64) -> f64 {
    sum(&identity, a, &inc, b)
}

/// The book's `pi-sum` restated with `pi-term` and `pi-next` as closures.
fn pi_sum(a: f64, b: f64) -> f64 {
    sum(&|x| 1.0 / (x * (x + 2.0)), a, &|x| x + 4.0, b)
}

fn main() {
    println!("{}", sum_cubes(1.0, 10.0));
    // => 3025
    assert!((sum_cubes(1.0, 10.0) - 3025.0).abs() < 1e-9);

    println!("{}", sum_integers(1.0, 10.0));
    // => 55
    assert!((sum_integers(1.0, 10.0) - 55.0).abs() < 1e-9);

    let eight_times_pi_sum = 8.0 * pi_sum(1.0, 1000.0);
    println!("{eight_times_pi_sum}");
    // => 3.139592655589783
    assert!((eight_times_pi_sum - 3.139_592_655_589_783).abs() < 1e-9);
}
