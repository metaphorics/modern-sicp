// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.60: multiplying power series as
//! streams of coefficients. Writing `S1 = a + S1'` and `S2 = b + S2'`,
//! the product's constant term is `a b`, and the rest is `a x S2'`
//! plus `S1' x S2` -- the head-scaled tail of each factor plus the
//! product that recursion still owes -- so the tail is
//! `add-streams (scale-stream S2' a) (mul-series S1' S2)`. The
//! definition is its own first cell, exactly the cons/add-streams fill
//! the statement asks to complete. The test multiplies the 3.59 sine
//! and cosine series each by itself and adds the squares: the
//! identity `sin^2 x + cos^2 x = 1` comes out as coefficients
//! 1 followed by zeros.

use ch03::sec_3_5::{Stream, add_streams, cons_stream, scale_stream};

/// Exercise-local copy of exercise 3.59's `integrate-series` (the
/// module ships it to no one): each coefficient divided by its 1-based
/// index. The index-to-`f64` cast is a real-number conversion the
/// series algebra wants; indices stay tiny, so it is exact where the
/// pinned prefixes test it.
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
/// defines -- sine is 0 consed onto the integral of cosine, cosine is 1
/// consed onto the integral of minus sine -- sharing two name slots so
/// the tests use one memoized pair of spines. See `ex_3_59` for the
/// slot discipline.
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

/// The statement's `mul-series`: the coefficient-wise convolution of
/// two power series, defined by its own first cell.
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

mod ex_3_60 {
    use super::{Stream, add_streams, mul_series, sine_cosine_pair};

    /// Exercise 3.60: mul-series convolution
    ///
    /// Answers the first 6 coefficients of `sin^2 + cos^2` by the
    /// series multiplier, the statement's own test of `mul-series`.
    #[must_use]
    pub fn ex_3_60() -> Vec<f64> {
        let (sine, cosine) = sine_cosine_pair();
        let sin_squared = mul_series(&sine, &sine);
        let cos_squared = mul_series(&cosine, &cosine);
        let identity: Stream<f64> = add_streams(&sin_squared, &cos_squared);
        identity.iter().take(6).collect()
    }
}

#[test]
fn ex_3_60() {
    let identity = ex_3_60::ex_3_60();
    // sin^2 + cos^2 = 1: the constant terms 0^2 + 1^2 give 1, and every
    // higher convolution sum cancels -- the identity holds coefficient
    // by coefficient, so the stream is 1 followed by zeros.
    let close = |got: &[f64], want: &[f64]| {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            assert!((g - w).abs() < 1e-12, "{g} vs {w}");
        }
    };
    close(&identity, &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
}
