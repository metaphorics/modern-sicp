// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.62: dividing power series.
//! Scaling the denominator by the reciprocal of its constant term makes
//! it a unit series, to which exercise 3.61's reciprocal applies, and
//! multiplying by the numerator finishes the quotient:
//! `div-series S1 S2 = mul-series S1 (invert-unit-series (scale-stream
//! S2 (/ 1 (stream-car S2))))`. A zero constant term in the denominator
//! has no reciprocal, so the division signals an error there, as the
//! statement requires. Applied to the 3.59 sine and cosine series,
//! whose constant terms are 0 and 1, the quotient is the tangent
//! series: x + x^3/3 + 2x^5/15 + 17x^7/315.

use ch03::sec_3_5::integers_starting_from;
use ch03::sec_3_5::{Stream, add_streams, cons_stream, scale_stream, self_stream, stream_map2};

/// Exercise-local copy of exercise 3.59's `integrate-series`: each
/// coefficient divided by its 1-based index; the index-to-`f64` cast is
/// a real-number conversion the series algebra wants, exact for the
/// tiny indices the pinned prefixes test.
#[expect(
    clippy::cast_precision_loss,
    reason = "the index becomes a real number for the series algebra; \
              indices stay tiny, so the pinned prefixes stay exact"
)]
fn integrate_series(s: &Stream<f64>) -> Stream<f64> {
    stream_map2(|a, n| a / (*n as f64), s, &integers_starting_from(1))
}

/// Exercise-local copy of exercise 3.59's sine/cosine construction (the
/// module ships neither): two mutually recursive self-referential
/// defines sharing two name slots so the tests use one memoized pair of
/// spines. See `ex_3_59` for the slot discipline.
fn sine_cosine_pair() -> (Stream<f64>, Stream<f64>) {
    use std::cell::RefCell;
    use std::rc::Rc;

    let sine_cell: Rc<RefCell<Option<Stream<f64>>>> = Rc::new(RefCell::new(None));
    let cosine_cell: Rc<RefCell<Option<Stream<f64>>>> = Rc::new(RefCell::new(None));

    let cosine = {
        let sine_cell = Rc::clone(&sine_cell);
        cons_stream(1.0, move || {
            let sine = sine_cell
                .borrow()
                .as_ref()
                .expect("cosine integral read sine before sine was defined")
                .clone();
            integrate_series(&scale_stream(&sine, -1.0))
        })
    };
    let sine = {
        let cosine_cell = Rc::clone(&cosine_cell);
        cons_stream(0.0, move || {
            let cosine = cosine_cell
                .borrow()
                .as_ref()
                .expect("sine integral read cosine before cosine was defined")
                .clone();
            integrate_series(&cosine)
        })
    };
    *sine_cell.borrow_mut() = Some(sine.clone());
    *cosine_cell.borrow_mut() = Some(cosine.clone());
    (sine, cosine)
}

/// Exercise-local copy of exercise 3.60's `mul-series`: the
/// coefficient-wise convolution, defined by its own first cell.
///
/// # Panics
/// Panics on an empty series, which has no constant term to multiply;
/// every series this exercise builds is infinite.
fn mul_series(s1: &Stream<f64>, s2: &Stream<f64>) -> Stream<f64> {
    let a = *s1.head();
    let b = *s2.head();
    let front = s1.clone();
    let back = s2.clone();
    cons_stream(a * b, move || {
        let scaled_tail = scale_stream(&back.tail(), a);
        add_streams(&scaled_tail, &mul_series(&front.tail(), &back))
    })
}

/// Exercise-local copy of exercise 3.61's `invert-unit-series`: the
/// reciprocal of a constant-term-1 series, `X = 1 - S_R X` consed onto
/// the scaled product of the denominator's cdr with `X` itself.
///
/// # Panics
/// Panics if `s` is empty: a series without a constant term has no
/// reciprocal with constant term 1.
fn invert_unit_series(s: &Stream<f64>) -> Stream<f64> {
    self_stream(|x| {
        let source = s.clone();
        cons_stream(1.0, move || {
            let product = mul_series(&source.tail(), &x.stream());
            scale_stream(&product, -1.0)
        })
    })
}

/// The statement's `div-series`: the quotient series `S1 / S2`,
/// requiring a nonzero constant term in the denominator.
///
/// # Panics
/// Panics when the denominator's constant term is exactly zero: the
/// statement requires the error, and no series reciprocal exists to
/// cancel it.
fn div_series(s1: &Stream<f64>, s2: &Stream<f64>) -> Stream<f64> {
    let c0 = *s2.head();
    assert!(c0 != 0.0, "div-series: zero constant term in denominator");
    mul_series(s1, &invert_unit_series(&scale_stream(s2, 1.0 / c0)))
}

mod ex_3_62 {
    use super::{div_series, sine_cosine_pair};

    /// Exercise 3.62: div-series and tangent series
    ///
    /// Answers the first 8 coefficients of the tangent series, computed
    /// as the quotient of the sine and cosine series.
    #[must_use]
    pub fn ex_3_62() -> Vec<f64> {
        let (sine, cosine) = sine_cosine_pair();
        let tangent = div_series(&sine, &cosine);
        tangent.iter().take(8).collect()
    }
}

#[test]
fn ex_3_62() {
    let tangent = ex_3_62::ex_3_62();
    // tan x = x + x^3/3 + 2x^5/15 + 17x^7/315: the odd coefficients of
    // sine divided by cosine, whose constant terms 0 and 1 make the
    // denominator a unit series already.
    let close = |got: &[f64], want: &[f64]| {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            assert!((g - w).abs() < 1e-12, "{g} vs {w}");
        }
    };
    close(
        &tangent,
        &[0.0, 1.0, 0.0, 1.0 / 3.0, 0.0, 2.0 / 15.0, 0.0, 17.0 / 315.0],
    );
}
