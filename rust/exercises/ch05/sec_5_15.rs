// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of exercises 5.15 and 5.15a: counting, and the budget.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.15`.
    pub exercise: &'static str,
}

mod ex_5_15 {
    //! Exercise 5.15: count executed instructions, print and reset
    //! the count.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_15() -> Result<String, Pending> {
        Err(Pending { exercise: "5.15" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_15() {
    let answer = ex_5_15::ex_5_15().expect("solved");
    assert!(!answer.is_empty());
}

mod ex_5_15a {
    //! Exercise 5.15a (this edition): extend the counting
    //! machine with a budget; a run due to pass it halts with a
    //! typed fault carrying the count and the program counter.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_15a() -> Result<String, Pending> {
        Err(Pending { exercise: "5.15a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_15a() {
    let answer = ex_5_15a::ex_5_15a().expect("solved");
    assert!(!answer.is_empty());
}
