// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of pushes and maximum depth of the factorial machine.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.14`.
    pub exercise: &'static str,
}

mod ex_5_14 {
    //! Exercise 5.14: measure pushes and maximum stack depth of the
    //! factorial machine and read the formulas off the data.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_14() -> Result<String, Pending> {
        Err(Pending { exercise: "5.14" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_14() {
    let answer = ex_5_14::ex_5_14().expect("solved");
    assert!(!answer.is_empty());
}
