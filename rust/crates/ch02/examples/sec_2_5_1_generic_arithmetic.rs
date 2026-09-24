// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.5.1

//! Section 2.5.1: generic arithmetic operations. One table carries the
//! ordinary, rational, real, and complex packages; `add` and friends
//! dispatch on the argument tags alone.

use ch02::sec_2_5::{
    add, div, install_generic_arithmetic, make_complex_from_mag_ang, make_complex_from_real_imag,
    make_rational, make_real, make_scheme_number, mul, sub,
};
use sicp_runtime::OpTable;

#[expect(
    clippy::many_single_char_names,
    reason = "mirrors the book's a, b, x, y, z, w notation across the tower's levels"
)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table = OpTable::new();
    install_generic_arithmetic(&table)?;

    // Ordinary numbers dispatch to the package keyed by
    // (scheme-number scheme-number).
    let a = make_scheme_number(&table, 10)?;
    let b = make_scheme_number(&table, 5)?;
    println!("{}", add(&table, &a, &b)?);
    // => 15
    assert_eq!(add(&table, &a, &b)?.to_string(), "15");
    println!("{}", mul(&table, &a, &b)?);
    // => 50
    assert_eq!(mul(&table, &a, &b)?.to_string(), "50");

    // Rational numbers dispatch to their own package; the arithmetic
    // reduces to lowest terms.
    let r1 = make_rational(&table, 1, 2)?;
    let r2 = make_rational(&table, 1, 3)?;
    println!("{}", add(&table, &r1, &r2)?);
    // => (rational (5 . 6))
    assert_eq!(add(&table, &r1, &r2)?.to_string(), "(rational (5 . 6))");
    println!("{}", div(&table, &r1, &r2)?);
    // => (rational (3 . 2))
    assert_eq!(div(&table, &r1, &r2)?.to_string(), "(rational (3 . 2))");

    // Reals are the tower's third level.
    let x = make_real(&table, 2.5)?;
    let y = make_real(&table, 0.25)?;
    println!("{}", mul(&table, &x, &y)?);
    // => (real 0.625)
    assert_eq!(mul(&table, &x, &y)?.to_string(), "(real 0.625)");

    // Complex numbers are a two-level tagged datum: the outer complex
    // tag, then the rectangular or polar tag inside.
    let z = make_complex_from_real_imag(&table, 3.0, 4.0)?;
    let w = make_complex_from_mag_ang(&table, 1.0, std::f64::consts::PI)?;
    println!("{z}");
    // => (complex (rectangular (3 . 4)))
    assert_eq!(z.to_string(), "(complex (rectangular (3 . 4)))");
    println!("{}", add(&table, &z, &z)?);
    // => (complex (rectangular (6 . 8)))
    assert_eq!(
        add(&table, &z, &z)?.to_string(),
        "(complex (rectangular (6 . 8)))"
    );
    let sum = sub(&table, &z, &w)?;
    assert_eq!(ch02::sec_2_5::type_tag(&sum)?.as_ref(), "complex");
    let prod = mul(&table, &z, &z)?;
    println!("{prod}");
    // => (complex (polar (25 . 1.8545904360032244)))
    assert_eq!(
        prod.to_string(),
        "(complex (polar (25 . 1.8545904360032244)))"
    );

    Ok(())
}
