// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.61: the reciprocal of a power
//! series whose constant term is 1. Writing `S = 1 + S_R`, the
//! statement's algebra `S X = 1` rearranges to `X = 1 - S_R X`, so the
//! reciprocal's constant term is 1 and every later coefficient is the
//! negative of a convolution sum of `S_R` with `X` itself. Because
//! `S_R`'s constant coefficient is 0, the stream product of the cdr of
//! `S` with `X` is exactly `S_R X` shifted one place left, which makes
//! the definition a tail-recursive cons:
//! `X = cons-stream 1 (scale-stream (mul-series (stream-cdr S) X) -1)`.
//! The test multiplies the 3.59 cosine series by its reciprocal -- the
//! identity 1 followed by zeros -- and inverts the geometric series
//! 1 + 2x, whose coefficients alternate powers of 2.

use ch03::sec_3_5::{Stream, add_streams, cons_stream, scale_stream, self_stream};

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
    use ch03::sec_3_5::{integers_starting_from, stream_map2};
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

/// The statement's `invert-unit-series`: the series `X` with `S X = 1`,
/// built from `X = 1 - S_R X`. The cdr of `S` starts with the 0
/// constant coefficient of `S_R`, so its stream product with `X` carries
/// `S_R X` from index 1 onward, and scaling by -1 gives exactly the
/// tail of `X`.
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

/// The test's `1 + 2x` series: 1, then 2, then zeros forever. The
/// zeros are their own self-referential stream, since a truncated
/// series would hit the empty stream inside `mul-series`.
fn one_plus_two_x() -> Stream<f64> {
    let zeros = self_stream(|z| cons_stream(0.0, move || z.stream()));
    cons_stream(1.0, move || {
        cons_stream(2.0, {
            let zeros = zeros.clone();
            move || zeros
        })
    })
}

mod ex_3_61 {
    use super::{mul_series, one_plus_two_x, sine_cosine_pair};

    /// Exercise 3.61: invert-unit-series
    ///
    /// Answers the first 5 coefficients of the cosine series times its
    /// series reciprocal, and the first 5 of the reciprocal of
    /// `1 + 2x`.
    #[must_use]
    pub fn ex_3_61() -> (Vec<f64>, Vec<f64>) {
        let (_, cosine) = sine_cosine_pair();
        let reciprocal = super::invert_unit_series(&cosine);
        let identity = mul_series(&cosine, &reciprocal);
        let geometric = super::invert_unit_series(&one_plus_two_x());
        (
            identity.iter().take(5).collect(),
            geometric.iter().take(5).collect(),
        )
    }
}

#[test]
fn ex_3_61() {
    let (identity, geometric) = ex_3_61::ex_3_61();
    // cos x times its reciprocal is 1: the definition solves S X = 1,
    // so the convolution is the unit series -- 1 then zeros.
    let close = |got: &[f64], want: &[f64]| {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            assert!((g - w).abs() < 1e-12, "{g} vs {w}");
        }
    };
    close(&identity, &[1.0, 0.0, 0.0, 0.0, 0.0]);
    // 1 / (1 + 2x) = 1 - 2x + 4x^2 - 8x^3 + 16x^4 - ...: geometric with
    // ratio -2, the pattern check of the reciprocal's coefficients.
    close(&geometric, &[1.0, -2.0, 4.0, -8.0, 16.0]);
}
