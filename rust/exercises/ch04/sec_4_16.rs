// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.16: Exercise 4.16: scan out internal definitions..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_16 {
    //! Exercise 4.16: scan out internal definitions.

    use super::Pending;

    /// Answers the value of the mutually recursive `f` under the
    /// scanned-out evaluator and the premature-use error on `g`.
    pub fn ex_4_16() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.16" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_16() {
    let (f, premature) = ex_4_16::ex_4_16().expect("solved");
    assert_eq!(f, "true");
    assert!(premature.contains("definition"));
}
