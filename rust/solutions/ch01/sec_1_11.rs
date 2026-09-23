// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.11: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_11 {
    /// `f` by a recursive process: the direct translation of the
    /// definition.
    fn f_rec(n: i64) -> i64 {
        if n < 3 {
            n
        } else {
            f_rec(n - 1) + 2 * f_rec(n - 2) + 3 * f_rec(n - 3)
        }
    }

    /// `f` by an iterative process: three state variables carry the last
    /// three values forward as a loop, in place of the recursive calls.
    fn f_iter(n: i64) -> i64 {
        if n < 3 {
            return n;
        }
        let (mut a, mut b, mut c) = (0, 1, 2);
        for _ in 3..=n {
            (a, b, c) = (b, c, c + 2 * b + 3 * a);
        }
        c
    }

    /// Exercise 1.11: `f` by a recursive process and by an iterative one
    ///
    /// Returns `f(8)` computed by the recursive process first and by the
    /// loop-spelled iterative process second.
    pub fn ex_1_11() -> [i64; 2] {
        [f_rec(8), f_iter(8)]
    }
}

#[test]
fn ex_1_11() {
    let values = ex_1_11::ex_1_11();
    assert_eq!(values, [335, 335]);
}
