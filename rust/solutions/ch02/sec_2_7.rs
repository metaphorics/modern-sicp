// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.7: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_07 {
    /// The interval type this exercise completes: the constructor is
    /// given directly in the exercise statement, `cons(a, b)`; this
    /// module reimplements it locally so the exercise stays a real,
    /// self-contained answer, even though `ch02::sec_2_1::Interval`
    /// already ships the same two-line selectors as part of the
    /// section's representative program.
    struct Interval {
        lower: f64,
        upper: f64,
    }

    impl Interval {
        /// The interval constructor given in the statement of exercise
        /// 2.7: `(define (make-interval a b) (cons a b))`.
        fn new(a: f64, b: f64) -> Self {
            Interval { lower: a, upper: b }
        }

        /// Exercise 2.7's `lower-bound`: the first part of the pair.
        fn lower_bound(&self) -> f64 {
            self.lower
        }

        /// Exercise 2.7's `upper-bound`: the second part of the pair.
        fn upper_bound(&self) -> f64 {
            self.upper
        }
    }

    /// Exercise 2.7: the interval selectors that complete `make-interval`
    ///
    /// Returns the lower and upper bounds of the interval built from
    /// `6.0` and `8.0`.
    pub fn ex_2_07() -> (f64, f64) {
        let i = Interval::new(6.0, 8.0);
        (i.lower_bound(), i.upper_bound())
    }
}

#[test]
fn ex_2_07() {
    assert_eq!(ex_2_07::ex_2_07(), (6.0, 8.0));
}
