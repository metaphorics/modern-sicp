// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.32: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_32 {
    use ch01::sec_1_3::{identity, inc};

    /// The book's `accumulate`, the recursive spelling: `combiner` folds
    /// each term into the accumulation of the terms after it.
    fn accumulate(
        combiner: &dyn Fn(f64, f64) -> f64,
        null_value: f64,
        term: &dyn Fn(f64) -> f64,
        a: f64,
        next: &dyn Fn(f64) -> f64,
        b: f64,
    ) -> f64 {
        if a > b {
            null_value
        } else {
            combiner(
                term(a),
                accumulate(combiner, null_value, term, next(a), next, b),
            )
        }
    }

    /// The book's `accumulate` as an iterative process, the loop
    /// spelling: the accumulation walks from the null value up.
    fn accumulate_iter(
        combiner: &dyn Fn(f64, f64) -> f64,
        null_value: f64,
        term: &dyn Fn(f64) -> f64,
        mut a: f64,
        next: &dyn Fn(f64) -> f64,
        b: f64,
    ) -> f64 {
        let mut result = null_value;
        while a <= b {
            result = combiner(term(a), result);
            a = next(a);
        }
        result
    }

    /// The book's `sum`, defined as a call to `accumulate`.
    fn sum(a: f64, b: f64) -> f64 {
        let add = |x: f64, y: f64| x + y;
        accumulate(&add, 0.0, &identity, a, &inc, b)
    }

    /// The book's `product`, defined as a call to `accumulate`.
    fn product(a: f64, b: f64) -> f64 {
        let multiply = |x: f64, y: f64| x * y;
        accumulate(&multiply, 1.0, &identity, a, &inc, b)
    }

    /// Exercise 1.32: the `accumulate` abstraction
    ///
    /// Returns the recursive spelling's sum of 1 through 10, its product
    /// for the factorial of 10, and then the loop spelling's values for
    /// the same two.
    pub fn ex_1_32() -> (f64, f64, f64, f64) {
        let add = |x: f64, y: f64| x + y;
        let multiply = |x: f64, y: f64| x * y;
        (
            sum(1.0, 10.0),
            product(1.0, 10.0),
            accumulate_iter(&add, 0.0, &identity, 1.0, &inc, 10.0),
            accumulate_iter(&multiply, 1.0, &identity, 1.0, &inc, 10.0),
        )
    }
}

#[test]
fn ex_1_32() {
    let (sum_rec, product_rec, sum_loop, product_loop) = ex_1_32::ex_1_32();
    assert!((sum_rec - 55.0).abs() < 1e-9);
    assert!((product_rec - 3_628_800.0).abs() < 1e-6);
    assert!((sum_loop - 55.0).abs() < 1e-9);
    assert!((product_loop - 3_628_800.0).abs() < 1e-6);
}
