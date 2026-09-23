// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.3: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_03 {
    use ch01::sec_1_1::sum_of_squares;

    /// Exercise 1.3: the sum of the squares of the two larger of three
    /// numbers
    ///
    /// Names the largest argument first, then the larger of the two that
    /// remain, and sums their squares with the book's `sum-of-squares`.
    /// Ties need no special case: when arguments are equal, either
    /// ordering names the same two values.
    pub fn sum_squares_two_larger(a: i64, b: i64, c: i64) -> i64 {
        if a >= b && a >= c {
            sum_of_squares(a, b.max(c))
        } else if b >= c {
            sum_of_squares(b, a.max(c))
        } else {
            sum_of_squares(c, a.max(b))
        }
    }
}

#[test]
fn ex_1_03() {
    assert_eq!(ex_1_03::sum_squares_two_larger(1, 2, 3), 13);
    assert_eq!(ex_1_03::sum_squares_two_larger(2, 2, 2), 8);
    assert_eq!(ex_1_03::sum_squares_two_larger(-1, -2, -3), 5);
}
