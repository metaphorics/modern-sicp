// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.3: finding roots by the half-interval method.

use ch01::sec_1_3::half_interval_method;

fn main() {
    match half_interval_method(f64::sin, 2.0, 4.0) {
        Ok(pi_approx) => {
            println!("{pi_approx}");
            // => 3.14111328125
            assert!((pi_approx - 3.141_113_281_25).abs() < 1e-9);
        }
        Err(err) => println!("Error: {err}"),
    }

    match half_interval_method(|x| x * x * x - 2.0 * x - 3.0, 1.0, 2.0) {
        Ok(root) => {
            println!("{root}");
            // => 1.89306640625
            assert!((root - 1.893_066_406_25).abs() < 1e-9);
        }
        Err(err) => println!("Error: {err}"),
    }
}
