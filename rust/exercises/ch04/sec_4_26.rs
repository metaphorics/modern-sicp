// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.26: `unless` eagerly expanded versus delayed.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_26 {
    //! Exercise 4.26: `unless` eagerly expanded versus delayed.

    use super::Pending;

    /// Answers Ben's eagerly-expanded `unless` value and Alyssa's delayed-operand `unless` value over the same armed call.
    pub fn ex_4_26() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.26" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_26() {
    let (special, procedure) = ex_4_26::ex_4_26().expect("solved");
    assert_eq!(special, "42");
    assert_eq!(procedure, "42");
}
