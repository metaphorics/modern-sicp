// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.78: query language on amb evaluator.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_78 {
    //! Exercise 4.78: the query language as a nondeterministic program over explicit choice frames.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_4_78() -> Result<String, Pending> {
        Err(Pending { exercise: "4.78" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_78() {
    let answer = ex_4_78::ex_4_78().expect("solved");
    assert!(!answer.is_empty());
}
