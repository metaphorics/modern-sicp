// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.5.4

//! Section 3.5.4: Streams and delayed evaluation -- the integrand as a
//! delayed argument, the `solve` feedback loop that could not even be
//! built without it, and the approximation of e from `dy/dt = y`.

use ch03::sec_3_5::{integers, integral_delayed, solve, stream_map, stream_ref};

fn main() {
    // The solution of dy/dt = y with y(0) = 1, integrated at step
    // 0.001: its 1000th value is e, the book's 2.716924 at three
    // printed decimals.
    let e_approx = stream_ref(&solve(|y| y, 1.0, 0.001), 1000);
    println!("{e_approx}");
    // => 2.716923932235896
    assert!((e_approx - std::f64::consts::E).abs() < 0.002);
    println!("{e_approx:.3}");
    // => 2.717

    // The same loop exposed: the integrator's output feeds the map that
    // feeds the integrator, through one delayed argument.
    let growth = solve(|y| 0.5 * y, 2.0, 0.1);
    let first: Vec<f64> = growth.iter().take(5).collect();
    println!("{first:?}");
    // => [2.0, 2.1, 2.205, 2.3152500000000003, 2.4310125000000005]
    assert!((first[0] - 2.0).abs() < 1e-12);
    assert!((first[1] - 2.100_000_000_000_000_5).abs() < 1e-12);
    let doubled: Vec<f64> = stream_map(|x| x * 2.0, &growth).iter().take(2).collect();
    println!("{doubled:?}");
    // => [4.0, 4.200000000000001]
    assert!((doubled[0] - 4.0).abs() < 1e-12);

    // The delayed integral alone: integrating the half-integers at
    // dt = 0.5 starts with the initial value and adds a quarter, a
    // half, three quarters, ... only as the consumer walks.
    #[expect(
        clippy::cast_precision_loss,
        reason = "the integrand starts as small integers, exact in f64"
    )]
    let half_integers = stream_map(|x| *x as f64 * 0.5, &integers());
    let integrated = integral_delayed(move || half_integers.clone(), 0.0, 0.5);
    let prefix: Vec<f64> = integrated.iter().take(5).collect();
    println!("{prefix:?}");
    // => [0.0, 0.25, 0.75, 1.5, 2.5]
    assert_eq!(prefix.len(), 5);
    for (actual, expected) in prefix.iter().zip([0.0, 0.25, 0.75, 1.5, 2.5]) {
        assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
    }
    let thousandth = stream_ref(&integrated, 1000);
    println!("{thousandth}");
    // => 125125
    assert!((thousandth - 125_125.0).abs() < 1e-6);
}
