// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.44: open-coding respects shadowed names.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.44`.
    pub exercise: &'static str,
}

mod ex_5_44 {
    //! Exercise 5.44: open-coding respects shadowed names.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_44() -> Result<String, Pending> {
        Err(Pending { exercise: "5.44" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_44() {
    let answer = ex_5_44::ex_5_44().expect("solved");
    assert!(!answer.is_empty());
}
