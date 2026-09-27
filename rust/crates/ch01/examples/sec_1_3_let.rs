// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3.2: local bindings.

use ch01::sec_1_1::square;

/// The book's first spelling of `f`: an auxiliary procedure carries the
/// local names.
fn f_helper(x: f64, y: f64) -> f64 {
    fn f_from_locals(a: f64, b: f64, x: f64, y: f64) -> f64 {
        x * square(a) + y * b + a * b
    }
    f_from_locals(1.0 + x * y, 1.0 - y, x, y)
}

/// The book's second spelling: the auxiliary procedure becomes an
/// anonymous closure applied to the two values.
#[allow(clippy::redundant_closure_call)] // the point of this spelling is the immediately-invoked closure itself
fn f_closure(x: f64, y: f64) -> f64 {
    (|a: f64, b: f64| x * square(a) + y * b + a * b)(1.0 + x * y, 1.0 - y)
}

/// The book's third spelling: `let` bindings for the local names.
fn f_let(x: f64, y: f64) -> f64 {
    let a = 1.0 + x * y;
    let b = 1.0 - y;
    x * square(a) + y * b + a * b
}

fn main() {
    println!("{}", f_let(2.0, 4.0));
    // => 123
    assert!((f_helper(2.0, 4.0) - f_let(2.0, 4.0)).abs() < 1e-9);
    assert!((f_closure(2.0, 4.0) - f_let(2.0, 4.0)).abs() < 1e-9);

    let x = 5.0;
    let inner = {
        let x = 3.0;
        x + x * 10.0
    };
    println!("{}", inner + x);
    // => 38
    assert!((inner + x - 38.0_f64).abs() < 1e-9);

    // Scheme's `let` computes every value before any binding comes into
    // scope, so the `y` of `(let ((x 3) (y (+ x 2))) (* x y))` reads the
    // outer `x`. Rust's sequential `let`s shadow at once, so the same
    // two lines read the new `x`.
    let x = 2.0;
    let y_outside = x + 2.0;
    let x = 3.0;
    let y_inside = x + 2.0;
    println!("{}", 3.0 * y_outside);
    // => 12
    println!("{}", x * y_inside);
    // => 15
    assert!((3.0 * y_outside - 12.0_f64).abs() < 1e-9);
    assert!((x * y_inside - 15.0_f64).abs() < 1e-9);
}
