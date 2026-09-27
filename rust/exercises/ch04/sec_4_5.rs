// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.5: Exercise 4.5: `cond` arrow clauses..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_05 {
    //! Exercise 4.5: `cond` arrow clauses.

    use super::Pending;

    /// Answers the value of the book's arrow-clause cond and of a plain
    /// clause cond.
    pub fn ex_4_05() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.5" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_05() {
    let values = ex_4_05::ex_4_05().expect("solved");
    assert_eq!(values, vec!["2", "b"]);
}
