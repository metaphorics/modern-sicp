// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.40: the compile-time environment threading.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.40`.
    pub exercise: &'static str,
}

mod ex_5_40 {
    //! Exercise 5.40: the compile-time environment threading.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_40() -> Result<String, Pending> {
        Err(Pending { exercise: "5.40" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_40() {
    let answer = ex_5_40::ex_5_40().expect("solved");
    assert!(!answer.is_empty());
}
