// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.6: Exercise 4.6: `let` as a derived expression..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_06 {
    //! Exercise 4.6: `let` as a derived expression.

    use super::Pending;

    /// Answers the printed `let->combination` rewrite and the value of
    /// the let that rewrite evaluates to.
    pub fn ex_4_06() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.6" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_06() {
    let (rewritten, value) = ex_4_06::ex_4_06().expect("solved");
    assert_eq!(rewritten, "((lambda (x y) (+ x y)) 3 4)");
    assert_eq!(value, "7");
}
