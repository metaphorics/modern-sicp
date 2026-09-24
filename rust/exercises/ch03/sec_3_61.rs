// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.61: the reciprocal of a power
//! series with constant term 1, `invert-unit-series`.

mod ex_3_61 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.61`.
        pub exercise: &'static str,
    }

    /// Exercise 3.61: invert-unit-series
    ///
    /// Answers the first 5 coefficients of the cosine series times its
    /// series reciprocal, and the first 5 coefficients of the reciprocal
    /// of the series 1 + 2x + 0x^2 + ... whose pattern is geometric.
    pub fn ex_3_61() -> Result<(Vec<f64>, Vec<f64>), Pending> {
        Err(Pending { exercise: "3.61" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_61() {
    let (identity, geometric) = ex_3_61::ex_3_61().expect("solved");
    let close = |got: &[f64], want: &[f64]| {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            assert!((g - w).abs() < 1e-12, "{g} vs {w}");
        }
    };
    close(&identity, &[1.0, 0.0, 0.0, 0.0, 0.0]);
    close(&geometric, &[1.0, -2.0, 4.0, -8.0, 16.0]);
}
