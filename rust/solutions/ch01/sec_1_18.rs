// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.18: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_18 {
    /// Iterative multiplication by doubling and halving: `a * b + c` is
    /// the invariant quantity, unchanged from state to state; `c` starts
    /// at 0 and holds the product once `b` reaches 0.
    fn fast_times_iter(base: i64, factor: i64) -> i64 {
        let (mut a, mut b, mut c) = (base, factor, 0_i64);
        while b != 0 {
            if b % 2 == 0 {
                a *= 2;
                b /= 2;
            } else {
                b -= 1;
                c += a;
            }
        }
        c
    }

    /// Exercise 1.18: iterative Russian peasant multiplication
    ///
    /// Returns `7 * 5`, `17 * 33`, and `0 * 9` as the loop-spelled
    /// invariant process produces them.
    pub fn ex_1_18() -> [i64; 3] {
        [
            fast_times_iter(7, 5),
            fast_times_iter(17, 33),
            fast_times_iter(0, 9),
        ]
    }
}

#[test]
fn ex_1_18() {
    let values = ex_1_18::ex_1_18();
    assert_eq!(values, [35, 561, 0]);
}
