// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.43: scanning out internal definitions.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.43`.
    pub exercise: &'static str,
}

mod ex_5_43 {
    //! Exercise 5.43: scanning out internal definitions.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_43() -> Result<String, Pending> {
        Err(Pending { exercise: "5.43" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_43() {
    let answer = ex_5_43::ex_5_43().expect("solved");
    assert!(!answer.is_empty());
}
