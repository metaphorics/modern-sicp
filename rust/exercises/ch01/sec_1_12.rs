// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.12: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_12 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.12`.
        pub exercise: &'static str,
    }

    /// Exercise 1.12: elements of Pascal's triangle
    ///
    /// Returns the elements at `(row 4, column 2)`, `(row 6, column 3)`,
    /// and `(row 0, column 0)` in that order, counting rows and columns
    /// from zero.
    pub fn ex_1_12() -> Result<[u64; 3], Pending> {
        Err(Pending { exercise: "1.12" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_12() {
    let values = ex_1_12::ex_1_12().unwrap_or([0; 3]);
    assert_eq!(values, [6, 20, 1]);
}
