// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.26: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_26 {
    /// The book's `expmod`, counted: `calls` increments once per call,
    /// and the recursive call is reused for both factors of the square,
    /// so the count grows as `Theta(log exp)`.
    fn expmod(base: i128, exp: i128, m: i128, calls: &mut u64) -> i128 {
        *calls += 1;
        if exp == 0 {
            1
        } else if exp % 2 == 0 {
            let half = expmod(base, exp / 2, m, calls);
            (half * half) % m
        } else {
            (base * expmod(base, exp - 1, m, calls)) % m
        }
    }

    /// Louis's `expmod`, counted: it calls itself twice per even step
    /// instead of reusing one result, which turns the halving recursion
    /// into two half-sized ones, `Theta(exp)` calls in total.
    fn louis_expmod(base: i128, exp: i128, m: i128, calls: &mut u64) -> i128 {
        *calls += 1;
        if exp == 0 {
            1
        } else if exp % 2 == 0 {
            (louis_expmod(base, exp / 2, m, calls) * louis_expmod(base, exp / 2, m, calls)) % m
        } else {
            (base * louis_expmod(base, exp - 1, m, calls)) % m
        }
    }

    /// Exercise 1.26: Louis's doubled recursion
    ///
    /// Returns the number of calls the proper `expmod` makes for
    /// `expmod(2, 1000, 251)` first, the number Louis's
    /// explicit-multiplication version makes second, and whether the two
    /// versions agree on the value third.
    pub fn ex_1_26() -> (u64, u64, bool) {
        let mut proper_calls = 0;
        let proper_value = expmod(2, 1000, 251, &mut proper_calls);
        let mut louis_calls = 0;
        let louis_value = louis_expmod(2, 1000, 251, &mut louis_calls);
        (proper_calls, louis_calls, proper_value == louis_value)
    }
}

#[test]
fn ex_1_26() {
    let (proper, louis, agree) = ex_1_26::ex_1_26();
    assert!(agree, "both versions must agree on the value");
    assert_eq!(proper, 16);
    assert_eq!(louis, 2023);
}
