// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.13: unbinding removes a frame entry.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_13 {
    //! Exercise 4.13: unbinding removes a frame entry.

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
    assert_eq!(values.len(), 2);
    assert_eq!(values[0], "1");
    assert!(values[1].starts_with("make-unbound"));
    assert!(values[1].contains("first frame") && values[1].contains('x'));
}
