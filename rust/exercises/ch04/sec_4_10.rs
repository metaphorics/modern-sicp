// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.10: Exercise 4.10: new syntax, unchanged eval..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_10 {
    //! Exercise 4.10: new syntax, unchanged eval.

    use super::Pending;

    /// Answers the value of a `defun`-spelled square under the reader
    /// transform, and the error the same form raises without it.
    pub fn ex_4_10() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.10" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_10() {
    let (with_transform, without) = ex_4_10::ex_4_10().expect("solved");
    assert_eq!(with_transform, "49");
    assert!(without.contains("defun"));
}
