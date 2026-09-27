// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.16: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_16 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.16`.
        pub exercise: &'static str,
    }

    /// Exercise 1.16: iterative exponentiation by successive squaring
    ///
    /// Returns `2^10` and `3^7` as the loop-spelled invariant process
    /// produces them.
    pub fn ex_1_16() -> Result<[i64; 2], Pending> {
        Err(Pending { exercise: "1.16" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_16() {
    let values = ex_1_16::ex_1_16().unwrap_or([0; 2]);
    assert_eq!(values, [1024, 2187]);
}
