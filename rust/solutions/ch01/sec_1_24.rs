// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.24: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_24 {
    use sicp_runtime::Random;
    use std::time::Instant;

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

    /// Draws the book's `(random (- n 1))` from the runtime's seeded
    /// generator, widened into `i128`.
    fn random_below(rng: &mut Random, n: i128) -> i128 {
        i128::from(rng.random(u64::try_from(n).unwrap_or(u64::MAX)))
    }

    /// The book's `fermat-test`: one witness `a` drawn from `1..n`.
    fn fermat_test(n: i128, rng: &mut Random) -> bool {
        let a = random_below(rng, n - 1) + 1;
        expmod(a, n, n) == a
    }

    /// The book's `fast-prime?`: the Fermat test run `times` times.
    fn fast_prime(n: i128, times: u32, rng: &mut Random) -> bool {
        (0..times).all(|_| fermat_test(n, rng))
    }

    /// The book's `timed-prime-test`, on `fast_prime` instead of trial
    /// division: `runtime` becomes `Instant::now`.
    fn timed_fast_prime_test(n: i128, times: u32, rng: &mut Random) -> Option<std::time::Duration> {
        let start = Instant::now();
        fast_prime(n, times, rng).then(|| start.elapsed())
    }

    /// Exercise 1.24: the timed Fermat test
    ///
    /// Returns whether the timed Fermat test reports every one of the 12
    /// primes found in exercise 1.22 as prime, drawing its witnesses from
    /// the seeded generator of 1.2.6.
    pub fn ex_1_24() -> bool {
        let primes: [i128; 12] = [
            1009, 1013, 1019, 10007, 10009, 10037, 100_003, 100_019, 100_043, 1_000_003, 1_000_033,
            1_000_037,
        ];
        let mut rng = Random::new(1).expect("seed 1 is nonzero, so this never fails");
        primes
            .into_iter()
            .all(|n| timed_fast_prime_test(n, 8, &mut rng).is_some())
    }
}

#[test]
fn ex_1_24() {
    let all_reported_prime = ex_1_24::ex_1_24();
    assert!(all_reported_prime);
}
