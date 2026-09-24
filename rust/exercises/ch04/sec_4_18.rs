// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.18: Exercise 4.18: the alternative scan-out strategy..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_18 {
    //! Exercise 4.18: the alternative scan-out strategy.

    use super::Pending;

    /// Answers the value the text's scan-out gives for a body whose
    /// second initializer reads the first name, and the error the
    /// alternative strategy raises on the same body.
    pub fn ex_4_18() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.18" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_18() {
    let (text_strategy, alternative) = ex_4_18::ex_4_18().expect("solved");
    assert_eq!(text_strategy, "10");
    assert!(alternative.contains("before its define runs"));
}
