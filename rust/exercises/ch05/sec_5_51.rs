// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.51: the explicit-control evaluator translated into C.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.51`.
    pub exercise: &'static str,
}

mod ex_5_51 {
    //! Exercise 5.51: the explicit-control evaluator translated into C.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_51() -> Result<String, Pending> {
        Err(Pending { exercise: "5.51" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_51() {
    let answer = ex_5_51::ex_5_51().expect("solved");
    assert!(!answer.is_empty());
}
