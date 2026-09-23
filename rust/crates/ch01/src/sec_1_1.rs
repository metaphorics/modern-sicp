// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.1

//! Section 1.1: The elements of programming.

/// Squares a number: the book's `square`.
pub fn square<T>(x: T) -> T
where
    T: Copy + std::ops::Mul<Output = T>,
{
    x * x
}

/// Sums the squares of two numbers: the book's `sum-of-squares`.
pub fn sum_of_squares<T>(x: T, y: T) -> T
where
    T: Copy + std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
{
    square(x) + square(y)
}

/// The book's `f`, the procedure whose application the substitution model
/// walks through.
#[must_use]
pub fn f(a: i64) -> i64 {
    sum_of_squares(a + 1, a * 2)
}

/// The absolute value of `x`: the book's `abs`, on floating-point numbers.
#[must_use]
pub fn abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

/// Averages two numbers: the book's `average`.
#[must_use]
pub fn average(x: f64, y: f64) -> f64 {
    f64::midpoint(x, y)
}

/// Improves a guess for a square root: the book's `improve`.
#[must_use]
pub fn improve(guess: f64, x: f64) -> f64 {
    average(guess, x / guess)
}

/// Whether a guess is within 0.001 of the radicand's square root: the book's
/// `good-enough?`.
#[must_use]
pub fn good_enough(guess: f64, x: f64) -> bool {
    abs(square(guess) - x) < 0.001
}

/// The book's `sqrt-iter`: improves the guess until it is good enough.
#[must_use]
pub fn sqrt_iter(guess: f64, x: f64) -> f64 {
    if good_enough(guess, x) {
        guess
    } else {
        sqrt_iter(improve(guess, x), x)
    }
}

/// The square root of `x` by Newton's method: the book's `sqrt`.
#[must_use]
pub fn sqrt(x: f64) -> f64 {
    sqrt_iter(1.0, x)
}

/// The block-structured `sqrt` of 1.1.8: the auxiliary functions are nested
/// inside the one function that uses them.
#[must_use]
pub fn sqrt_block(x: f64) -> f64 {
    fn good_enough(guess: f64, x: f64) -> bool {
        abs(square(guess) - x) < 0.001
    }

    fn improve(guess: f64, x: f64) -> f64 {
        average(guess, x / guess)
    }

    fn sqrt_iter(guess: f64, x: f64) -> f64 {
        if good_enough(guess, x) {
            guess
        } else {
            sqrt_iter(improve(guess, x), x)
        }
    }

    sqrt_iter(1.0, x)
}

/// The lexically scoped `sqrt` of 1.1.8: the helper closures read `x` from
/// the enclosing scope. A closure cannot call itself by name, so the
/// recursive driver stays a named function and carries the helpers with it.
#[must_use]
pub fn sqrt_lexical(x: f64) -> f64 {
    fn iter(guess: f64, good_enough: impl Fn(f64) -> bool, improve: impl Fn(f64) -> f64) -> f64 {
        if good_enough(guess) {
            guess
        } else {
            iter(improve(guess), good_enough, improve)
        }
    }

    let good_enough = |guess: f64| abs(square(guess) - x) < 0.001;
    let improve = |guess: f64| average(guess, x / guess);
    iter(1.0, good_enough, improve)
}
