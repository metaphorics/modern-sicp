// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.2: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_02 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.2`.
        pub exercise: &'static str,
    }

    /// Adds two numbers: one of the four primitives the call tree is built
    /// from.
    #[allow(dead_code)]
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    /// Subtracts the second number from the first.
    #[allow(dead_code)]
    pub fn sub(a: f64, b: f64) -> f64 {
        a - b
    }

    /// Multiplies two numbers.
    #[allow(dead_code)]
    pub fn mul(a: f64, b: f64) -> f64 {
        a * b
    }

    /// Divides the first number by the second.
    #[allow(dead_code)]
    pub fn div(a: f64, b: f64) -> f64 {
        a / b
    }

    /// Exercise 1.2: write the fraction as pure nested calls
    ///
    /// Builds the book's expression out of nothing but calls to [`add`],
    /// [`sub`], [`mul`], and [`div`], so the call nesting mirrors the
    /// expression tree, and returns the value the tree evaluates to.
    pub fn ex_1_02() -> Result<f64, Pending> {
        Err(Pending { exercise: "1.2" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_02() {
    let value = ex_1_02::ex_1_02().unwrap_or(f64::NAN);
    assert!((value - (-37.0 / 150.0)).abs() < 1e-12);
}
