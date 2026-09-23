// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.2: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_02 {
    /// Adds two numbers: one of the four primitives the call tree is built
    /// from.
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    /// Subtracts the second number from the first.
    pub fn sub(a: f64, b: f64) -> f64 {
        a - b
    }

    /// Multiplies two numbers.
    pub fn mul(a: f64, b: f64) -> f64 {
        a * b
    }

    /// Divides the first number by the second.
    pub fn div(a: f64, b: f64) -> f64 {
        a / b
    }

    /// Exercise 1.2: write the fraction as pure nested calls
    ///
    /// The call nesting mirrors the expression tree: the outer `div`
    /// separates numerator and denominator, `add` and `sub` build the
    /// running sums and differences inside, and `mul` flattens the
    /// product `3(6-2)(2-7)` left to right.
    pub fn ex_1_02() -> f64 {
        div(
            add(add(5.0, 4.0), sub(2.0, sub(3.0, add(6.0, div(4.0, 5.0))))),
            mul(mul(3.0, sub(6.0, 2.0)), sub(2.0, 7.0)),
        )
    }
}

#[test]
fn ex_1_02() {
    let value = ex_1_02::ex_1_02();
    assert!((value - (-37.0 / 150.0)).abs() < 1e-12);
}
