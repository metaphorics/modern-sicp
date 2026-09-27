// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.8, listing: propagating a typed error with `?`.

use ch00::sec_0_8::{ArithError, safe_div};

fn main() -> Result<(), ArithError> {
    let quotient = safe_div(10, 2)?;
    println!("{quotient}");
    // => 5
    assert_eq!(quotient, 5);

    let failure = safe_div(1, 0);
    println!("{failure:?}");
    // => Err(DivideByZero)
    assert_eq!(failure, Err(ArithError::DivideByZero));
    Ok(())
}
