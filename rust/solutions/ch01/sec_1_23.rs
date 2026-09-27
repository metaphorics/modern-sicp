// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.23: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_23 {
    /// Whether `a` divides `b`: the book's `divides?`.
    fn divides(a: u64, b: u64) -> bool {
        b.is_multiple_of(a)
    }

    /// The book's `next`: 3 after 2, and the next odd integer after any
    /// odd `test-divisor`, so `find-divisor` stops testing even
    /// candidates once it has ruled out 2.
    fn next(test_divisor: u64) -> u64 {
        if test_divisor == 2 {
            3
        } else {
            test_divisor + 2
        }
    }

    /// The book's `find-divisor`, stepped by `next` instead of `+ 1`.
    fn find_divisor(n: u64, test_divisor: u64) -> u64 {
        if test_divisor * test_divisor > n {
            n
        } else if divides(test_divisor, n) {
            test_divisor
        } else {
            find_divisor(n, next(test_divisor))
        }
    }

    /// The book's `smallest-divisor`, using the `next`-stepped
    /// `find-divisor`.
    fn smallest_divisor(n: u64) -> u64 {
        find_divisor(n, 2)
    }

    /// The book's `prime?`, on the faster `smallest-divisor`.
    fn is_prime(n: u64) -> bool {
        n == smallest_divisor(n)
    }

    /// Checks consecutive odd integers from `start` upward and returns
    /// the first `count` that are prime.
    fn search_for_primes(start: u64, count: usize) -> Vec<u64> {
        let mut candidate = if start.is_multiple_of(2) {
            start + 1
        } else {
            start
        };
        let mut found = Vec::with_capacity(count);
        while found.len() < count {
            if is_prime(candidate) {
                found.push(candidate);
            }
            candidate += 2;
        }
        found
    }

    /// Exercise 1.23: skipping even divisors
    ///
    /// Returns the three smallest primes larger than 1,000,000 as the
    /// `next`-stepped `smallest_divisor` finds them: the same values
    /// `smallest_divisor` finds, by fewer divisor tests.
    pub fn ex_1_23() -> [u64; 3] {
        let found = search_for_primes(1_000_000, 3);
        [found[0], found[1], found[2]]
    }

    /// The 12 primes exercise 1.22 found across its four search ranges,
    /// stated as literal values rather than re-derived by
    /// `search_for_primes`: the exercise statement asks the `next`-stepped
    /// `smallest_divisor` to be run "on each of the 12 primes found in
    /// that exercise", not to search for primes independently.
    const PRIMES_FROM_EX_1_22: [u64; 12] = [
        1009, 1013, 1019, 10_007, 10_009, 10_037, 100_003, 100_019, 100_043, 1_000_003, 1_000_033,
        1_000_037,
    ];

    /// Runs the `next`-stepped `smallest_divisor` on each of the 12
    /// primes exercise 1.22 found, confirming the faster algorithm still
    /// reports every one of them prime, as the exercise statement asks.
    pub fn confirms_ex_1_22_primes() -> bool {
        PRIMES_FROM_EX_1_22.into_iter().all(is_prime)
    }
}

#[test]
fn ex_1_23() {
    let values = ex_1_23::ex_1_23();
    assert_eq!(values, [1_000_003, 1_000_033, 1_000_037]);
}

#[test]
fn ex_1_23_confirms_all_twelve_primes_from_ex_1_22() {
    assert!(ex_1_23::confirms_ex_1_22_primes());
}
