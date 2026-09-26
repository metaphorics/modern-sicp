// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of three save and restore disciplines, and a pruned machine.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.11`.
    pub exercise: &'static str,
}

mod ex_5_11 {
    //! Exercise 5.11: give restore three possible meanings and
    //! exploit the book's untagged discipline to remove one
    //! instruction from the Fibonacci machine.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_11() -> Result<String, Pending> {
        Err(Pending { exercise: "5.11" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_11() {
    let answer = ex_5_11::ex_5_11().expect("solved");
    assert!(!answer.is_empty());
}
