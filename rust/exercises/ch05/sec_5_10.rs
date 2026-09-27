// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of a new surface syntax behind the datum seam.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.10`.
    pub exercise: &'static str,
}

mod ex_5_10 {
    //! Exercise 5.10: design a new syntax for register-machine
    //! instructions and install it without touching the assembler or
    //! the machine.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_10() -> Result<String, Pending> {
        Err(Pending { exercise: "5.10" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_10() {
    let answer = ex_5_10::ex_5_10().expect("solved");
    assert!(!answer.is_empty());
}
