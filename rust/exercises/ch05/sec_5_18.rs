// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of per-register tracing.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.18`.
    pub exercise: &'static str,
}

mod ex_5_18 {
    //! Exercise 5.18: let registers be traced so every assignment
    //! reports old and new contents.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_18() -> Result<String, Pending> {
        Err(Pending { exercise: "5.18" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_18() {
    let answer = ex_5_18::ex_5_18().expect("solved");
    assert!(!answer.is_empty());
}
