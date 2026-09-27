// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.13: Exercise 4.13: `make-unbound!` removes a binding..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_13 {
    //! Exercise 4.13: `make-unbound!` removes a binding.

    use super::Pending;

    /// Answers the lookups after unbinding a shadowing name and after
    /// unbinding a name the first frame does not bind.
    pub fn ex_4_13() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.13" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_13() {
    let values = ex_4_13::ex_4_13().expect("solved");
    assert_eq!(
        values,
        vec!["1", "make-unbound!: not bound in the first frame: x"]
    );
}
