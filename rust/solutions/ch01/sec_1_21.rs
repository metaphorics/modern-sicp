// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.21: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_21 {
    /// Whether `a` divides `b`: the book's `divides?`.
    fn divides(a: u64, b: u64) -> bool {
        b.is_multiple_of(a)
    }

    /// The smallest integral divisor greater than 1: the book's
    /// `smallest-divisor`, built on `find-divisor`'s square-root end
    /// test.
    fn smallest_divisor(n: u64) -> u64 {
        find_divisor(n, 2)
    }

    /// The book's `find-divisor`.
    fn find_divisor(n: u64, test_divisor: u64) -> u64 {
        if test_divisor * test_divisor > n {
            n
        } else if divides(test_divisor, n) {
            test_divisor
        } else {
            find_divisor(n, test_divisor + 1)
        }
    }

    /// Exercise 1.21: smallest divisors
    ///
    /// Returns the smallest divisors of 199, 1999, and 19999 in that
    /// order.
    pub fn ex_1_21() -> [u64; 3] {
        [
            smallest_divisor(199),
            smallest_divisor(1999),
            smallest_divisor(19999),
        ]
    }
}

#[test]
fn ex_1_21() {
    let values = ex_1_21::ex_1_21();
    assert_eq!(values, [199, 1999, 7]);
}
