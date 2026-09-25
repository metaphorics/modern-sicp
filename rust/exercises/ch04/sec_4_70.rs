// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.70: let binding in add-assertion!.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_70 {
    //! Exercise 4.70: the cyclic-assertion hazard of add-assertion! without the let binding.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_4_70() -> Result<String, Pending> {
        Err(Pending { exercise: "4.70" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_70() {
    let answer = ex_4_70::ex_4_70().expect("solved");
    assert!(!answer.is_empty());
}
