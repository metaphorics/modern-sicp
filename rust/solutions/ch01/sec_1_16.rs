// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.16: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_16 {
    /// Iterative exponentiation by successive squaring: `a * b^n` is the
    /// invariant quantity, unchanged from state to state; `a` starts at
    /// 1 and holds the answer once `n` reaches 0.
    fn fast_expt_iter(base: i64, exponent: u64) -> i64 {
        let (mut a, mut b, mut n) = (1_i64, base, exponent);
        while n != 0 {
            if n.is_multiple_of(2) {
                b *= b;
                n /= 2;
            } else {
                a *= b;
                n -= 1;
            }
        }
        a
    }

    /// Exercise 1.16: iterative exponentiation by successive squaring
    ///
    /// Returns `2^10` and `3^7` as the loop-spelled invariant process
    /// produces them.
    pub fn ex_1_16() -> [i64; 2] {
        [fast_expt_iter(2, 10), fast_expt_iter(3, 7)]
    }
}

#[test]
fn ex_1_16() {
    let values = ex_1_16::ex_1_16();
    assert_eq!(values, [1024, 2187]);
}
