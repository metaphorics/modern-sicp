// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.3: data-directed dispatch over typed constructors.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_03 {
    //! Exercise 4.3: data-directed dispatch over typed constructors.

    use super::Pending;

    /// Evaluates a vector datum, an `if`, and a table-dispatched call
    /// through the put/get dispatch table, answering the printed values.
    pub fn ex_4_03() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.3" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_03() {
    let values = ex_4_03::ex_4_03().expect("solved");
    assert!(values[0].contains('a') && values[0].contains('b'));
    assert_eq!(&values[1..], &["42", "49"]);
}
