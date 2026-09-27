// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.3: the square-root machine in
//! two stages.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.3`.
    pub exercise: &'static str,
}

mod ex_5_03 {
    //! Exercise 5.3: design a square-root machine on Newton's method,
    //! with the compound operations first primitive, then expanded.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_03() -> Result<String, Pending> {
        Err(Pending { exercise: "5.3" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_03() {
    let answer = ex_5_03::ex_5_03().expect("solved");
    assert!(!answer.is_empty());
}
