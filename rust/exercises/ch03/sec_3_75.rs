// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.75: Louis Reasoner's buggy
//! smoothing detector translated verbatim, the fixed structure with the
//! hint's extra argument, and Louis's internal averaging ladder that
//! shows the contamination.

mod ex_3_75 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.75`.
        pub exercise: &'static str,
    }

    /// Exercise 3.75: buggy smoothing detector fix
    ///
    /// Louis's crossings, the fixed form's, and his internal averages.
    pub type CrossingReport = ([f64; 14], [f64; 14], [f64; 7]);

    /// Answers Louis's crossings on the sense data, the fixed form's
    /// crossings, and the first seven of Louis's internal averages.
    pub fn ex_3_75() -> Result<CrossingReport, Pending> {
        Err(Pending { exercise: "3.75" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_75() {
    let (louis, fixed, averages) = ex_3_75::ex_3_75().expect("solved");
    // Louis detects on a lagging blend: -1 at index 6, +1 at index 11,
    // one step later than Alyssa's raw answer (5 and 10).
    for (got, want) in louis.iter().zip([
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    ]) {
        assert!(
            (got - want).abs() < 1e-12,
            "louis crossing: {got} vs {want}"
        );
    }
    // The fixed form compares each new average against the previous
    // average: the same crossing positions on this clean data, from the
    // correct adjacent-pair ladder.
    for (got, want) in fixed.iter().zip([
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    ]) {
        assert!(
            (got - want).abs() < 1e-12,
            "fixed crossing: {got} vs {want}"
        );
    }
    // From the second cell Louis's averages stop being adjacent-pair
    // means: 1.25 = (2 + 0.5)/2 where the plan gives (2 + 1)/2 = 1.5.
    for (got, want) in
        averages
            .iter()
            .zip([0.5, 1.25, 1.375, 1.187_5, 0.843_75, 0.371_875, -0.814_062_5])
    {
        assert!((got - want).abs() < 1e-12, "louis average: {got} vs {want}");
    }
}
