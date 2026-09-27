// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.38: open-coded primitives over the argument registers.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.38`.
    pub exercise: &'static str,
}

mod ex_5_38 {
    //! Exercise 5.38: open-coded primitives over the argument registers.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_38() -> Result<String, Pending> {
        Err(Pending { exercise: "5.38" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_38() {
    let answer = ex_5_38::ex_5_38().expect("solved");
    assert!(!answer.is_empty());
}
