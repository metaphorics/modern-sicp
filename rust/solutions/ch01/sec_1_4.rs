// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.4: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_04 {
    /// Adds two numbers: one of the two operator functions the branch may
    /// yield.
    pub fn add(a: i64, b: i64) -> i64 {
        a + b
    }

    /// Subtracts the second number from the first: the other operator
    /// function.
    pub fn sub(a: i64, b: i64) -> i64 {
        a - b
    }

    /// Exercise 1.4: `a + |b|`, built by letting a branch choose the
    /// operator function
    ///
    /// Rust's built-in operators are not values, so the branch yields a
    /// function instead: `add` for a positive `b`, `sub` (which is `a -
    /// b`, the same as `a + (-b)`) otherwise. Both branches are function
    /// items of one type, so the `if` binds `op` to one of them and the
    /// call below applies it.
    pub fn a_plus_abs_b(a: i64, b: i64) -> i64 {
        let op: fn(i64, i64) -> i64 = if b > 0 { add } else { sub };
        op(a, b)
    }
}

#[test]
fn ex_1_04() {
    assert_eq!(ex_1_04::a_plus_abs_b(1, 5), 6);
    assert_eq!(ex_1_04::a_plus_abs_b(1, -5), 6);
    assert_eq!(ex_1_04::a_plus_abs_b(2, 0), 2);
}
