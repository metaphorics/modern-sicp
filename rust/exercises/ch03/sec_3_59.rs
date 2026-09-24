// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.59: `integrate-series` and the
//! power series of `e^x`, sine, and cosine built from it.

mod ex_3_59 {
    /// The three pinned coefficient prefixes: the `e^x` series, the
    /// sine series, and the cosine series.
    type SeriesPrefixes = (Vec<f64>, Vec<f64>, Vec<f64>);

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.59`.
        pub exercise: &'static str,
    }

    /// Exercise 3.59: integrate-series, exp sin cos
    ///
    /// Answers the first 6 coefficients of the `e^x` series, the first
    /// 6 of the sine series, and the first 7 of the cosine series.
    pub fn ex_3_59() -> Result<SeriesPrefixes, Pending> {
        Err(Pending { exercise: "3.59" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_59() {
    let (exp, sine, cosine) = ex_3_59::ex_3_59().expect("solved");
    let close = |got: &[f64], want: &[f64]| {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            assert!((g - w).abs() < 1e-12, "{g} vs {w}");
        }
    };
    close(&exp, &[1.0, 1.0, 0.5, 1.0 / 6.0, 1.0 / 24.0, 1.0 / 120.0]);
    close(&sine, &[0.0, 1.0, 0.0, -(1.0 / 6.0), 0.0, 1.0 / 120.0]);
    close(
        &cosine,
        &[1.0, 0.0, -0.5, 0.0, 1.0 / 24.0, 0.0, -(1.0 / 720.0)],
    );
}
