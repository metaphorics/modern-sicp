// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.22: the register machines that
//! append two lists and splice two lists together over the
//! list-structure memory.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.22`.
    pub exercise: &'static str,
}

mod ex_5_22 {
    //! Exercise 5.22: design a register machine to implement
    //! `append`, which appends two lists to form a new list, and one
    //! to implement `append!`, which splices two lists together. The
    //! list-structure memory operations are the primitive operations.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_22() -> Result<String, Pending> {
        Err(Pending { exercise: "5.22" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_22() {
    let answer = ex_5_22::ex_5_22().expect("solved");
    assert!(!answer.is_empty());
}
