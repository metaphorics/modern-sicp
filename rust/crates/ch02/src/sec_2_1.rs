// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.1

//! Section 2.1: Introduction to data abstraction.
//!
//! Constructors and selectors become Rust structs with private fields and
//! methods, so the abstraction barriers of @ref{Figure 2.1} are enforced
//! at compile time: code outside this module cannot reach a [`Rational`]
//! or [`Interval`]'s fields at all, let alone build one that skips the
//! constructor's invariant.

use std::fmt;
use std::rc::Rc;

use sicp_runtime::SicpError;

// ---------------------------------------------------------------------
// Pairs
// ---------------------------------------------------------------------

/// A pair of two values: this section's `cons` cell. The fields are
/// private, so [`car`] and [`cdr`] are the only way to reach them.
#[derive(Clone, Debug)]
pub struct Pair<A, B> {
    first: A,
    second: B,
}

/// Builds a pair: the book's `cons`.
#[must_use]
pub fn cons<A, B>(first: A, second: B) -> Pair<A, B> {
    Pair { first, second }
}

/// The first element of a pair: the book's `car`.
#[must_use]
pub fn car<A: Clone, B>(pair: &Pair<A, B>) -> A {
    pair.first.clone()
}

/// The second element of a pair: the book's `cdr`.
#[must_use]
pub fn cdr<A, B: Clone>(pair: &Pair<A, B>) -> B {
    pair.second.clone()
}

// ---------------------------------------------------------------------
// Rational numbers
// ---------------------------------------------------------------------

/// A rational number, reduced to lowest terms by its constructor: the
/// book's rational-number package. The fields are private, so `numer` and
/// `denom` are the only way in, exactly the "public use" surface of
/// Figure 2.1.
///
/// This constructor reduces the arguments with [`gcd`] but does not yet
/// normalize the sign onto the numerator; exercise 2.1 fixes that.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rational {
    num: i128,
    den: i128,
}

impl Rational {
    /// Builds a rational number in lowest terms by dividing both
    /// arguments by their gcd before constructing the pair.
    ///
    /// # Errors
    /// [`SicpError::DivisionByZero`] when `den` is zero.
    /// [`SicpError::Overflow`] when reducing the arguments overflows
    /// `i128`.
    pub fn new(num: i128, den: i128) -> Result<Self, SicpError> {
        if den == 0 {
            return Err(SicpError::DivisionByZero);
        }
        let num_abs = num.checked_abs().ok_or(SicpError::Overflow)?;
        let den_abs = den.checked_abs().ok_or(SicpError::Overflow)?;
        let g = gcd(num_abs, den_abs);
        let num = num.checked_div(g).ok_or(SicpError::Overflow)?;
        let den = den.checked_div(g).ok_or(SicpError::Overflow)?;
        Ok(Rational { num, den })
    }

    /// The numerator: the book's `numer`.
    #[must_use]
    pub fn numer(&self) -> i128 {
        self.num
    }

    /// The denominator: the book's `denom`.
    #[must_use]
    pub fn denom(&self) -> i128 {
        self.den
    }

    /// Adds two rationals: `(n1*d2 + n2*d1) / (d1*d2)`.
    ///
    /// # Errors
    /// [`SicpError::Overflow`] when the cross-multiplication or the
    /// reduction overflows.
    pub fn add(&self, other: &Self) -> Result<Self, SicpError> {
        let num = cross_add(self.num, self.den, other.num, other.den)?;
        let den = self.den.checked_mul(other.den).ok_or(SicpError::Overflow)?;
        Self::new(num, den)
    }

    /// Subtracts two rationals: `(n1*d2 - n2*d1) / (d1*d2)`.
    ///
    /// # Errors
    /// [`SicpError::Overflow`] when the cross-multiplication or the
    /// reduction overflows.
    pub fn sub(&self, other: &Self) -> Result<Self, SicpError> {
        let num = cross_sub(self.num, self.den, other.num, other.den)?;
        let den = self.den.checked_mul(other.den).ok_or(SicpError::Overflow)?;
        Self::new(num, den)
    }

    /// Multiplies two rationals: the book's `mul-rat`.
    ///
    /// # Errors
    /// [`SicpError::Overflow`] when either product overflows.
    pub fn mul(&self, other: &Self) -> Result<Self, SicpError> {
        let num = self.num.checked_mul(other.num).ok_or(SicpError::Overflow)?;
        let den = self.den.checked_mul(other.den).ok_or(SicpError::Overflow)?;
        Self::new(num, den)
    }

    /// Divides two rationals: the book's `div-rat`.
    ///
    /// # Errors
    /// [`SicpError::Overflow`] when either product overflows.
    /// [`SicpError::DivisionByZero`] when `other` is zero.
    pub fn div(&self, other: &Self) -> Result<Self, SicpError> {
        let num = self.num.checked_mul(other.den).ok_or(SicpError::Overflow)?;
        let den = self.den.checked_mul(other.num).ok_or(SicpError::Overflow)?;
        Self::new(num, den)
    }
}

impl fmt::Display for Rational {
    /// Prints as `numerator/denominator`: the book's `print-rat`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

/// Computes `a * d + c * b`, checked: the numerator of `add-rat`.
fn cross_add(a: i128, b: i128, c: i128, d: i128) -> Result<i128, SicpError> {
    let ad = a.checked_mul(d).ok_or(SicpError::Overflow)?;
    let cb = c.checked_mul(b).ok_or(SicpError::Overflow)?;
    ad.checked_add(cb).ok_or(SicpError::Overflow)
}

/// Computes `a * d - c * b`, checked: the numerator of `sub-rat`.
fn cross_sub(a: i128, b: i128, c: i128, d: i128) -> Result<i128, SicpError> {
    let ad = a.checked_mul(d).ok_or(SicpError::Overflow)?;
    let cb = c.checked_mul(b).ok_or(SicpError::Overflow)?;
    ad.checked_sub(cb).ok_or(SicpError::Overflow)
}

/// Euclid's Algorithm on nonnegative `i128`: this section's own copy of
/// 1.2.5's `gcd` (chapter crates do not depend on one another). Callers
/// pass a nonzero `b`, so the loop always terminates with a positive
/// result.
#[must_use]
fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

// ---------------------------------------------------------------------
// What is meant by data: procedural pairs
// ---------------------------------------------------------------------

/// A pair built entirely from a dispatch procedure, with no data
/// structure behind it: the book's answer to "what is meant by data"
/// (2.1.3). Both parts share one Rust type, since the dispatch closure's
/// return type is fixed once; the fully dynamic version, where the two
/// parts may differ, arrives with `Value` in section 2.4.
pub struct ProcPair<T>(Rc<dyn Fn(u8) -> Result<T, SicpError>>);

/// Builds a procedural pair: a dispatch closure answering `x` for
/// message `0`, `y` for message `1`, and an error for anything else.
#[must_use]
pub fn cons_proc<T: Clone + 'static>(x: T, y: T) -> ProcPair<T> {
    ProcPair(Rc::new(move |m| match m {
        0 => Ok(x.clone()),
        1 => Ok(y.clone()),
        other => Err(SicpError::UnknownMessage(other)),
    }))
}

/// The book's procedural `car`: applies the dispatch procedure to 0.
///
/// # Errors
/// Never, for a `z` built by [`cons_proc`]; the `Result` is part of the
/// dispatch procedure's own signature, since any [`ProcPair`] answers only
/// 0 and 1.
pub fn car_proc<T>(z: &ProcPair<T>) -> Result<T, SicpError> {
    (z.0)(0)
}

/// The book's procedural `cdr`: applies the dispatch procedure to 1.
///
/// # Errors
/// Never, for a `z` built by [`cons_proc`]; see [`car_proc`].
pub fn cdr_proc<T>(z: &ProcPair<T>) -> Result<T, SicpError> {
    (z.0)(1)
}

// ---------------------------------------------------------------------
// Church numerals (exercise 2.6's setting)
// ---------------------------------------------------------------------

/// One church-numeral application step: an effect on a running counter,
/// standing in for the arbitrary function `f` a numeral applies. Public,
/// and `Clone`, so exercise 2.6 can compose new numerals that apply one
/// step more than once.
#[derive(Clone)]
pub struct Step(Rc<dyn Fn(&mut i128)>);

impl Step {
    /// Builds a step from an arbitrary effect on the counter: exercise
    /// 2.6's `one` and `two` use this to define numerals directly,
    /// without going through [`church_succ`].
    #[must_use]
    pub fn from_fn(f: impl Fn(&mut i128) + 'static) -> Self {
        Step(Rc::new(f))
    }

    /// Applies the step's effect to the counter.
    pub fn apply(&self, x: &mut i128) {
        (self.0)(x);
    }
}

/// A Church numeral: a function from one step to the step of applying it
/// one more time. The wrapper struct breaks the type cycle a bare `type`
/// alias cannot, because `Church` must both accept and produce a `Step`.
/// `Clone`, so exercise 2.6's `+` can hold onto both of its arguments
/// inside the numeral it returns.
#[derive(Clone)]
pub struct Church(Rc<dyn Fn(Step) -> Step>);

impl Church {
    /// Builds a numeral from its own step-transformer body: exercise
    /// 2.6's `one`, `two`, and `+` use this to define numerals directly.
    #[must_use]
    pub fn from_fn(f: impl Fn(Step) -> Step + 'static) -> Self {
        Church(Rc::new(f))
    }

    /// Applies the numeral to one step, producing the step of applying it
    /// one more time.
    #[must_use]
    pub fn apply(&self, f: Step) -> Step {
        (self.0)(f)
    }

    /// Converts a numeral to an ordinary integer, for testing and
    /// display: applies it to a step that increments a counter, starting
    /// from zero.
    #[must_use]
    pub fn to_i128(&self) -> i128 {
        let mut count: i128 = 0;
        let counted = self.apply(Step::from_fn(|x: &mut i128| *x += 1));
        counted.apply(&mut count);
        count
    }
}

/// Church-numeral zero: applies its argument step zero times, so the
/// initial state returns unchanged.
#[must_use]
pub fn church_zero() -> Church {
    Church::from_fn(|_f: Step| Step::from_fn(|_x: &mut i128| {}))
}

/// The successor of a numeral: one more application of the step than
/// the numeral itself performs.
#[must_use]
pub fn church_succ(n: &Church) -> Church {
    let n = n.clone();
    Church::from_fn(move |f: Step| {
        let rest = n.apply(f.clone());
        Step::from_fn(move |x: &mut i128| {
            f.apply(x);
            rest.apply(x);
        })
    })
}

// ---------------------------------------------------------------------
// Interval arithmetic (2.1.4)
// ---------------------------------------------------------------------

/// A closed interval `[lower, upper]`: Alyssa's interval-arithmetic
/// package. The constructor given in the statement of exercise 2.7
/// enforces `lower <= upper`, so every interval this module hands out is
/// well formed; `width` (below) can therefore assume a nonnegative
/// result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    lower: f64,
    upper: f64,
}

impl Interval {
    /// Builds an interval from its two bounds: the book's
    /// `make-interval`, given directly in the statement of exercise 2.7.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] when `lower` is greater than `upper`.
    pub fn new(lower: f64, upper: f64) -> Result<Self, SicpError> {
        if lower > upper {
            return Err(SicpError::TypeMismatch(format!(
                "interval lower bound {lower} exceeds upper bound {upper}"
            )));
        }
        Ok(Interval { lower, upper })
    }

    /// Builds an interval from its center and half-width: the book's
    /// `make-center-width`.
    ///
    /// # Errors
    /// [`SicpError::TypeMismatch`] when `width` is negative.
    pub fn from_center_width(center: f64, width: f64) -> Result<Self, SicpError> {
        Self::new(center - width, center + width)
    }

    /// The lower bound: the book's `lower-bound`, exercise 2.7.
    #[must_use]
    pub fn lower_bound(&self) -> f64 {
        self.lower
    }

    /// The upper bound: the book's `upper-bound`, exercise 2.7.
    #[must_use]
    pub fn upper_bound(&self) -> f64 {
        self.upper
    }

    /// The center of the interval: the book's `center`.
    #[must_use]
    pub fn center(&self) -> f64 {
        f64::midpoint(self.lower, self.upper)
    }

    /// Half the difference between the bounds: the book's `width`. Always
    /// nonnegative, since the constructor enforces `lower <= upper`.
    #[must_use]
    pub fn width(&self) -> f64 {
        (self.upper - self.lower) / 2.0
    }
}

/// Adds two intervals: the book's `add-interval`. The minimum possible sum
/// is the sum of the lower bounds and the maximum possible sum is the sum
/// of the upper bounds, so the result is well formed without rechecking
/// the invariant.
#[must_use]
pub fn add_interval(x: &Interval, y: &Interval) -> Interval {
    Interval {
        lower: x.lower + y.lower,
        upper: x.upper + y.upper,
    }
}

/// Multiplies two intervals: the book's `mul-interval`, taking the minimum
/// and maximum of all four products of the endpoints. The result is well
/// formed without rechecking the invariant, since it is a minimum and a
/// maximum of the same four numbers.
#[must_use]
pub fn mul_interval(x: &Interval, y: &Interval) -> Interval {
    let p1 = x.lower * y.lower;
    let p2 = x.lower * y.upper;
    let p3 = x.upper * y.lower;
    let p4 = x.upper * y.upper;
    Interval {
        lower: p1.min(p2).min(p3).min(p4),
        upper: p1.max(p2).max(p3).max(p4),
    }
}

/// Divides two intervals: the book's `div-interval`, before exercise
/// 2.10's check. When `y` spans zero this multiplies by a reciprocal
/// interval whose lower bound exceeds its upper bound; the struct literal
/// bypasses the invariant-checking constructor on purpose here, in the one
/// module that defines it, to reproduce the book's own undefined result:
/// ordinary `f64` division never panics, so the outcome is either a
/// finite interval too narrow to be mathematically meaningful or, when an
/// endpoint of `y` is exactly zero, an infinite one. Exercise 2.10 is the
/// fix.
#[must_use]
pub fn div_interval(x: &Interval, y: &Interval) -> Interval {
    let reciprocal_y = Interval {
        lower: 1.0 / y.upper,
        upper: 1.0 / y.lower,
    };
    mul_interval(x, &reciprocal_y)
}
