// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.48: the compile-and-run primitive.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.48`.
    pub exercise: &'static str,
}

mod ex_5_48 {
    //! Exercise 5.48: the compile-and-run primitive.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_48() -> Result<String, Pending> {
        Err(Pending { exercise: "5.48" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_48() {
    let answer = ex_5_48::ex_5_48().expect("solved");
    assert!(!answer.is_empty());
}
