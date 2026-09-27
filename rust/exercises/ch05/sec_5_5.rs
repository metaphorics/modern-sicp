// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of exercises 5.5 and 5.5a: hand-simulated
//! machines, and the edition's restore annotations.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.5`.
    pub exercise: &'static str,
}

mod ex_5_05 {
    //! Exercise 5.5: hand-simulate the factorial and Fibonacci
    //! machines, showing the stack at each significant point.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_05() -> Result<String, Pending> {
        Err(Pending { exercise: "5.5" })
    }
}

mod ex_5_05a {
    //! Exercise 5.5a (this edition): annotate every restore in the
    //! Fibonacci trace with the save it matches.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_05a() -> Result<String, Pending> {
        Err(Pending { exercise: "5.5a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_05() {
    let answer = ex_5_05::ex_5_05().expect("solved");
    assert!(!answer.is_empty());
}

#[test]
#[ignore = "pending solution"]
fn ex_5_05a() {
    let answer = ex_5_05a::ex_5_05a().expect("solved");
    assert!(!answer.is_empty());
}
