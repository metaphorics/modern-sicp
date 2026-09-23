// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.28: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_28 {
    /// The Miller-Rabin `expmod`: whenever squaring reveals a nontrivial
    /// square root of 1 (a residue other than 1 or `m - 1` whose square
    /// is 1 modulo `m`), it returns 0, which can never be the true
    /// residue of a power coprime to `m`.
    fn expmod(base: i128, exp: i128, m: i128) -> i128 {
        if exp == 0 {
            1
        } else if exp % 2 == 0 {
            let half = expmod(base, exp / 2, m);
            let squared = (half * half) % m;
            if squared == 1 && half != 1 && half != m - 1 {
                0
            } else {
                squared
            }
        } else {
            (base * expmod(base, exp - 1, m)) % m
        }
    }

    /// The Miller-Rabin test for one witness `a`: `n` is declared prime
    /// by this witness when `a^(n - 1)` reduces to 1 without ever
    /// exposing a nontrivial square root of 1 along the way.
    fn miller_rabin_test(n: i128, a: i128) -> bool {
        expmod(a, n - 1, n) == 1
    }

    /// Whether every witness from 1 up to `n - 1` reports `n` prime.
    fn all_witnesses_pass(n: i128) -> bool {
        (1..n).all(|a| miller_rabin_test(n, a))
    }

    /// A plain trial-division primality check, used only to build the
    /// list of primes below 100 to test the Miller-Rabin witnesses
    /// against.
    fn is_prime(n: i128) -> bool {
        n > 1 && (2..n).take_while(|d| d * d <= n).all(|d| n % d != 0)
    }

    /// Exercise 1.28: the Miller-Rabin test
    ///
    /// Returns whether every prime below 100 passes the Miller-Rabin test
    /// for every witness first, and whether every one of the six
    /// Carmichael numbers is rejected by at least one witness second.
    pub fn ex_1_28() -> (bool, bool) {
        let primes_pass = (2..100).filter(|&n| is_prime(n)).all(all_witnesses_pass);
        let carmichaels_rejected = [561, 1105, 1729, 2465, 2821, 6601]
            .into_iter()
            .all(|n| !all_witnesses_pass(n));
        (primes_pass, carmichaels_rejected)
    }
}

#[test]
fn ex_1_28() {
    let (primes_pass, carmichaels_rejected) = ex_1_28::ex_1_28();
    assert!(primes_pass);
    assert!(carmichaels_rejected);
}
