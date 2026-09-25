// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.6: the Fibonacci machine's
//! redundant save and restore.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.6`.
    pub exercise: &'static str,
}

mod ex_5_06 {
    //! Exercise 5.6: find the Fibonacci machine's redundant save and
    //! restore.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_06() -> Result<String, Pending> {
        Err(Pending { exercise: "5.6" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_06() {
    let answer = ex_5_06::ex_5_06().expect("solved");
    assert!(!answer.is_empty());
}
