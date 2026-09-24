// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.22: Exercise 4.22: `let` in the analyzed evaluator..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_22 {
    //! Exercise 4.22: `let` in the analyzed evaluator.

    use super::Pending;

    /// Answers the analyzed values of a plain let and of a lambda body
    /// whose internal let runs twice under one analysis.
    pub fn ex_4_22() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.22" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_22() {
    let values = ex_4_22::ex_4_22().expect("solved");
    assert_eq!(values, vec!["7", "5", "5"]);
}
