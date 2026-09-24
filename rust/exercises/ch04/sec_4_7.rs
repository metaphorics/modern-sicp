// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.7: Exercise 4.7: `let*` as nested `let`s..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_07 {
    //! Exercise 4.7: `let*` as nested `let`s.

    use super::Pending;

    /// Answers the value of the book's `let*` example and of a `let*`
    /// nested inside another's body.
    pub fn ex_4_07() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.7" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_07() {
    let values = ex_4_07::ex_4_07().expect("solved");
    assert_eq!(values, vec!["39", "5"]);
}
