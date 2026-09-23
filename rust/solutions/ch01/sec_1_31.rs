// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.31: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_31 {
    use ch01::sec_1_3::{identity, inc};

    /// The book's `product`, the recursive spelling: the pending
    /// multiplication of each term waits for the recursion to return.
    fn product(term: &dyn Fn(f64) -> f64, a: f64, next: &dyn Fn(f64) -> f64, b: f64) -> f64 {
        if a > b {
            1.0
        } else {
            term(a) * product(term, next(a), next, b)
        }
    }

    /// The book's `product` as an iterative process, the loop spelling.
    fn product_iter(
        term: &dyn Fn(f64) -> f64,
        mut a: f64,
        next: &dyn Fn(f64) -> f64,
        b: f64,
    ) -> f64 {
        let mut result = 1.0;
        while a <= b {
            result *= term(a);
            a = next(a);
        }
        result
    }

    /// The factorial of `n` as the product of the identity over 1..=n.
    fn factorial(n: u32) -> f64 {
        product(&identity, 1.0, &inc, f64::from(n))
    }

    /// The Wallis product of the statement, over the first `terms`
    /// factors: the fractions pair up as `2/3 * 4/3`, `4/5 * 6/5`, and
    /// so on, which is one factor per `i` of
    /// `4 i (i + 1) / (2 i + 1)^2`, converging to pi over 4.
    fn wallis_pi(terms: u32) -> f64 {
        let factor = |i: f64| {
            let odd = 2.0 * i + 1.0;
            4.0 * i * (i + 1.0) / (odd * odd)
        };
        let successor = |x: f64| x + 1.0;
        4.0 * product_iter(&factor, 1.0, &successor, f64::from(terms))
    }

    /// Exercise 1.31: the `product` abstraction
    ///
    /// Returns the factorial of 10 computed with `product` first, and the
    /// Wallis estimate of pi computed with `product` second.
    pub fn ex_1_31() -> (f64, f64) {
        (factorial(10), wallis_pi(10_000))
    }

    /// Both spellings, for the agreement check of part b.
    pub fn spellings(
        term: &dyn Fn(f64) -> f64,
        a: f64,
        next: &dyn Fn(f64) -> f64,
        b: f64,
    ) -> (f64, f64) {
        (product(term, a, next, b), product_iter(term, a, next, b))
    }
}

#[test]
fn ex_1_31() {
    let (factorial_10, wallis_pi) = ex_1_31::ex_1_31();
    assert!((factorial_10 - 3_628_800.0).abs() < 1e-6);
    assert!((wallis_pi - std::f64::consts::PI).abs() < 0.01);

    // Part b: the two processes compute the same product.
    let successor = |x: f64| x + 1.0;
    let (recursive, iterative) = ex_1_31::spellings(&|x| x * x, 1.0, &successor, 10.0);
    assert!((recursive - iterative).abs() < 1e-9);
}
