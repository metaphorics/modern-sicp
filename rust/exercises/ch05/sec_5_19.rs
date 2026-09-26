// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of breakpoints with proceed and cancel.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.19`.
    pub exercise: &'static str,
}

mod ex_5_19 {
    //! Exercise 5.19: stop the machine before the nth instruction
    //! after a label, examine it, and proceed or cancel.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_19() -> Result<String, Pending> {
        Err(Pending { exercise: "5.19" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_19() {
    let answer = ex_5_19::ex_5_19().expect("solved");
    assert!(!answer.is_empty());
}
