// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.17: Exercise 4.17: the extra frame of the scanned-out program..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_17 {
    //! Exercise 4.17: the extra frame of the scanned-out program.

    use super::Pending;

    /// Answers the value `f` gives under both mechanisms and the
    /// printed scanned-out body whose `let` is the extra frame.
    pub fn ex_4_17() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.17" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_17() {
    let (value, scanned_body) = ex_4_17::ex_4_17().expect("solved");
    assert_eq!(value, "21");
    assert!(scanned_body.starts_with("(let ((u *unassigned*) (v *unassigned*))"));
}
