// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of running the exercise 5.4 machines on the simulator.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.7`.
    pub exercise: &'static str,
}

mod ex_5_07 {
    //! Exercise 5.7: run the recursive and iterative exponentiation
    //! machines of exercise 5.4 on the section's simulator.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_07() -> Result<String, Pending> {
        Err(Pending { exercise: "5.7" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_07() {
    let answer = ex_5_07::ex_5_07().expect("solved");
    assert!(!answer.is_empty());
}
