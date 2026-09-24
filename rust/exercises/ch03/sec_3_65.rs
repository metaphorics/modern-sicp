// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.65: the natural logarithm of 2
//! as `1 - 1/2 + 1/3 - ...`, computed as raw partial sums, Euler
//! transform, and recursively accelerated sequence, to compare how
//! rapidly the three approximation sequences converge.

mod ex_3_65 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.65`.
        pub exercise: &'static str,
    }

    /// Exercise 3.65: ln 2 approximation streams
    ///
    /// Answers element 10 of the raw partial sums, element 5 of the
    /// Euler transform, and element 5 of the recursively accelerated
    /// sequence, as `(raw_10, euler_5, accelerated_5)`.
    pub fn ex_3_65() -> Result<(f64, f64, f64), Pending> {
        Err(Pending { exercise: "3.65" })
    }
}

/// The double nearest the natural logarithm of 2.
use std::f64::consts::LN_2;

#[test]
#[ignore = "pending solution"]
fn ex_3_65() {
    let (raw_10, euler_5, accelerated_5) = ex_3_65::ex_3_65().expect("solved");
    // Element 10 of the raw sums errs by about 4.3e-2 (one digit);
    // element 5 of the Euler transform by about 2.5e-4 (three digits);
    // element 5 of the accelerated sequence by about 1.7e-9 (eight
    // digits). Each acceleration stage is orders of magnitude better.
    assert!((raw_10 - LN_2).abs() < 5.0e-2);
    assert!((euler_5 - LN_2).abs() < 1.0e-3);
    assert!((accelerated_5 - LN_2).abs() < 1.0e-8);
    assert!((raw_10 - LN_2).abs() > (euler_5 - LN_2).abs() * 100.0);
    assert!((euler_5 - LN_2).abs() > (accelerated_5 - LN_2).abs() * 1.0e4);
}
