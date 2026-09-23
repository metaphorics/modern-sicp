// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.10: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_10 {
    /// Ackermann's function: the book's `A`.
    fn a(x: u64, y: u64) -> u64 {
        if y == 0 {
            0
        } else if x == 0 {
            2 * y
        } else if y == 1 {
            2
        } else {
            a(x - 1, a(x, y - 1))
        }
    }

    /// Exercise 1.10: Ackermann's function values
    ///
    /// Returns the values of `(A 1 10)`, `(A 2 4)`, and `(A 3 3)` in that
    /// order. `f(n) = A(0, n) = 2n`; `g(n) = A(1, n) = 2^n`;
    /// `h(n) = A(2, n)` is a tower of 2s `n` high (`h(n) = 2^h(n - 1)`,
    /// `h(0) = 0`).
    pub fn ex_1_10() -> [u64; 3] {
        [a(1, 10), a(2, 4), a(3, 3)]
    }
}

#[test]
fn ex_1_10() {
    let values = ex_1_10::ex_1_10();
    assert_eq!(values, [1024, 65_536, 65_536]);
}
