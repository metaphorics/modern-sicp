// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.4: `and` and `or` as special forms..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_04 {
    //! Exercise 4.4: `and` and `or` as special forms.

    use super::Pending;

    /// Answers the printed values of the section's and/or probes.
    pub fn ex_4_04() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.4" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_04() {
    let values = ex_4_04::ex_4_04().expect("solved");
    assert_eq!(values, vec!["#t", "3", "#f", "#f", "7", "#f"]);
}
