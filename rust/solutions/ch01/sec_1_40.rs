// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.40: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

use ch01::sec_1_3::newtons_method;

mod ex_1_40 {
    use ch01::sec_1_3::newtons_method;

    /// The book's `cubic`: the closure that computes
    /// `x^3 + a x^2 + b x + c`, ready for `newtons_method`.
    pub fn cubic(a: f64, b: f64, c: f64) -> impl Fn(f64) -> f64 {
        move |x| x * x * x + a * x * x + b * x + c
    }

    /// Exercise 1.40: `cubic` for Newton's method
    ///
    /// Returns the zero that `newtons_method` finds for the cubic
    /// `x^3 + x^2 - 2`, whose root is 1.
    pub fn ex_1_40() -> f64 {
        newtons_method(cubic(1.0, 0.0, -2.0), 1.0)
    }
}

#[test]
fn ex_1_40() {
    let zero = ex_1_40::ex_1_40();
    assert!((zero - 1.0).abs() < 1e-5);

    // A second spelling end to end: the zero of `x^3 - 8` is 2, reached
    // from the same guess of 1.
    let eight = newtons_method(ex_1_40::cubic(0.0, 0.0, -8.0), 1.0);
    assert!((eight - 2.0).abs() < 1e-5);
}
