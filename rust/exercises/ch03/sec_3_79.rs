// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.79: the general second-order
//! solver, `d^2 y/dt^2 = f(dy/dt, y)`, the same two-integral loop of
//! exercise 3.78 with the `a dy + b y` adder replaced by `f`.

mod ex_3_79 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.79`.
        pub exercise: &'static str,
    }

    /// Exercise 3.79: general second-order solver
    ///
    /// The solved entry point answers two runs of the general network at
    /// `dt` = 0.01, `y0` = 1, `dy0` = 0: the undamped oscillator
    /// `f(dy, y) = -y` sampled at `y` indices 0, 50, 157, 314, and the
    /// damped run `f(dy, y) = -0.1 dy - y` reduced to the peak
    /// `|y|` of each of five successive one-period (629-sample)
    /// windows. The pending body reports [`Pending`].
    #[expect(
        clippy::type_complexity,
        reason = "the pending report mirrors the solution's answer type"
    )]
    pub fn ex_3_79() -> Result<((f64, f64, f64, f64), [f64; 5]), Pending> {
        Err(Pending { exercise: "3.79" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_79() {
    let (undamped, envelope) = ex_3_79::ex_3_79().expect("solved");
    let (y0, y50, y157, y314) = undamped;
    // The undamped run is exercise 3.78's oscillator: same pins, same
    // measured cos tolerances.
    assert!((y0 - 1.0).abs() < 1e-12);
    assert!((y50 - 0.879_787_162_885_263).abs() < 1e-12);
    assert!((y157 - 0.000_855_344_724_179).abs() < 1e-12);
    assert!((y314 + 1.015_821_631_924_213).abs() < 1e-12);
    assert!((y0 - 0.0_f64.cos()).abs() < 1e-12);
    assert!((y50 - 0.5_f64.cos()).abs() < 0.01);
    assert!((y157 - 1.57_f64.cos()).abs() < 0.001);
    assert!((y314 - (314.0_f64 * 0.01).cos()).abs() < 0.05);
    // The damped run's window peaks: exact pins of the measured
    // envelope, then the decay itself.
    for (peak, pinned) in envelope.into_iter().zip(PINNED_ENVELOPE) {
        assert!((peak - pinned).abs() < 1e-9);
    }
    assert!(envelope[0] > envelope[1]);
    assert!(envelope[1] > envelope[2]);
    assert!(envelope[2] > envelope[3]);
    assert!(envelope[3] > envelope[4]);
}

/// The five measured window peaks of the damped run.
const PINNED_ENVELOPE: [f64; 5] = [
    1.0,
    0.753_455_690_398,
    0.567_693_464_718,
    0.427_728_838_755,
    0.322_271_300_395,
];
