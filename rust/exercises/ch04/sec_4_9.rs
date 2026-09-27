// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.9: Exercise 4.9: iteration constructs as derived expressions..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_09 {
    //! Exercise 4.9: iteration constructs as derived expressions.

    use super::Pending;

    /// Answers the loop counters a `while` and an `until` produce.
    pub fn ex_4_09() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.9" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_09() {
    let values = ex_4_09::ex_4_09().expect("solved");
    assert_eq!(values, vec!["5", "13"]);
}
