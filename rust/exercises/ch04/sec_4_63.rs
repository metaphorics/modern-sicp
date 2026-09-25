// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.63: family relations rules.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_63 {
    //! Exercise 4.63: Genesis son, wife, and grandson rules over the book's data base.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_4_63() -> Result<String, Pending> {
        Err(Pending { exercise: "4.63" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_63() {
    let answer = ex_4_63::ex_4_63().expect("solved");
    assert!(!answer.is_empty());
}
