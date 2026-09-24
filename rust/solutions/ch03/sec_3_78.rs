// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.78: `solve-2nd`, the feedback
//! network of Figure 3.35 for the homogeneous second-order equation
//! `y'' - a y' - b y = 0`. Two delayed integrals produce `dy` and `y`
//! from the shared `ddy`; `ddy` is the scaled sum `a dy + b y` read
//! back from both integrals, so the loop the diagram draws is real:
//! the value of `d^2 y` depends on the two streams that integrate it.

use ch03::sec_3_5::{Stream, add_streams, integral_delayed, scale_stream, self_stream, stream_ref};

/// The book's `solve-2nd`: the stream of `y` values of the solution of
/// `y'' - a y' - b y = 0` with `y(0)` = `y0` and `y'(0)` = `dy0`,
/// integrated at step `dt`.
///
/// The network is the diagram: `y` is the integral of the delayed `dy`
/// from `y0`, `dy` is the integral of the delayed `ddy` from `dy0`, and
/// `ddy = a dy + b y`. Each integrand is a thunk that [`integral_delayed`]
/// forces only when its integral's first tail element is demanded --
/// strictly after both definitions are complete -- which is how the
/// loop closes without any definition reading itself too early.
#[must_use]
pub fn solve_2nd(a: f64, b: f64, dt: f64, y0: f64, dy0: f64) -> Stream<f64> {
    self_stream(move |y| {
        // `dy`'s integrand names both streams: `dy` through its own
        // definition's name, `y` through the outer one. Both names are
        // read inside the delayed thunk, so both read finished streams.
        let dy = self_stream(move |dy_name| {
            let ddy = {
                let y = y.clone();
                move || {
                    let dy_stream = dy_name.stream();
                    let y_stream = y.stream();
                    add_streams(&scale_stream(&dy_stream, a), &scale_stream(&y_stream, b))
                }
            };
            integral_delayed(ddy, dy0, dt)
        });
        // The book's `(delay dy)`: `dy` handed over as a thunk.
        integral_delayed(move || dy, y0, dt)
    })
}

mod ex_3_78 {
    use super::stream_ref;

    /// Exercise 3.78: solve-2nd feedback loop
    ///
    /// Answers the harmonic-oscillator run of [`super::solve_2nd`]
    /// (`a` = 0, `b` = -1, `dt` = 0.01, `y0` = 1, `dy0` = 0, so
    /// `y'' = -y` and `y` approximates cos `t`): `y` at indices 0, 50,
    /// 157, 314, that is `t` = 0, 0.5, 1.57, 3.14.
    #[must_use]
    pub fn ex_3_78() -> (f64, f64, f64, f64) {
        let y = super::solve_2nd(0.0, -1.0, 0.01, 1.0, 0.0);
        (
            stream_ref(&y, 0),
            stream_ref(&y, 50),
            stream_ref(&y, 157),
            stream_ref(&y, 314),
        )
    }
}

#[test]
fn ex_3_78() {
    let (y0, y50, y157, y314) = ex_3_78::ex_3_78();
    // Exact pins of the forward-Euler run: y[1] = y0 + dt*ddy[0] with
    // ddy[0] = a*dy0 + b*y0, and so on down the memoized spine.
    assert!((y0 - 1.0).abs() < 1e-12);
    assert!((y50 - 0.879_787_162_885_263).abs() < 1e-12);
    assert!((y157 - 0.000_855_344_724_179).abs() < 1e-12);
    assert!((y314 + 1.015_821_631_924_213).abs() < 1e-12);
    // The same samples against cos t. Explicit Euler both lags the
    // phase and slowly pumps the amplitude, so the honest tolerance
    // grows with t: the measured errors are 0, 2.2e-3, 5.9e-5, 1.6e-2.
    assert!((y0 - 0.0_f64.cos()).abs() < 1e-12);
    assert!((y50 - 0.5_f64.cos()).abs() < 0.01);
    assert!((y157 - 1.57_f64.cos()).abs() < 0.001);
    assert!((y314 - (314.0_f64 * 0.01).cos()).abs() < 0.05);
}
