// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.18: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_18 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.18`.
        pub exercise: &'static str,
    }

    /// Exercise 1.18: iterative Russian peasant multiplication
    ///
    /// Returns `7 * 5`, `17 * 33`, and `0 * 9` as the loop-spelled
    /// invariant process produces them.
    pub fn ex_1_18() -> Result<[i64; 3], Pending> {
        Err(Pending { exercise: "1.18" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_18() {
    let values = ex_1_18::ex_1_18().unwrap_or([0; 3]);
    assert_eq!(values, [35, 561, 0]);
}
