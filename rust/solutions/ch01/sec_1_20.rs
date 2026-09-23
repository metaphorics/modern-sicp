// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.20: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_20 {
    /// The book's `gcd`, evaluated eagerly the way Rust evaluates every
    /// call: each argument is computed once, before the recursive call
    /// happens. `remainders` counts every computation of `a % b`.
    fn gcd_eager(a: u64, b: u64, remainders: &mut u64) -> u64 {
        if b == 0 {
            a
        } else {
            *remainders += 1;
            gcd_eager(b, a % b, remainders)
        }
    }

    /// Exercise 1.20: counting the remainders of the eager `gcd`
    ///
    /// Returns the value of `gcd(206, 40)` first, and the number of
    /// remainder computations the eager evaluation performs while
    /// producing it second. Eager (applicative-order) evaluation
    /// computes the four reductions `gcd(206,40) -> gcd(40,6) ->
    /// gcd(6,4) -> gcd(4,2) -> gcd(2,0)`, one remainder apiece: four
    /// calls, not the eighteen that normal order would spend
    /// re-evaluating unreduced argument expressions at every test (see
    /// the rationale).
    pub fn ex_1_20() -> (u64, u64) {
        let mut remainders = 0;
        let value = gcd_eager(206, 40, &mut remainders);
        (value, remainders)
    }
}

#[test]
fn ex_1_20() {
    let (value, remainders) = ex_1_20::ex_1_20();
    assert_eq!(value, 2);
    assert_eq!(remainders, 4);
}
