// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 1.3

//! Section 1.3: Formulating abstractions with higher-order procedures.
//!
//! The book's procedures that take procedures as arguments arrive here as
//! parameters of type [`dyn Fn`](std::ops::Fn), and the book's procedures
//! that return procedures leave as closures that own what they capture.
//! Rust performs no tail-call optimization, so the recursive spellings
//! that remain recurse only to the depth the tolerances bound.

use std::rc::Rc;

use sicp_runtime::{SicpError, Value};

use crate::sec_1_1::{average, square};

/// The tolerance of the section's fixed-point search: two successive
/// guesses closer than this stop the search.
pub const TOLERANCE: f64 = 0.00001;

/// The step of the numerical derivative [`deriv`].
pub const DX: f64 = 0.00001;

/// Cubes a number: the book's `cube`.
#[must_use]
pub fn cube(x: f64) -> f64 {
    x * x * x
}

/// Adds one to its argument: the book's `inc`.
#[must_use]
pub fn inc(x: f64) -> f64 {
    x + 1.0
}

/// Returns its argument unchanged: the book's `identity`.
#[must_use]
pub fn identity(x: f64) -> f64 {
    x
}

/// Sums the integers from `a` through `b`: the book's `sum-integers`, the
/// first of the three range procedures of 1.3.1.
#[must_use]
pub fn sum_integers(a: f64, b: f64) -> f64 {
    if a > b {
        0.0
    } else {
        a + sum_integers(a + 1.0, b)
    }
}

/// Sums the cubes of the integers from `a` through `b`: the book's
/// `sum-cubes`, the second of the three range procedures.
#[must_use]
pub fn sum_cubes(a: f64, b: f64) -> f64 {
    if a > b {
        0.0
    } else {
        cube(a) + sum_cubes(a + 1.0, b)
    }
}

/// Sums the series `1/1*3 + 1/5*7 + 1/9*11 + ...` from `a` through `b`:
/// the book's `pi-sum`, eight times whose sum over 1 through 1000
/// approximates pi.
#[must_use]
pub fn pi_sum(a: f64, b: f64) -> f64 {
    if a > b {
        0.0
    } else {
        1.0 / (a * (a + 2.0)) + pi_sum(a + 4.0, b)
    }
}

/// The book's `sum`: the general summation of `term(a)`, `term(next(a))`,
/// and so on up to `b`.
#[must_use]
pub fn sum(term: &dyn Fn(f64) -> f64, a: f64, next: &dyn Fn(f64) -> f64, b: f64) -> f64 {
    if a > b {
        0.0
    } else {
        term(a) + sum(term, next(a), next, b)
    }
}

/// The book's `integral`: the numerical integral of `f` between `a` and
/// `b` in steps of `dx`, sampled at the step midpoints.
#[must_use]
pub fn integral(f: &dyn Fn(f64) -> f64, a: f64, b: f64, dx: f64) -> f64 {
    sum(f, a + dx / 2.0, &|x| x + dx, b) * dx
}

/// Whether two points are within the 0.001 tolerance of the half-interval
/// method: the book's `close-enough?`.
#[must_use]
pub fn close_enough(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}

/// The book's `search`: the bisection of the interval between points
/// where `f` takes values of opposite sign.
#[must_use]
pub fn search(f: &dyn Fn(f64) -> f64, neg_point: f64, pos_point: f64) -> f64 {
    let midpoint = average(neg_point, pos_point);
    if close_enough(neg_point, pos_point) {
        midpoint
    } else {
        let test_value = f(midpoint);
        if test_value > 0.0 {
            search(f, neg_point, midpoint)
        } else if test_value < 0.0 {
            search(f, midpoint, pos_point)
        } else {
            midpoint
        }
    }
}

/// The book's `half-interval-method`: finds a root of `f` between points
/// where its values have opposite sign, and signals the book's error when
/// the signs agree.
///
/// # Errors
/// Returns [`SicpError::UserRaised`] when the values of `f` at `a` and
/// `b` are not of opposite sign, the book's error case.
pub fn half_interval_method(f: impl Fn(f64) -> f64, a: f64, b: f64) -> Result<f64, SicpError> {
    let a_value = f(a);
    let b_value = f(b);
    if a_value < 0.0 && b_value > 0.0 {
        Ok(search(&f, a, b))
    } else if b_value < 0.0 && a_value > 0.0 {
        Ok(search(&f, b, a))
    } else {
        Err(SicpError::UserRaised {
            message: "Values are not of opposite sign".to_owned(),
            irritants: vec![Value::Real(a), Value::Real(b)],
        })
    }
}

/// The book's `fixed-point`: applies `f` repeatedly until two successive
/// values are within [`TOLERANCE`].
#[must_use]
pub fn fixed_point(f: impl Fn(f64) -> f64, first_guess: f64) -> f64 {
    fn try_guess(f: &dyn Fn(f64) -> f64, guess: f64) -> f64 {
        let next = f(guess);
        if (guess - next).abs() < TOLERANCE {
            next
        } else {
            try_guess(f, next)
        }
    }
    try_guess(&f, first_guess)
}

/// The book's `average-damp`: returns a function that averages `x` with
/// `f(x)`, the damping transform that tames 1.3.3's oscillating search.
pub fn average_damp(f: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
    move |x| average(x, f(x))
}

/// The dynamically dispatched form of [`average_damp`], passable as a
/// function value to [`fixed_point_of_transform`].
#[must_use]
pub fn average_damp_dyn(f: Box<dyn Fn(f64) -> f64>) -> Box<dyn Fn(f64) -> f64> {
    Box::new(move |x| average(x, f(x)))
}

/// The book's `deriv`: the numerical derivative of `g` with step [`DX`].
pub fn deriv(g: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
    move |x| (g(x + DX) - g(x)) / DX
}

/// The transform passed to [`fixed_point_of_transform`]: Rust function
/// values carry no type parameters, so the transform is the dynamically
/// dispatched form.
pub type Transform = fn(Box<dyn Fn(f64) -> f64>) -> Box<dyn Fn(f64) -> f64>;

/// The book's `newton-transform`: `x` mapped to `x - g(x) / Dg(x)`, whose
/// fixed points solve `g(x) = 0`. The transform needs `g` twice, so the
/// function is shared through one `Rc` cell.
pub fn newton_transform(g: impl Fn(f64) -> f64 + 'static) -> impl Fn(f64) -> f64 {
    let shared: Rc<dyn Fn(f64) -> f64> = Rc::new(g);
    let for_deriv = Rc::clone(&shared);
    let dg = deriv(move |x| for_deriv(x));
    move |x| x - shared(x) / dg(x)
}

/// The dynamically dispatched form of [`newton_transform`].
#[must_use]
pub fn newton_transform_dyn(g: Box<dyn Fn(f64) -> f64>) -> Box<dyn Fn(f64) -> f64> {
    let shared: Rc<dyn Fn(f64) -> f64> = Rc::from(g);
    let for_deriv = Rc::clone(&shared);
    let dg = deriv(move |x| for_deriv(x));
    Box::new(move |x| x - shared(x) / dg(x))
}

/// The book's `newtons-method`: the zero of `g` as a fixed point of the
/// [`newton_transform`] of `g`.
#[must_use]
pub fn newtons_method(g: impl Fn(f64) -> f64 + 'static, guess: f64) -> f64 {
    fixed_point(newton_transform(g), guess)
}

/// The square root of `x` as a fixed-point search damped by
/// [`average_damp`]: the book's reformulated `sqrt` of 1.3.3.
#[must_use]
pub fn sqrt(x: f64) -> f64 {
    fixed_point(average_damp(move |y| x / y), 1.0)
}

/// The cube root of `x` as a fixed-point search damped once: the book's
/// `cube-root`.
#[must_use]
pub fn cube_root(x: f64) -> f64 {
    fixed_point(average_damp(move |y| x / square(y)), 1.0)
}

/// The square root of `x` by Newton's method: the final spelling of the
/// section, composed from [`newtons_method`].
#[must_use]
pub fn sqrt_newton(x: f64) -> f64 {
    newtons_method(move |y| square(y) - x, 1.0)
}

/// The book's `fixed-point-of-transform`: finds a fixed point of the
/// transform of `g`. Rust function values carry no type parameters, so
/// the transform takes and returns the dynamically dispatched form.
#[must_use]
pub fn fixed_point_of_transform(
    g: Box<dyn Fn(f64) -> f64>,
    transform: Transform,
    guess: f64,
) -> f64 {
    fixed_point(transform(g), guess)
}
