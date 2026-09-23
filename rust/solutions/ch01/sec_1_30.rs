// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.30: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_30 {
    use ch01::sec_1_3::{identity, inc};

    /// The book's `sum` as an iterative process: the running result and
    /// the current value of `a` are the state, and the loop pass is the
    /// step the recursive spelling took with one frame.
    fn sum_iter(term: &dyn Fn(f64) -> f64, mut a: f64, next: &dyn Fn(f64) -> f64, b: f64) -> f64 {
        let mut result = 0.0;
        while a <= b {
            result += term(a);
            a = next(a);
        }
        result
    }

    /// Exercise 1.30: `sum` as an iterative process
    ///
    /// Returns the iterative sum of the integers from 1 through 10.
    pub fn ex_1_30() -> f64 {
        sum_iter(&identity, 1.0, &inc, 10.0)
    }

    /// The iterative `sum` itself, for the agreement check against the
    /// book's recursive spelling.
    pub fn sum_iter_public(
        term: &dyn Fn(f64) -> f64,
        a: f64,
        next: &dyn Fn(f64) -> f64,
        b: f64,
    ) -> f64 {
        sum_iter(term, a, next, b)
    }
}

#[test]
fn ex_1_30() {
    assert!((ex_1_30::ex_1_30() - 55.0).abs() < 1e-9);

    // The loop spelling agrees with the recursive one on the section's
    // other running example, the sum of the cubes from 1 through 10.
    let inc = |x: f64| x + 1.0;
    let recursive = ch01::sec_1_3::sum(&ch01::sec_1_3::cube, 1.0, &inc, 10.0);
    let iterative = ex_1_30::sum_iter_public(&ch01::sec_1_3::cube, 1.0, &inc, 10.0);
    assert!((recursive - iterative).abs() < 1e-9);
    assert!((iterative - 3025.0).abs() < 1e-9);
}
