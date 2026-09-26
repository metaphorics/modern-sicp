// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of the assembler's instruction-use summary.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.12`.
    pub exercise: &'static str,
}

mod ex_5_12 {
    //! Exercise 5.12: have the assembler collect the instruction
    //! types, entry-point registers, stacked registers, and assign
    //! sources of a controller.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_12() -> Result<String, Pending> {
        Err(Pending { exercise: "5.12" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_12() {
    let answer = ex_5_12::ex_5_12().expect("solved");
    assert!(!answer.is_empty());
}
