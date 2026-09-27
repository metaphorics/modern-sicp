// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.19: Exercise 4.19: the internal-definition scoping debate..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_19 {
    //! Exercise 4.19: the internal-definition scoping debate.

    use super::Pending;

    /// Answers Ben's sequential result for the book's expression and
    /// Alyssa's error under the scan-out mechanism.
    pub fn ex_4_19() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.19" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_19() {
    let (ben, alyssa) = ex_4_19::ex_4_19().expect("solved");
    assert_eq!(ben, "16");
    assert!(alyssa.contains("before its define runs"));
}
