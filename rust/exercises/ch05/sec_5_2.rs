// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.2: the iterative factorial
//! controller sequence.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.2`.
    pub exercise: &'static str,
}

mod ex_5_02 {
    //! Exercise 5.2: describe the iterative factorial machine of
    //! exercise 5.1 in the register-machine language.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_02() -> Result<String, Pending> {
        Err(Pending { exercise: "5.2" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_02() {
    let answer = ex_5_02::ex_5_02().expect("solved");
    assert!(!answer.is_empty());
}
