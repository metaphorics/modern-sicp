// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.12: Exercise 4.12: abstract environment traversals..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_12 {
    //! Exercise 4.12: abstract environment traversals.

    use super::Pending;

    /// Answers the lookups the traversals-redefined operations produce.
    pub fn ex_4_12() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.12" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_12() {
    let values = ex_4_12::ex_4_12().expect("solved");
    assert_eq!(values, vec!["2", "10", "unbound variable: z"]);
}
