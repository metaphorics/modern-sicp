// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.14: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_14 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.14`.
        pub exercise: &'static str,
    }

    /// Exercise 1.14: the count-change tree and its growth
    ///
    /// Returns the number of ways to change 11 cents first, and the
    /// number of calls the tree-recursive process makes while computing
    /// it second.
    pub fn ex_1_14() -> Result<(i64, u64), Pending> {
        Err(Pending { exercise: "1.14" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_14() {
    let (ways, calls) = ex_1_14::ex_1_14().unwrap_or((0, 0));
    assert_eq!(ways, 4);
    assert_eq!(calls, 55);
}
