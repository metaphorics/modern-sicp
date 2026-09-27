// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.14: Exercise 4.14: the host `map` as a primitive fails..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_14 {
    //! Exercise 4.14: the host `map` as a primitive fails.

    use super::Pending;

    /// Answers the error Louis's primitive map raises on a compound
    /// procedure and the list Eva's object-language map produces.
    pub fn ex_4_14() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.14" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_14() {
    let (louis, eva) = ex_4_14::ex_4_14().expect("solved");
    assert!(louis.contains("not a procedure"));
    assert_eq!(eva, "(1 4 9)");
}
