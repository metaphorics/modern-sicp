// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.28: forcing the operator.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_28 {
    //! Exercise 4.28: forcing the operator.

    use super::Pending;

    /// Answers the value of an application whose operator arrives delayed, and the error without operator forcing.
    pub fn ex_4_28() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.28" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_28() {
    let (forced, error) = ex_4_28::ex_4_28().expect("solved");
    assert_eq!(forced, "5");
    assert!(error.contains("thunk"));
}
