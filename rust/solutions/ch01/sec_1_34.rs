// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.34: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_34 {
    use ch01::sec_1_1::square;

    /// The book's `f`: a procedure whose argument is itself a procedure.
    /// The parameter type is the plain function-pointer type, so any
    /// argument must be a procedure of one `f64` returning one `f64`.
    pub fn f(g: fn(f64) -> f64) -> f64 {
        g(2.0)
    }

    /// Exercise 1.34: applying a procedure to itself
    ///
    /// Returns the value of `f(square)` first and the value of
    /// `f` applied to the closure `z * (z + 1)` second.
    pub fn ex_1_34() -> (f64, f64) {
        (f(square), f(|z| z * (z + 1.0)))
    }
}

#[test]
fn ex_1_34() {
    let (of_square, of_closure) = ex_1_34::ex_1_34();
    assert!((of_square - 4.0).abs() < 1e-9);
    assert!((of_closure - 6.0).abs() < 1e-9);
}
