// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.2

//! Section 1.2: Procedures and the processes they generate.
//!
//! The book's iterative processes are spelled as loops: Rust performs no
//! tail-call optimization, so a recursive spelling of a linear iteration
//! would grow the stack by one frame per step for no reason.

use sicp_runtime::Random;

/// The book's `factorial` as a linear recursive process: each pending
/// multiplication is a deferred operation on the stack.
#[must_use]
pub fn factorial(n: u64) -> i128 {
    if n <= 1 {
        1
    } else {
        i128::from(n) * factorial(n - 1)
    }
}

/// The book's `factorial` entry point for the linear iteration: it hands
/// the initial state to [`fact_iter`].
#[must_use]
pub fn factorial_iter(n: u64) -> i128 {
    fact_iter(1, 1, n)
}

/// The book's `fact-iter`: `product`, `counter`, and `max_count` are the
/// state variables, and the tail call of the Scheme form has become the
/// next pass of a `while` loop.
#[must_use]
pub fn fact_iter(mut product: i128, mut counter: u64, max_count: u64) -> i128 {
    while counter <= max_count {
        product *= i128::from(counter);
        counter += 1;
    }
    product
}

/// The book's `fib`, the direct tree-recursive translation of the
/// definition of the Fibonacci sequence.
#[must_use]
pub fn fib(n: u64) -> i128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

/// The book's `fib` entry point for the linear iteration: it hands the
/// initial state to [`fib_iter`].
#[must_use]
pub fn fib_iter(n: u64) -> i128 {
    fib_iter_state(1, 0, n)
}

/// The book's `fib-iter`: `a` and `b` walk the sequence while `count`
/// counts down, as a loop.
#[must_use]
pub fn fib_iter_state(mut a: i128, mut b: i128, mut count: u64) -> i128 {
    while count != 0 {
        (a, b) = (a + b, a);
        count -= 1;
    }
    b
}

/// Counts the ways to change `amount` with the five US coins: the book's
/// `count-change`.
#[must_use]
pub fn count_change(amount: i64) -> i64 {
    cc(amount, 5)
}

/// The book's `cc`: either use a coin of the first kind or do not.
fn cc(amount: i64, kinds_of_coins: i64) -> i64 {
    if amount == 0 {
        1
    } else if amount < 0 || kinds_of_coins == 0 {
        0
    } else {
        cc(amount, kinds_of_coins - 1)
            + cc(amount - first_denomination(kinds_of_coins), kinds_of_coins)
    }
}

/// The book's `first-denomination`: US denominations, half-dollar first.
#[must_use]
pub fn first_denomination(kinds_of_coins: i64) -> i64 {
    match kinds_of_coins {
        1 => 1,
        2 => 5,
        3 => 10,
        4 => 25,
        _ => 50,
    }
}

/// The book's `expt`, a linear recursive process.
#[must_use]
pub fn expt(b: i128, n: u64) -> i128 {
    if n == 0 { 1 } else { b * expt(b, n - 1) }
}

/// The book's `expt` entry point for the linear iteration.
#[must_use]
pub fn expt_iter(b: i128, n: u64) -> i128 {
    let (mut counter, mut product) = (n, 1_i128);
    while counter != 0 {
        product *= b;
        counter -= 1;
    }
    product
}

/// The book's `fast-expt`: successive squaring, a number of steps that
/// grows logarithmically with the exponent.
#[must_use]
pub fn fast_expt(b: i128, n: u64) -> i128 {
    if n == 0 {
        1
    } else if n.is_multiple_of(2) {
        let half = fast_expt(b, n / 2);
        half * half
    } else {
        b * fast_expt(b, n - 1)
    }
}

/// The book's `gcd`: Euclid's Algorithm. The Scheme form's tail call has
/// become a `while` loop over the two state variables; the process is the
/// iterative one either way, and the number of steps grows as the
/// logarithm of the numbers involved.
#[must_use]
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// The book's `smallest-divisor`: the smallest integral divisor greater
/// than 1.
#[must_use]
pub fn smallest_divisor(n: u64) -> u64 {
    find_divisor(n, 2)
}

/// The book's `find-divisor`: successive integers against the square-root
/// end test.
fn find_divisor(n: u64, test_divisor: u64) -> u64 {
    if test_divisor * test_divisor > n {
        n
    } else if divides(test_divisor, n) {
        test_divisor
    } else {
        find_divisor(n, test_divisor + 1)
    }
}

/// The book's `divides?`: whether `a` divides `b`.
#[must_use]
pub fn divides(a: u64, b: u64) -> bool {
    b.is_multiple_of(a)
}

/// The book's `prime?`: a number is prime if and only if it is its own
/// smallest divisor.
#[must_use]
pub fn is_prime(n: u64) -> bool {
    n == smallest_divisor(n)
}

/// The book's `expmod`: `base` to the `exp` power modulo `m` by
/// successive squaring, so the intermediates never leave the neighborhood
/// of `m`. Residues are carried in `i128`, whose square fits while `m`
/// stays below about `10^19`.
#[must_use]
pub fn expmod(base: i128, exp: i128, m: i128) -> i128 {
    if exp == 0 {
        1
    } else if exp % 2 == 0 {
        let half = expmod(base, exp / 2, m);
        (half * half) % m
    } else {
        (base * expmod(base, exp - 1, m)) % m
    }
}

/// Draws the book's `(random (- n 1))` on section-sized numbers: the
/// seeded generator of the runtime reduced into `0..n`. Section bounds
/// never leave the `u64` range, so the widening saturates instead of
/// failing.
fn random_below(rng: &mut Random, n: i128) -> i128 {
    i128::from(rng.random(u64::try_from(n).unwrap_or(u64::MAX)))
}

/// The book's `fermat-test`: one witness `a` drawn from `1..n` and checked
/// against Fermat's Little Theorem.
///
/// # Panics
/// Panics when `n <= 1`, where the witness range is empty; callers test
/// primality of numbers greater than 1.
#[must_use]
pub fn fermat_test(n: i128, rng: &mut Random) -> bool {
    fn try_it(a: i128, n: i128) -> bool {
        expmod(a, n, n) == a
    }

    try_it(random_below(rng, n - 1) + 1, n)
}

/// The book's `fast-prime?`: the Fermat test run `times` times. The
/// Scheme form's tail call has become a loop.
#[must_use]
pub fn fast_prime(n: i128, times: u32, rng: &mut Random) -> bool {
    (0..times).all(|_| fermat_test(n, rng))
}
