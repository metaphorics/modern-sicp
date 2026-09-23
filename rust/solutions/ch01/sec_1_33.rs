// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.33: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_33 {
    use ch01::sec_1_1::square;
    use ch01::sec_1_3::inc;

    /// The book's `filtered_accumulate`: folds only the terms whose
    /// range value passes `filter`, leaving the rest out entirely.
    fn filtered_accumulate(
        filter: &dyn Fn(f64) -> bool,
        combiner: &dyn Fn(f64, f64) -> f64,
        null_value: f64,
        term: &dyn Fn(f64) -> f64,
        a: f64,
        next: &dyn Fn(f64) -> f64,
        b: f64,
    ) -> f64 {
        let rest = if a > b {
            return null_value;
        } else {
            filtered_accumulate(filter, combiner, null_value, term, next(a), next, b)
        };
        if filter(a) {
            combiner(term(a), rest)
        } else {
            rest
        }
    }

    /// The book's `prime?` of 1.2.6, on the small range this exercise
    /// sums over; the range values are whole numbers, so the float
    /// remainder is exact.
    fn is_prime(n: f64) -> bool {
        if n < 2.0 {
            return false;
        }
        let mut d = 2.0;
        while d * d <= n {
            if n % d == 0.0 {
                return false;
            }
            d += 1.0;
        }
        true
    }

    /// The sum of the squares of the primes between `a` and `b`.
    fn sum_prime_squares(a: f64, b: f64) -> f64 {
        let add = |x: f64, y: f64| x + y;
        filtered_accumulate(&is_prime, &add, 0.0, &square, a, &inc, b)
    }

    /// The product of the positive integers below `n` that are relatively
    /// prime to `n`.
    fn product_relative_primes(n: f64) -> f64 {
        #[allow(clippy::float_cmp)]
        let relatively_prime = |i: f64| {
            let (mut a, mut b) = (i, n);
            while b != 0.0 {
                (a, b) = (b, a % b);
            }
            a == 1.0
        };
        let multiply = |x: f64, y: f64| x * y;
        let identity = |x: f64| x;
        let successor = |x: f64| x + 1.0;
        filtered_accumulate(
            &relatively_prime,
            &multiply,
            1.0,
            &identity,
            1.0,
            &successor,
            n - 1.0,
        )
    }

    /// Exercise 1.33: `filtered_accumulate`
    ///
    /// Returns the sum of the squares of the primes between 2 and 10
    /// first, and the product of the positive integers below 10 that are
    /// relatively prime to 10 second.
    pub fn ex_1_33() -> (f64, f64) {
        (sum_prime_squares(2.0, 10.0), product_relative_primes(10.0))
    }
}

#[test]
fn ex_1_33() {
    let (prime_squares, relative_primes) = ex_1_33::ex_1_33();
    assert!((prime_squares - 87.0).abs() < 1e-9);
    assert!((relative_primes - 189.0).abs() < 1e-9);
}

/// Exercise 1.33a (this edition): `filtered_accumulate` re-expressed with
/// Rust's `Iterator` adapters, generating the range with `successors` and
/// selecting, transforming, and combining terms with `filter`, `map`, and
/// `fold` instead of writing the range recursion by hand.
mod ex_1_33a {
    use ch01::sec_1_1::square;
    use ch01::sec_1_3::inc;

    /// `filtered_accumulate` built from iterator adapters: the range walks
    /// by `successors`, the predicate becomes `filter`, the term becomes
    /// `map`, and the combiner becomes `fold`.
    fn filtered_accumulate_fold(
        filter: impl Fn(f64) -> bool,
        combiner: impl Fn(f64, f64) -> f64,
        null_value: f64,
        term: impl Fn(f64) -> f64,
        a: f64,
        next: impl Fn(f64) -> f64,
        b: f64,
    ) -> f64 {
        std::iter::successors(Some(a), move |&x| Some(next(x)))
            .take_while(|&x| x <= b)
            .filter(|&x| filter(x))
            .map(term)
            .fold(null_value, |rest, term_value| combiner(term_value, rest))
    }

    /// The book's `prime?` of 1.2.6, duplicated from exercise 1.33's
    /// solution so this module stands on its own.
    fn is_prime(n: f64) -> bool {
        if n < 2.0 {
            return false;
        }
        let mut d = 2.0;
        while d * d <= n {
            if n % d == 0.0 {
                return false;
            }
            d += 1.0;
        }
        true
    }

    /// The sum of the squares of the primes between `a` and `b`, folded.
    fn sum_prime_squares(a: f64, b: f64) -> f64 {
        filtered_accumulate_fold(is_prime, |x, y| x + y, 0.0, square, a, inc, b)
    }

    /// The product of the positive integers below `n` that are relatively
    /// prime to `n`, folded.
    #[allow(clippy::float_cmp)] // gcd of small exact integers compares exactly
    fn product_relative_primes(n: f64) -> f64 {
        let relatively_prime = move |i: f64| {
            let (mut a, mut b) = (i, n);
            while b != 0.0 {
                (a, b) = (b, a % b);
            }
            a == 1.0
        };
        let identity = |x: f64| x;
        let successor = |x: f64| x + 1.0;
        filtered_accumulate_fold(
            relatively_prime,
            |x, y| x * y,
            1.0,
            identity,
            1.0,
            successor,
            n - 1.0,
        )
    }

    /// Exercise 1.33a: `filtered_accumulate` re-expressed with `fold`
    ///
    /// Returns the same two results as exercise 1.33, computed by the
    /// iterator-adapter spelling instead of the explicit recursion.
    pub fn ex_1_33a() -> (f64, f64) {
        (sum_prime_squares(2.0, 10.0), product_relative_primes(10.0))
    }
}

#[test]
fn ex_1_33a() {
    let (prime_squares, relative_primes) = ex_1_33a::ex_1_33a();
    assert!((prime_squares - 87.0).abs() < 1e-9);
    assert!((relative_primes - 189.0).abs() < 1e-9);
}
