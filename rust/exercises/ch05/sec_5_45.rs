// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.45: stack ratios, compiled versus interpreted.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.45`.
    pub exercise: &'static str,
}

mod ex_5_45 {
    //! Exercise 5.45: stack ratios, compiled versus interpreted.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_45() -> Result<String, Pending> {
        Err(Pending { exercise: "5.45" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_45() {
    let answer = ex_5_45::ex_5_45().expect("solved");
    assert!(!answer.is_empty());
}
