// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.25: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_25 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.25`.
        pub exercise: &'static str,
    }

    /// Exercise 1.25: Alyssa's `expmod` and where its width runs out
    ///
    /// Returns `Some` of Alyssa's checked result for a small case where
    /// `fast_expt` still fits `i128`, and `None` for a prime-sized case
    /// where it overflows.
    pub fn ex_1_25() -> Result<(Option<i128>, Option<i128>), Pending> {
        Err(Pending { exercise: "1.25" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_25() {
    let (small, large) = ex_1_25::ex_1_25().unwrap_or((None, None));
    assert_eq!(small, Some(5));
    assert_eq!(large, None);
}
