// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of instruction tracing on and off.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.16`.
    pub exercise: &'static str,
}

mod ex_5_16 {
    //! Exercise 5.16: print the text of every instruction before it
    //! executes, switchable at run time.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_16() -> Result<String, Pending> {
        Err(Pending { exercise: "5.16" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_16() {
    let answer = ex_5_16::ex_5_16().expect("solved");
    assert!(!answer.is_empty());
}
