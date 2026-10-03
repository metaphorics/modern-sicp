// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.4.1

//! Section 2.4.1: the complex-number arithmetic, written once against the
//! generic selectors, over representations from both designers. The
//! generic interface itself is the table-backed one of 2.4.3; the book
//! states 2.4.1's selectors as assumed primitives and supplies them later
//! in the section, and the example runs only once they exist.

use std::f64::consts::FRAC_PI_2;

use ch02::sec_2_4::{
    add_complex, angle, div_complex, imag_part, install_polar_package, install_rectangular_package,
    magnitude, make_from_mag_ang, make_from_real_imag, mul_complex, real_part, sub_complex,
};
use sicp_runtime::{OpTable, SicpError, Value};

/// The real number a selector produced, as the example's error type.
fn real_value(v: Result<Value, SicpError>) -> Result<f64, Box<dyn std::error::Error>> {
    match v {
        Ok(Value::Real(x)) => Ok(x),
        other => Err(format!("expected a real: {other:?}").into()),
    }
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table = OpTable::new();
    install_rectangular_package(&table);
    install_polar_package(&table);

    // Ben's constructor builds rectangular numbers; Alyssa's builds polar
    // ones; the arithmetic never learns which is which.
    let z = make_from_real_imag(&table, 3.0, 4.0)?;
    println!("{z}");
    // => (rectangular (3 . 4))
    assert_eq!(z.to_string(), "(rectangular (3 . 4))");

    let w = make_from_mag_ang(&table, 1.0, FRAC_PI_2)?;
    println!("{w}");
    // => (polar (1 . 1.5707963267948966))
    assert_eq!(w.to_string(), "(polar (1 . 1.5707963267948966))");

    // Either selector serves either number: magnitude of a rectangular
    // number, real part of a polar one.
    println!("{}", real_value(magnitude(&table, &z))?);
    // => 5
    assert!(close(real_value(magnitude(&table, &z))?, 5.0));

    println!("{}", real_value(angle(&table, &z))?);
    // => 0.9272952180016122
    assert!(close(
        real_value(angle(&table, &z))?,
        0.927_295_218_001_612_2
    ));

    println!("{}", real_value(real_part(&table, &w))?);
    // => 0.00000000000000006123233995736766
    assert!(close(real_value(real_part(&table, &w))?, 0.0));

    println!("{}", real_value(imag_part(&table, &w))?);
    // => 1
    assert!(close(real_value(imag_part(&table, &w))?, 1.0));

    // The two constructors reproduce any number from its own parts, the
    // property the book states before choosing representations.
    let round_trip = make_from_real_imag(
        &table,
        real_value(real_part(&table, &z))?,
        real_value(imag_part(&table, &z))?,
    )?;
    println!("{round_trip}");
    // => (rectangular (3 . 4))
    assert_eq!(round_trip.to_string(), "(rectangular (3 . 4))");

    let round_trip = make_from_mag_ang(
        &table,
        real_value(magnitude(&table, &w))?,
        real_value(angle(&table, &w))?,
    )?;
    println!("{round_trip}");
    // => (polar (1 . 1.5707963267948966))
    assert_eq!(round_trip.to_string(), "(polar (1 . 1.5707963267948966))");

    // The four arithmetic procedures, each written once, over a
    // rectangular and a polar operand in the same call.
    let sum = add_complex(&table, &z, &w)?;
    println!(
        "{} + {} = real {} + i*{}",
        z,
        w,
        real_value(real_part(&table, &sum))?,
        real_value(imag_part(&table, &sum))?
    );
    // => (rectangular (3 . 4)) + (polar (1 . 1.5707963267948966)) = real 3 + i*5
    assert!(close(real_value(real_part(&table, &sum))?, 3.0));
    assert!(close(real_value(imag_part(&table, &sum))?, 5.0));

    let difference = sub_complex(&table, &sum, &z)?;
    println!(
        "{} - {} = real {} + i*{}",
        sum,
        z,
        real_value(real_part(&table, &difference))?,
        real_value(imag_part(&table, &difference))?
    );
    // => real 0 + i*1
    assert!(close(real_value(real_part(&table, &difference))?, 0.0));
    assert!(close(real_value(imag_part(&table, &difference))?, 1.0));

    let product = mul_complex(&table, &z, &w)?;
    println!(
        "{} * {} = magnitude {}, angle {}",
        z,
        w,
        real_value(magnitude(&table, &product))?,
        real_value(angle(&table, &product))?
    );
    // => ... * ... = magnitude 5, angle 2.498091544796509
    assert!(close(real_value(magnitude(&table, &product))?, 5.0));
    assert!(close(
        real_value(angle(&table, &product))?,
        0.927_295_218_001_612_2 + FRAC_PI_2
    ));

    let quotient = div_complex(&table, &z, &w)?;
    println!(
        "{} / {} = magnitude {}, angle {}",
        z,
        w,
        real_value(magnitude(&table, &quotient))?,
        real_value(angle(&table, &quotient))?
    );
    // => ... / ... = magnitude 5, angle -0.6435011087932844
    assert!(close(real_value(magnitude(&table, &quotient))?, 5.0));
    assert!(close(
        real_value(angle(&table, &quotient))?,
        0.927_295_218_001_612_2 - FRAC_PI_2
    ));

    Ok(())
}
