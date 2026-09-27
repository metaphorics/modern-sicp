// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.34: printing lazy pairs.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_34 {
    //! Exercise 4.34: printing lazy pairs.

    use super::Pending;

    /// Answers the printable driver transcript over an infinite lazy list and a nested pair.
    pub fn ex_4_34() -> Result<String, Pending> {
        Err(Pending { exercise: "4.34" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_34() {
    let transcript = ex_4_34::ex_4_34().expect("solved");
    assert!(transcript.contains("(1 1 1 1 1 1 1 1 1 1 ...)"));
}
