// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.21: Exercise 4.21: recursion without `define`..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_21 {
    //! Exercise 4.21: recursion without `define`.

    use super::Pending;

    /// Answers the book's applicative-order factorial at 10 and the
    /// completed `f` at 10.
    pub fn ex_4_21() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.21" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_21() {
    let values = ex_4_21::ex_4_21().expect("solved");
    assert_eq!(values, vec!["3628800", "#t"]);
}
