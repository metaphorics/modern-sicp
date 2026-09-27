// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.17: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_17 {
    /// The book's `*`, restated as repeated addition: a number of steps
    /// linear in `b`.
    fn times(a: i64, b: i64) -> i64 {
        if b == 0 { 0 } else { a + times(a, b - 1) }
    }

    /// Doubles an integer: the book's `double`.
    fn double(x: i64) -> i64 {
        x * 2
    }

    /// Halves an even integer: the book's `halve`.
    fn halve(x: i64) -> i64 {
        x / 2
    }

    /// Multiplication by doubling and halving, `fast-expt`'s shape:
    /// halve `b` while it is even, double `a` to match, and fall back to
    /// one addition when it is odd. A number of steps logarithmic in
    /// `b`.
    fn fast_times(a: i64, b: i64) -> i64 {
        if b == 0 {
            0
        } else if b % 2 == 0 {
            fast_times(double(a), halve(b))
        } else {
            a + fast_times(a, b - 1)
        }
    }

    /// Exercise 1.17: fast multiplication by doubling and halving
    ///
    /// Returns `7 * 5` as the linear-recursive procedure computes it
    /// first, and as the logarithmic procedure built from `double` and
    /// `halve` computes it second.
    pub fn ex_1_17() -> [i64; 2] {
        [times(7, 5), fast_times(7, 5)]
    }
}

#[test]
fn ex_1_17() {
    let values = ex_1_17::ex_1_17();
    assert_eq!(values, [35, 35]);
}
