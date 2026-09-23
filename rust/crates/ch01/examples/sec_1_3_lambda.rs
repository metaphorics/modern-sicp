// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.2: closures in place of named helper procedures.

use ch01::sec_1_1::square;
use ch01::sec_1_3::sum;

/// The book's `pi-sum`, with the term and the successor written as
/// closures and no helper definitions anywhere.
fn pi_sum(a: f64, b: f64) -> f64 {
    sum(&|x| 1.0 / (x * (x + 2.0)), a, &|x| x + 4.0, b)
}

/// The book's `integral`, with the step successor a closure instead of
/// the named `add-dx`.
fn integral(f: &dyn Fn(f64) -> f64, a: f64, b: f64, dx: f64) -> f64 {
    sum(f, a + dx / 2.0, &|x| x + dx, b) * dx
}

fn main() {
    let eight_times_pi_sum = 8.0 * pi_sum(1.0, 1000.0);
    println!("{eight_times_pi_sum}");
    // => 3.139592655589783
    assert!((eight_times_pi_sum - 3.139_592_655_589_783).abs() < 1e-9);

    println!("{}", integral(&|x| x * x * x, 0.0, 1.0, 0.01));
    // => 0.24998750000000042
    assert!((integral(&|x| x * x * x, 0.0, 1.0, 0.01) - 0.25).abs() < 0.0001);

    let combine = |x: f64, y: f64, z: f64| x + y + square(z);
    let value = combine(1.0, 2.0, 3.0);
    println!("{value}");
    // => 12
    assert!((value - 12.0).abs() < 1e-9);
}
