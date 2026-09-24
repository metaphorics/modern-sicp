// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.5.2

//! Section 2.5.2: combining data of different types. The coercion table
//! lets `add` take an ordinary number and a complex number without any
//! explicit cross-type entry: the ordinary number is coerced to a complex
//! number with zero imaginary part, and the complex package does the
//! work.

use ch02::sec_2_5::{
    add, apply_generic, get_coercion, install_generic_arithmetic, make_complex_from_real_imag,
    put_coercion, scheme_number_to_complex,
};
use sicp_runtime::{OpTable, SchemeError, Value};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let table = OpTable::new();
    install_generic_arithmetic(&table)?;

    // Before any coercion is installed, mixing the types misses.
    let z = make_complex_from_real_imag(&table, 1.0, 2.0)?;
    let miss = add(&table, &Value::Int(3), &z).expect_err("no mixed entry yet");
    println!("{miss}");
    // => No method for these types (add (scheme-number complex))
    assert!(miss.to_string().contains("No method for these types"));

    // The coercion table: an ordinary number becomes a complex number.
    put_coercion(
        &table,
        "scheme-number",
        "complex",
        Rc::new(|args: &[Value]| scheme_number_to_complex(&args[0])),
    );
    assert!(get_coercion(&table, "scheme-number", "complex").is_some());
    assert!(get_coercion(&table, "complex", "scheme-number").is_none());

    // The same generic call now goes through: coerce, then add.
    let sum = add(&table, &Value::Int(3), &z)?;
    println!("{sum}");
    // => (complex (rectangular (4 . 2)))
    assert_eq!(sum.to_string(), "(complex (rectangular (4 . 2)))");

    // Coercion runs whichever way fills the slot: here the second
    // argument is the ordinary number.
    let sum = add(&table, &z, &Value::Int(4))?;
    println!("{sum}");
    // => (complex (rectangular (5 . 2)))
    assert_eq!(sum.to_string(), "(complex (rectangular (5 . 2)))");

    // With no coercion between two types and no direct entry,
    // apply_generic gives up naming the operation and the tag list.
    let err = apply_generic(
        &table,
        "exp",
        &[
            make_complex_from_real_imag(&table, 1.0, 0.0)?,
            make_complex_from_real_imag(&table, 1.0, 0.0)?,
        ],
    );
    match err {
        Err(SchemeError::UserRaised { message, .. }) => {
            println!("{message}");
            // => No method for these types
            assert_eq!(message, "No method for these types");
        }
        other => return Err(format!("expected a method miss, got {other:?}").into()),
    }

    Ok(())
}
