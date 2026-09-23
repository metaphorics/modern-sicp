// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution tests of section 0.2: one test per
//! exercise with its statement in a doc comment, and shared code in
//! the matching src module.

/// The reference solution of exercise 0.1: the stub and the
/// exercise-named test share one module so both carry the exercise's
/// name.
mod ex_0_01 {
    /// Exercise 0.1: factorial by checked multiplication
    ///
    /// Computes `n!` recursively with `checked_mul`, returning `None`
    /// instead of wrapping when the exact result would overflow `i128`.
    pub fn factorial(n: u32) -> Option<i128> {
        if n == 0 {
            Some(1)
        } else {
            factorial(n - 1)?.checked_mul(i128::from(n))
        }
    }

    /// The largest `n` for which [`factorial`] still returns `Some`.
    pub fn largest_factorial_n() -> u32 {
        let mut n = 0;
        while factorial(n + 1).is_some() {
            n += 1;
        }
        n
    }
}

#[test]
fn ex_0_01() {
    assert_eq!(ex_0_01::factorial(5), Some(120));
    assert_eq!(ex_0_01::factorial(34), None);
    assert_eq!(ex_0_01::largest_factorial_n(), 33);
}

/// The reference solution of exercise 0.2: the stub and the
/// exercise-named test share one module so both carry the exercise's
/// name.
mod ex_0_02 {
    const COINS: [i64; 5] = [50, 25, 10, 5, 1];

    /// Exercise 0.2: change-counting, recursive and iterative
    ///
    /// Counts the ways to make `amount` cents from half-dollars,
    /// quarters, dimes, nickels, and pennies, by the book's recursive
    /// process (1.2.2): use the first kind of coin, or don't.
    pub fn count_change_recursive(amount: i64) -> i64 {
        cc(amount, COINS.len())
    }

    fn cc(amount: i64, kinds_of_coins: usize) -> i64 {
        if amount == 0 {
            1
        } else if amount < 0 || kinds_of_coins == 0 {
            0
        } else {
            cc(amount, kinds_of_coins - 1) + cc(amount - COINS[kinds_of_coins - 1], kinds_of_coins)
        }
    }

    /// The same count, by a loop over an accumulator table indexed by
    /// amount, one pass per denomination: an unbounded-coin combination
    /// count, the standard iterative reading of the recursion above.
    ///
    /// # Panics
    /// If `amount` is negative: every amount this book counts change for
    /// is a nonnegative cent count.
    #[expect(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        reason = "amount is asserted nonnegative above, and no book amount approaches usize::MAX"
    )]
    pub fn count_change_iterative(amount: i64) -> i64 {
        assert!(amount >= 0, "amount must be nonnegative");
        let mut ways = vec![0i64; amount as usize + 1];
        ways[0] = 1;
        for &coin in &COINS {
            for a in coin..=amount {
                ways[a as usize] += ways[(a - coin) as usize];
            }
        }
        ways[amount as usize]
    }

    /// The maximum call depth [`count_change_recursive`] reaches while
    /// counting change for `amount`, to compare against the loop's
    /// constant depth.
    pub fn recursive_depth(amount: i64) -> u32 {
        fn cc_depth(amount: i64, kinds_of_coins: usize, depth: u32, max_depth: &mut u32) -> i64 {
            *max_depth = (*max_depth).max(depth);
            if amount == 0 {
                1
            } else if amount < 0 || kinds_of_coins == 0 {
                0
            } else {
                cc_depth(amount, kinds_of_coins - 1, depth + 1, max_depth)
                    + cc_depth(
                        amount - COINS[kinds_of_coins - 1],
                        kinds_of_coins,
                        depth + 1,
                        max_depth,
                    )
            }
        }
        let mut max_depth = 0;
        cc_depth(amount, COINS.len(), 0, &mut max_depth);
        max_depth
    }
}

#[test]
fn ex_0_02() {
    assert_eq!(ex_0_02::count_change_recursive(100), 292);
    assert_eq!(ex_0_02::count_change_iterative(100), 292);
    assert_eq!(ex_0_02::count_change_iterative(400), 26_517);
    assert!(ex_0_02::recursive_depth(400) > 400);
}
