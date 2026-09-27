// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.60: series multiplication by
//! convolution, `mul-series`.

mod ex_3_60 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.60`.
        pub exercise: &'static str,
    }

    /// Exercise 3.60: mul-series convolution
    ///
    /// Answers the first 6 coefficients of `sin^2 + cos^2` computed by
    /// the series multiplier over the 3.59 sine and cosine series.
    pub fn ex_3_60() -> Result<Vec<f64>, Pending> {
        Err(Pending { exercise: "3.60" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_60() {
    let identity = ex_3_60::ex_3_60().expect("solved");
    let close = |got: &[f64], want: &[f64]| {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            assert!((g - w).abs() < 1e-12, "{g} vs {w}");
        }
    };
    close(&identity, &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
}
