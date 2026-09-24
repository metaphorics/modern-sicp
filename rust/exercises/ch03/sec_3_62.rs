// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.62: series division and the
//! tangent series it generates.

mod ex_3_62 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.62`.
        pub exercise: &'static str,
    }

    /// Exercise 3.62: div-series and tangent series
    ///
    /// Answers the first 8 coefficients of the tangent series computed
    /// as sine divided by cosine, the statement's demonstration.
    pub fn ex_3_62() -> Result<Vec<f64>, Pending> {
        Err(Pending { exercise: "3.62" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_62() {
    let tangent = ex_3_62::ex_3_62().expect("solved");
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
