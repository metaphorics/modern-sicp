// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.76: `smooth` as a reusable
//! component -- each element the average of two successive input
//! elements -- and the section's own zero-crossing detector run
//! unchanged on the smoothed sense data.

mod ex_3_76 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.76`.
        pub exercise: &'static str,
    }

    /// Exercise 3.76: smooth as reusable combinator
    ///
    /// Answers the first thirteen smoothed sense values and the first
    /// fourteen crossings of the smoothed signal, the section's own
    /// detector run unchanged on the smoothed stream with last value 0.
    pub fn ex_3_76() -> Result<([f64; 13], [f64; 14]), Pending> {
        Err(Pending { exercise: "3.76" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_76() {
    let (smoothed, crossings) = ex_3_76::ex_3_76().expect("solved");
    // Each cell is the mean of the raw cell and its successor.
    for (got, want) in smoothed.iter().zip([
        1.5, 1.75, 1.25, 0.75, 0.2, -1.05, -2.5, -2.5, -1.25, -0.15, 1.6, 3.5, 4.0,
    ]) {
        assert!((got - want).abs() < 1e-12, "smoothed: {got} vs {want}");
    }
    // The two real crossings survive, no spurious crossing appears:
    // -1 at index 5 and +1 at index 10, the raw answer's indices.
    for (got, want) in crossings.iter().zip([
        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
    ]) {
        assert!((got - want).abs() < 1e-12, "crossing: {got} vs {want}");
    }
}
