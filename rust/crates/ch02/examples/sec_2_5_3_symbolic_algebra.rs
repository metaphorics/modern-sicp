// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.5.3

//! Section 2.5.3: symbolic algebra. The polynomial package adds and
//! multiplies sparse term lists whose coefficients go through the generic
//! arithmetic package, so a polynomial over polynomials is just the
//! dispatch recursing one level down.

use ch02::sec_2_5::{
    add, install_generic_arithmetic, install_polynomial_is_zero, install_polynomial_package,
    make_polynomial, make_term, mul,
};
use sicp_runtime::{OpTable, Value};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table = Rc::new(OpTable::new());
    install_generic_arithmetic(&table)?;
    install_polynomial_package(&table);
    install_polynomial_is_zero(&table);

    // 5x^2 + 3x + 7 and x^2 - 1, in sparse term lists ordered from
    // highest to lowest order.
    let p1 = make_polynomial(
        &table,
        "x",
        &[
            make_term(2, Value::Int(5)),
            make_term(1, Value::Int(3)),
            make_term(0, Value::Int(7)),
        ],
    )?;
    let p2 = make_polynomial(
        &table,
        "x",
        &[make_term(1, Value::Int(1)), make_term(0, Value::Int(-1))],
    )?;
    let sum = add(&table, &p1, &p2)?;
    println!("{sum}");
    // => (polynomial (x (2 5) (1 4) (0 6)))
    assert_eq!(sum.to_string(), "(polynomial (x (2 5) (1 4) (0 6)))");

    // Multiplication combines same-order terms generically.
    let p3 = make_polynomial(
        &table,
        "x",
        &[make_term(1, Value::Int(1)), make_term(0, Value::Int(1))],
    )?;
    let product = mul(&table, &p3, &p3)?;
    println!("{product}");
    // => (polynomial (x (2 1) (1 2) (0 1)))
    assert_eq!(product.to_string(), "(polynomial (x (2 1) (1 2) (0 1)))");

    // Coefficients can be polynomials in another variable: the call to
    // combine them dispatches back into this same package.
    let y1 = make_polynomial(
        &table,
        "y",
        &[make_term(1, Value::Int(1)), make_term(0, Value::Int(1))],
    )?;
    let nested = make_polynomial(&table, "x", &[make_term(3, y1.clone()), make_term(0, y1)])?;
    let doubled = add(&table, &nested, &nested)?;
    println!("{doubled}");
    // => (polynomial (x (3 (polynomial (y (1 2) (0 2)))) (0 (polynomial (y (1 2) (0 2))))))
    assert_eq!(
        doubled.to_string(),
        "(polynomial (x (3 (polynomial (y (1 2) (0 2)))) (0 (polynomial (y (1 2) (0 2))))))"
    );
    let squared = mul(&table, &nested, &nested)?;
    assert!(
        squared
            .to_string()
            .contains("(6 (polynomial (y (2 1) (1 2) (0 1))))"),
        "{squared}"
    );

    // Rational numbers work as coefficients too, through the table.
    let half = ch02::sec_2_5::make_rational(&table, 1, 2)?;
    let p4 = make_polynomial(&table, "x", &[make_term(1, half)])?;
    let p5 = make_polynomial(
        &table,
        "x",
        &[
            make_term(1, ch02::sec_2_5::make_rational(&table, 1, 2)?),
            make_term(0, Value::Int(1)),
        ],
    )?;
    let rsum = add(&table, &p4, &p5)?;
    println!("{rsum}");
    // => (polynomial (x (1 (rational (1 . 1))) (0 1)))
    assert_eq!(
        rsum.to_string(),
        "(polynomial (x (1 (rational (1 . 1))) (0 1)))"
    );

    Ok(())
}
