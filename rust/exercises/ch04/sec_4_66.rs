// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.66: accumulation over frames.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_66 {
    //! Exercise 4.66: salary accumulation over frames, broken by duplicated answers.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_4_66() -> Result<String, Pending> {
        Err(Pending { exercise: "4.66" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_66() {
    let answer = ex_4_66::ex_4_66().expect("solved");
    assert!(!answer.is_empty());
}
