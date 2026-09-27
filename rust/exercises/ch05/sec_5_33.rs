// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.33: the alternative factorial's compilation and its differences.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.33`.
    pub exercise: &'static str,
}

mod ex_5_33 {
    //! Exercise 5.33: the alternative factorial's compilation and its differences.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_33() -> Result<String, Pending> {
        Err(Pending { exercise: "5.33" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_33() {
    let answer = ex_5_33::ex_5_33().expect("solved");
    assert!(!answer.is_empty());
}
