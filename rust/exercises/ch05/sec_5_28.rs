// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.28: the tail recursion removed: the naive ev-sequence reruns the 5.26 and 5.27 experiments.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.28`.
    pub exercise: &'static str,
}

mod ex_5_28 {
    //! Exercise 5.28: the tail recursion removed: the naive ev-sequence reruns the 5.26 and 5.27 experiments.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_28() -> Result<String, Pending> {
        Err(Pending { exercise: "5.28" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_28() {
    let answer = ex_5_28::ex_5_28().expect("solved");
    assert!(!answer.is_empty());
}
