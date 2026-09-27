// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.24: cond as a basic controller form, without reducing it to if.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.24`.
    pub exercise: &'static str,
}

mod ex_5_24 {
    //! Exercise 5.24: cond as a basic controller form, without reducing it to if.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_24() -> Result<String, Pending> {
        Err(Pending { exercise: "5.24" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_24() {
    let answer = ex_5_24::ex_5_24().expect("solved");
    assert!(!answer.is_empty());
}
