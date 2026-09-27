// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.1.4

//! Section 2.1.4: extended exercise, interval arithmetic for Alyssa P.
//! Hacker's system of inexact quantities with known precision.

use ch02::sec_2_1::{Interval, add_interval, div_interval, mul_interval};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let six_to_eight = Interval::new(6.0, 8.0)?;
    let four_to_five = Interval::new(4.0, 5.0)?;

    let sum = add_interval(&six_to_eight, &four_to_five);
    println!("[{}, {}]", sum.lower_bound(), sum.upper_bound());
    // => [10, 13]
    assert_eq!((sum.lower_bound(), sum.upper_bound()), (10.0, 13.0));

    let product = mul_interval(&six_to_eight, &four_to_five);
    println!("[{}, {}]", product.lower_bound(), product.upper_bound());
    // => [24, 40]
    assert_eq!((product.lower_bound(), product.upper_bound()), (24.0, 40.0));

    // Bounds chosen as exact binary fractions, so the reciprocal and the
    // product below carry no floating-point rounding noise.
    let two_to_four = Interval::new(2.0, 4.0)?;
    let quotient = div_interval(&six_to_eight, &two_to_four);
    println!("[{}, {}]", quotient.lower_bound(), quotient.upper_bound());
    // => [1.5, 4]
    assert_eq!((quotient.lower_bound(), quotient.upper_bound()), (1.5, 4.0));

    // `make-center-width`, `center`, and `width`: engineers specify
    // tolerances as a center value and an allowed deviation, not raw
    // bounds.
    let resistor = Interval::from_center_width(7.0, 1.0)?;
    println!("{} +/- {}", resistor.center(), resistor.width());
    // => 7 +/- 1
    assert_eq!(resistor, six_to_eight);
    assert_eq!((resistor.center(), resistor.width()), (7.0, 1.0));

    // Ben's comment: dividing by an interval that spans zero is not
    // clearly defined. `div_interval` does not check for this yet
    // (exercise 2.10 adds the check); ordinary `f64` division never
    // panics, so the undefined case surfaces as a numeric answer that
    // looks plausible but is too narrow to be mathematically meaningful,
    // no wider than dividing by either half of `spans_zero` alone would
    // give.
    let spans_zero = Interval::new(-1.0, 1.0)?;
    let undefined = div_interval(&six_to_eight, &spans_zero);
    println!("[{}, {}]", undefined.lower_bound(), undefined.upper_bound());
    // => [-8, 8]
    assert_eq!(
        (undefined.lower_bound(), undefined.upper_bound()),
        (-8.0, 8.0)
    );

    Ok(())
}
