// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.78: the second-order feedback
//! network of Figure 3.35, whose `d^2 y` output is integrated twice
//! into `dy` and `y` and fed back scaled by `a` and `b`.

mod ex_3_78 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.78`.
        pub exercise: &'static str,
    }

    /// Exercise 3.78: solve-2nd feedback loop
    ///
    /// The solved entry point answers the harmonic-oscillator run of
    /// the `solve-2nd` network (`a` = 0, `b` = -1, `dt` = 0.01, `y0` =
    /// 1, `dy0` = 0, so `y'' = -y` and `y` approximates cos `t`):
    /// `y` at indices 0, 50, 157, 314, that is `t` = 0, 0.5, 1.57, 3.14.
    /// The pending body reports [`Pending`].
    pub fn ex_3_78() -> Result<(f64, f64, f64, f64), Pending> {
        Err(Pending { exercise: "3.78" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_78() {
    let (y0, y50, y157, y314) = ex_3_78::ex_3_78().expect("solved");
    // Forward-Euler values of the y'' = -y run; exact pins measured on
    // the solved network, then the same samples against cos t with the
    // tolerances the growing Euler error allows.
    assert!((y0 - 1.0).abs() < 1e-12);
    assert!((y50 - 0.879_787_162_885_263).abs() < 1e-12);
    assert!((y157 - 0.000_855_344_724_179).abs() < 1e-12);
    assert!((y314 + 1.015_821_631_924_213).abs() < 1e-12);
    assert!((y0 - 0.0_f64.cos()).abs() < 1e-12);
    assert!((y50 - 0.5_f64.cos()).abs() < 0.01);
    assert!((y157 - 1.57_f64.cos()).abs() < 0.001);
    assert!((y314 - (314.0_f64 * 0.01).cos()).abs() < 0.05);
}
