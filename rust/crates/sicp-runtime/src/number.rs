// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The numeric tower of 2.5: exact integers over exact rationals over
//! inexact reals over complex. Moving between rungs is the `raise` and
//! `project` of the generic-arithmetic table; the edition plan routes it
//! through conversions on this enum.

use std::fmt::{self, Display, Formatter};

use crate::error::SicpError;

/// A number of the tower. Exact integers are checked `i128`; exact
/// rationals carry the `make-rat` invariants — normalized by the gcd,
/// denominator positive — enforced by [`Number::rat`].
#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    /// An exact integer.
    Int(i128),
    /// An exact rational, always normalized: `gcd(|num|, den) == 1`,
    /// `den > 0`. `Rat` never collapses to `Int` here; rung collapse is
    /// the projection step, not a constructor side effect.
    Rat {
        /// The signed numerator.
        num: i128,
        /// The positive denominator.
        den: i128,
    },
    /// An inexact real.
    Real(f64),
    /// An inexact complex number.
    Complex {
        /// The real part.
        re: f64,
        /// The imaginary part.
        im: f64,
    },
}

/// Greatest common divisor of two nonnegative magnitudes, Euclid's
/// algorithm; `gcd(x, 0) == x`.
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl Number {
    /// Builds an exact integer.
    #[must_use]
    pub fn int(n: i128) -> Self {
        Number::Int(n)
    }

    /// Builds an inexact real.
    #[must_use]
    pub fn real(x: f64) -> Self {
        Number::Real(x)
    }

    /// Builds an inexact complex number.
    #[must_use]
    pub fn complex(re: f64, im: f64) -> Self {
        Number::Complex { re, im }
    }

    /// Builds the book's `make-rat`: normalizes by the gcd and keeps the
    /// denominator positive, so `rat(6, 4)` and `rat(-3, -2)` are both
    /// `3/2`.
    ///
    /// # Errors
    /// [`SicpError::DivisionByZero`] on a zero denominator;
    /// [`SicpError::Overflow`] when normalizing `i128::MIN / -1`, whose
    /// positive quotient does not fit the exact width.
    pub fn rat(num: i128, den: i128) -> Result<Self, SicpError> {
        if den == 0 {
            return Err(SicpError::DivisionByZero);
        }
        let negative = (num < 0) != (den < 0);
        let g = gcd(num.unsigned_abs(), den.unsigned_abs());
        let num = match i128::try_from(num.unsigned_abs() / g) {
            Ok(n) => {
                if negative {
                    -n
                } else {
                    n
                }
            }
            // The magnitude is 2^127 only for num == i128::MIN with gcd 1;
            // -2^127 fits as i128::MIN, +2^127 does not exist.
            Err(_) if negative => i128::MIN,
            Err(_) => return Err(SicpError::Overflow),
        };
        let den = i128::try_from(den.unsigned_abs() / g).map_err(|_| SicpError::Overflow)?;
        Ok(Number::Rat { num, den })
    }

    /// Raises one rung of the tower: `Int` to `Rat`, `Rat` to `Real`,
    /// `Real` to `Complex`.
    #[must_use]
    pub fn raise(&self) -> Option<Self> {
        match self {
            Number::Int(n) => Some(Number::Rat { num: *n, den: 1 }),
            Number::Rat { num, den } => {
                // The rung's meaning is "become inexact": the i128->f64
                // narrowing past 2^53 is the conversion the book's
                // exact->inexact performs, not silent truncation.
                #[allow(
                    clippy::cast_precision_loss,
                    reason = "exact->inexact is the rung's stated meaning"
                )]
                Some(Number::real((*num as f64) / (*den as f64)))
            }
            Number::Real(x) => Some(Number::complex(*x, 0.0)),
            Number::Complex { .. } => None,
        }
    }

    /// Projects one rung down, the mechanical half of the book's
    /// `project`: `Complex` to its real part, `Rat` to `Int` only when the
    /// denominator is 1. The `Real` rung needs `rationalize`, which
    /// belongs with 2.5's generic operations, so this spine returns the
    /// absent option there rather than guessing its rounding.
    #[must_use]
    pub fn project(&self) -> Option<Self> {
        match self {
            Number::Complex { re, .. } => Some(Number::real(*re)),
            Number::Rat { num, den: 1 } => Some(Number::int(*num)),
            _ => None,
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(n) => write!(f, "{n}"),
            Number::Rat { num, den } => write!(f, "{num}/{den}"),
            Number::Real(x) => write!(f, "{x}"),
            Number::Complex { re, im } => {
                if *im < 0.0 {
                    write!(f, "{re}{im}i")
                } else {
                    write!(f, "{re}+{im}i")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Number;
    use crate::error::SicpError;

    #[test]
    fn rat_normalizes_by_gcd_and_sign() {
        assert_eq!(Number::rat(6, 4), Ok(Number::Rat { num: 3, den: 2 }));
        assert_eq!(Number::rat(-3, -2), Ok(Number::Rat { num: 3, den: 2 }));
        assert_eq!(Number::rat(3, -2), Ok(Number::Rat { num: -3, den: 2 }));
        assert_eq!(Number::rat(0, 5), Ok(Number::Rat { num: 0, den: 1 }));
        assert_eq!(Number::rat(7, 1), Ok(Number::Rat { num: 7, den: 1 }));
    }

    #[test]
    fn zero_denominator_is_division_by_zero() {
        assert_eq!(Number::rat(1, 0), Err(SicpError::DivisionByZero));
    }

    #[test]
    fn min_int_over_negative_one_overflows() {
        assert_eq!(Number::rat(i128::MIN, -1), Err(SicpError::Overflow));
        // The extreme numerator over denominator 1 is itself representable.
        assert_eq!(
            Number::rat(i128::MIN, 1),
            Ok(Number::Rat {
                num: i128::MIN,
                den: 1
            })
        );
        // The same quotient reached from a normalizable pair fits.
        assert_eq!(
            Number::rat(i128::MIN, -2),
            Ok(Number::Rat {
                num: i128::MAX / 2 + 1,
                den: 1
            })
        );
    }

    #[test]
    fn raise_walks_int_rat_real_complex() {
        assert_eq!(Number::int(2).raise(), Some(Number::Rat { num: 2, den: 1 }));
        assert_eq!(
            Number::rat(4, 2).map(|n| n.raise()),
            Ok(Some(Number::real(2.0)))
        );
        assert_eq!(Number::real(2.0).raise(), Some(Number::complex(2.0, 0.0)));
        assert_eq!(Number::complex(2.0, 0.0).raise(), None);
    }

    #[test]
    fn project_walks_down_where_the_rung_is_mechanical() {
        assert_eq!(Number::complex(2.0, 3.0).project(), Some(Number::real(2.0)));
        assert_eq!(
            Number::rat(6, 2).map(|n| n.project()),
            Ok(Some(Number::int(3)))
        );
        assert_eq!(Number::rat(3, 2).map(|n| n.project()), Ok(None));
        // Real needs rationalize: left to 2.5's generic operations.
        assert_eq!(Number::real(0.5).project(), None);
        assert_eq!(Number::int(5).project(), None);
    }

    #[test]
    fn display_is_scheme_syntax() {
        assert_eq!(Number::int(-7).to_string(), "-7");
        assert_eq!(Number::rat(3, 2).expect("valid").to_string(), "3/2");
        assert_eq!(Number::real(2.5).to_string(), "2.5");
        assert_eq!(Number::complex(3.0, 2.0).to_string(), "3+2i");
        assert_eq!(Number::complex(3.0, -2.0).to_string(), "3-2i");
    }
}
