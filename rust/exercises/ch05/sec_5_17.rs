// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of labels announced in the trace.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.17`.
    pub exercise: &'static str,
}

mod ex_5_17 {
    //! Exercise 5.17: print the labels that precede a traced
    //! instruction, without disturbing the count.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_17() -> Result<String, Pending> {
        Err(Pending { exercise: "5.17" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_17() {
    let answer = ex_5_17::ex_5_17().expect("solved");
    assert!(!answer.is_empty());
}
