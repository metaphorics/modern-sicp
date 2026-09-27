// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.8: Exercise 4.8: named `let`..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_08 {
    //! Exercise 4.8: named `let`.

    use super::Pending;

    /// Answers the value of the book's named-let Fibonacci at 10.
    pub fn ex_4_08() -> Result<String, Pending> {
        Err(Pending { exercise: "4.8" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_08() {
    assert_eq!(ex_4_08::ex_4_08().expect("solved"), "55");
}
