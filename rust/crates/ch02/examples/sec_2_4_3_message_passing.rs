// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.4.3

//! Section 2.4.3, message passing: the table decomposed into columns.
//! The data object itself answers operation names, and the generic
//! procedure is one line.

use ch02::sec_2_4::{apply_generic_message_passing, make_from_real_imag_message_passing};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let z = make_from_real_imag_message_passing(3.0, 4.0);

    println!("{}", apply_generic_message_passing("real-part", &z)?);
    // => 3
    assert_eq!(
        apply_generic_message_passing("real-part", &z)?.to_string(),
        "3"
    );
    println!("{}", apply_generic_message_passing("imag-part", &z)?);
    // => 4
    assert_eq!(
        apply_generic_message_passing("imag-part", &z)?.to_string(),
        "4"
    );
    println!("{}", apply_generic_message_passing("magnitude", &z)?);
    // => 5
    assert_eq!(
        apply_generic_message_passing("magnitude", &z)?.to_string(),
        "5"
    );
    println!("{}", apply_generic_message_passing("angle", &z)?);
    // => 0.9272952180016122
    assert_eq!(
        apply_generic_message_passing("angle", &z)?.to_string(),
        "0.9272952180016122"
    );

    // A message the object does not answer is the book's error case.
    let err = apply_generic_message_passing("rotation", &z).expect_err("unanswered message");
    println!("{err}");
    // => Unknown op: MAKE-FROM-REAL-IMAG rotation
    assert_eq!(err.to_string(), "Unknown op: MAKE-FROM-REAL-IMAG rotation");

    Ok(())
}
