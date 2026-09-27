// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.74: Eva Lu Ator's zero-crossing
//! rewrite with the general two-stream map of exercise 3.50, the
//! detector mapped over the sense data and the same data shifted one
//! cell, seeded with previous value 0.

mod ex_3_74 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.74`.
        pub exercise: &'static str,
    }

    /// Exercise 3.74: zero crossings via stream-map
    ///
    /// Answers the first fourteen crossings of the book's sense data,
    /// the delayed stream seeded with previous value 0.
    pub fn ex_3_74() -> Result<[f64; 14], Pending> {
        Err(Pending { exercise: "3.74" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_74() {
    let crossings = ex_3_74::ex_3_74().expect("solved");
    // The book's own rows: -1 at index 5 (the dip to -0.1) and +1 at
    // index 10 (the recovery at 0.2), 0 everywhere else.
    for (got, want) in crossings.iter().zip([
        0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
    ]) {
        assert!((got - want).abs() < 1e-12, "crossing: {got} vs {want}");
    }
}
