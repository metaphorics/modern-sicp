// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of section 1.1, one module and one ignored test per
//! exercise.

/// The pending scaffold of exercise 1.1: the stub and the exercise-named
/// test share one module so both carry the exercise's name.
mod ex_1_01 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.1`.
        pub exercise: &'static str,
    }

    /// Exercise 1.1: evaluate a sequence of expressions in order
    ///
    /// Yields the ten numeric values the sequence prints, in book order; the
    /// two `define` lines and the `#f` comparison carry no number.
    pub fn ex_1_01() -> Result<Vec<i64>, Pending> {
        Err(Pending { exercise: "1.1" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_01() {
    assert_eq!(
        ex_1_01::ex_1_01(),
        Ok(vec![10, 12, 8, 3, 6, 19, 4, 16, 6, 16])
    );
}
