// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.20: Exercise 4.20: `letrec` as a derived expression..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_20 {
    //! Exercise 4.20: `letrec` as a derived expression.

    use super::Pending;

    /// Answers the value of a letrec of mutually recursive procedures
    /// and the premature-use error inside a letrec binding.
    pub fn ex_4_20() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.20" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_20() {
    let (mutual, premature) = ex_4_20::ex_4_20().expect("solved");
    assert_eq!(mutual, "#t");
    assert!(premature.contains("before its define runs"));
}
