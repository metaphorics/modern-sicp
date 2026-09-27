// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of duplicate label detection in the assembler.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.8`.
    pub exercise: &'static str,
}

mod ex_5_08 {
    //! Exercise 5.8: make the assembler signal an error when a label
    //! name is used for two different locations.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_08() -> Result<String, Pending> {
        Err(Pending { exercise: "5.8" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_08() {
    let answer = ex_5_08::ex_5_08().expect("solved");
    assert!(!answer.is_empty());
}
