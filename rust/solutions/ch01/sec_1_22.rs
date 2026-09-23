// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.22: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_22 {
    use std::time::{Duration, Instant};

    /// Whether `a` divides `b`: the book's `divides?`.
    fn divides(a: u64, b: u64) -> bool {
        b.is_multiple_of(a)
    }

    /// The book's `smallest-divisor`.
    fn smallest_divisor(n: u64) -> u64 {
        find_divisor(n, 2)
    }

    /// The book's `find-divisor`.
    fn find_divisor(n: u64, test_divisor: u64) -> u64 {
        if test_divisor * test_divisor > n {
            n
        } else if divides(test_divisor, n) {
            test_divisor
        } else {
            find_divisor(n, test_divisor + 1)
        }
    }

    /// The book's `prime?`.
    fn is_prime(n: u64) -> bool {
        n == smallest_divisor(n)
    }

    /// The book's `report-prime`: what a caller would print once the
    /// test completes.
    fn report_prime(elapsed: Duration) -> Duration {
        elapsed
    }

    /// The book's `start-prime-test`: checks primality and, on success,
    /// hands the elapsed time to `report-prime`.
    fn start_prime_test(n: u64, start_time: Instant) -> Option<Duration> {
        is_prime(n).then(|| report_prime(start_time.elapsed()))
    }

    /// The book's `timed-prime-test`: `runtime` becomes `Instant::now`,
    /// Rust's monotonic clock.
    fn timed_prime_test(n: u64) -> Option<Duration> {
        start_prime_test(n, Instant::now())
    }

    /// Checks consecutive odd integers from `start` upward and returns
    /// the first `count` that are prime, each timed with
    /// `timed_prime_test`.
    fn search_for_primes(start: u64, count: usize) -> Vec<u64> {
        let mut candidate = if start.is_multiple_of(2) {
            start + 1
        } else {
            start
        };
        let mut found = Vec::with_capacity(count);
        while found.len() < count {
            if timed_prime_test(candidate).is_some() {
                found.push(candidate);
            }
            candidate += 2;
        }
        found
    }

    /// Exercise 1.22: the timed prime search
    ///
    /// Returns the three smallest primes larger than 1000, then the three
    /// smallest primes larger than 1,000,000.
    pub fn ex_1_22() -> [u64; 6] {
        let near_thousand = search_for_primes(1000, 3);
        let near_million = search_for_primes(1_000_000, 3);
        [
            near_thousand[0],
            near_thousand[1],
            near_thousand[2],
            near_million[0],
            near_million[1],
            near_million[2],
        ]
    }
}

#[test]
fn ex_1_22() {
    let values = ex_1_22::ex_1_22();
    assert_eq!(values, [1009, 1013, 1019, 1_000_003, 1_000_033, 1_000_037]);
}
