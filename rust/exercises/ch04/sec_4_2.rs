// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.2: dispatch order and call-position bindings.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_02 {
    //! Exercise 4.2: dispatch order and call-position bindings.

    use super::Pending;

    /// Answers the error of `let x: i64 = 3;` in call position under
    /// applications-first dispatch, and the value of the explicitly applied factorial at 5.
    pub fn ex_4_02() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.2" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_02() {
    let (call_error, call_factorial) = ex_4_02::ex_4_02().expect("solved");
    assert!(call_error.contains("let"));
    assert_eq!(call_factorial, "120");
}
