// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.1: design the iterative
//! factorial machine.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.1`.
    pub exercise: &'static str,
}

mod ex_5_01 {
    //! Exercise 5.1: design a register machine for the iterative
    //! factorial, drawing the data-path and controller diagrams.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_01() -> Result<String, Pending> {
        Err(Pending { exercise: "5.1" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_01() {
    let answer = ex_5_01::ex_5_01().expect("solved");
    assert!(!answer.is_empty());
}
