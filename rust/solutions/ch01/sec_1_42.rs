// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.42: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_42 {
    use ch01::sec_1_1::square;

    /// The book's `compose`: the function that computes `f(g(x))`. Both
    /// arguments are owned by the returned closure.
    fn compose(f: impl Fn(f64) -> f64, g: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
        move |x| f(g(x))
    }

    /// The book's `inc`.
    fn inc(x: f64) -> f64 {
        x + 1.0
    }

    /// Exercise 1.42: composition
    ///
    /// Returns `(compose square inc)(6)`, which is 49.
    pub fn ex_1_42() -> f64 {
        compose(square, inc)(6.0)
    }
}

#[test]
fn ex_1_42() {
    let value = ex_1_42::ex_1_42();
    assert!((value - 49.0).abs() < 1e-9);
}
