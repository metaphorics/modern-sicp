// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.59: power series as infinite
//! streams of coefficients. `integrate-series` divides each coefficient
//! by its 1-based index, the termwise integral without its constant
//! term. `e^x` is its own integral, so the series is 1 consed onto the
//! integral of itself. Sine and cosine are mutually defined -- the
//! derivative of sine is cosine, of cosine is minus sine -- so the two
//! self-referential defines share two name slots: each cons is built
//! eagerly (the constant terms 0 and 1 need nothing), and each integral
//! tail thunk reads the other series' finished definition.

use std::cell::RefCell;
use std::rc::Rc;

use ch03::sec_3_5::{
    Stream, cons_stream, integers_starting_from, scale_stream, self_stream, stream_map2,
};

/// Exercise-local `integrate-series`: each coefficient divided by its
/// 1-based index, the book's `(stream-map / s (integers-starting-from
/// 1))`. The section module does not ship it; it is this exercise's
/// starting machinery. The index-to-`f64` cast is a real-number
/// conversion the series algebra wants: indices stay tiny, so the
/// division is exact where the pinned prefixes test it.
#[expect(
    clippy::cast_precision_loss,
    reason = "the index becomes a real number for the series algebra; \
              indices stay tiny, so the pinned prefixes stay exact"
)]
fn integrate_series(s: &Stream<f64>) -> Stream<f64> {
    stream_map2(|a, n| a / (*n as f64), s, &integers_starting_from(1))
}

/// The book's `exp-series`: 1 consed onto the integral of itself, since
/// `d(e^x) = e^x` pins the constant term at `e^0 = 1`.
fn exp_series() -> Stream<f64> {
    self_stream(|exp| cons_stream(1.0, move || integrate_series(&exp.stream())))
}

/// Builds the book's `sine-series` and `cosine-series` against each
/// other: two mutually recursive self-referential defines, so the two
/// names are two shared slots filled after both first cells exist.
/// Each tail thunk reads the other slot only once forced, which is
/// after both definitions are complete; the slots form the designed
/// `Rc` cycle the stream spine already leaks by construction.
fn sine_cosine_pair() -> (Stream<f64>, Stream<f64>) {
    let sine_cell: Rc<RefCell<Option<Stream<f64>>>> = Rc::new(RefCell::new(None));
    let cosine_cell: Rc<RefCell<Option<Stream<f64>>>> = Rc::new(RefCell::new(None));

    let cosine = {
        let sine_cell = Rc::clone(&sine_cell);
        // cosine-series: 1 consed onto the integral of minus-sine.
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
        // sine-series: 0 consed onto the integral of cosine.
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

mod ex_3_59 {
    use super::{exp_series, sine_cosine_pair};

    /// The three pinned coefficient prefixes: the `e^x` series, the
    /// sine series, and the cosine series.
    type SeriesPrefixes = (Vec<f64>, Vec<f64>, Vec<f64>);

    /// Exercise 3.59: integrate-series, exp sin cos
    ///
    /// Answers the first 6 coefficients of the `e^x` series, the first
    /// 6 of the sine series, and the first 7 of the cosine series.
    #[must_use]
    pub fn ex_3_59() -> SeriesPrefixes {
        let (sine, cosine) = sine_cosine_pair();
        let exp = exp_series().iter().take(6).collect();
        let sine = sine.iter().take(6).collect();
        let cosine = cosine.iter().take(7).collect();
        (exp, sine, cosine)
    }
}

#[test]
fn ex_3_59() {
    let (exp, sine, cosine) = ex_3_59::ex_3_59();
    let close = |got: &[f64], want: &[f64]| {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            assert!((g - w).abs() < 1e-12, "{g} vs {w}");
        }
    };
    // e^x = 1 + x + x^2/2 + x^3/6 + x^4/24 + x^5/120: integrating the
    // series gives the series back, so the self-reference closes.
    close(&exp, &[1.0, 1.0, 0.5, 1.0 / 6.0, 1.0 / 24.0, 1.0 / 120.0]);
    // sin x = x - x^3/6 + x^5/120: constant 0, each coefficient the
    // running integral of the cosine coefficients.
    close(&sine, &[0.0, 1.0, 0.0, -(1.0 / 6.0), 0.0, 1.0 / 120.0]);
    // cos x = 1 - x^2/2 + x^4/24 - x^6/720: constant 1, the integral of
    // minus sine, the other half of the mutual definition.
    close(
        &cosine,
        &[1.0, 0.0, -0.5, 0.0, 1.0 / 24.0, 0.0, -(1.0 / 720.0)],
    );
}
