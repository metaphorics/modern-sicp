// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.11: Exercise 4.11: the frame as an association list..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_11 {
    //! Exercise 4.11: the frame as an association list.

    use super::Pending;

    /// Answers the lookups the association-list environment operations
    /// produce for a shadowed, a rebound, and an unbound name.
    pub fn ex_4_11() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.11" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_11() {
    let values = ex_4_11::ex_4_11().expect("solved");
    assert_eq!(values, vec!["2", "10", "unbound variable: z"]);
}
