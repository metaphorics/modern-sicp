// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.27: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_27 {
    /// `base^exp mod m` by successive squaring: the book's `expmod`.
    fn expmod(base: i128, exp: i128, m: i128) -> i128 {
        if exp == 0 {
            1
        } else if exp % 2 == 0 {
            let half = expmod(base, exp / 2, m);
            (half * half) % m
        } else {
            (base * expmod(base, exp - 1, m)) % m
        }
    }

    /// Whether `a^n` is congruent to `a` modulo `n` for every `a` less
    /// than `n`: an exhaustive Fermat check, in place of a single random
    /// witness.
    fn fools_fermat_test(n: i128) -> bool {
        (1..n).all(|a| expmod(a, n, n) == a)
    }

    /// Exercise 1.27: the Carmichael numbers fool the Fermat test
    ///
    /// Returns, for each of the six Carmichael numbers 561, 1105, 1729,
    /// 2465, 2821, and 6601, whether `a^n` is congruent to `a` modulo `n`
    /// for every `a` less than `n`.
    pub fn ex_1_27() -> [bool; 6] {
        [561, 1105, 1729, 2465, 2821, 6601].map(fools_fermat_test)
    }
}

#[test]
fn ex_1_27() {
    let fooled = ex_1_27::ex_1_27();
    assert_eq!(fooled, [true; 6]);
}
