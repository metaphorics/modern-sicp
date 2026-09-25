// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.29: memoization speed difference.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_29 {
    //! Exercise 4.29: memoization speed difference.

    use super::Pending;

    /// Answers the counting session under memoized and unmemoized forcing.
    pub fn ex_4_29() -> Result<(Vec<String>, Vec<String>), Pending> {
        Err(Pending { exercise: "4.29" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_29() {
    let (memo, no_memo) = ex_4_29::ex_4_29().expect("solved");
    assert_eq!(memo.last(), Some(&"1000".to_owned()));
    assert_ne!(memo, no_memo);
}
