// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.3: Exercise 4.3: data-directed dispatch in `eval`..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_03 {
    //! Exercise 4.3: data-directed dispatch in `eval`.

    use super::Pending;

    /// Evaluates a quotation, an `if`, and a defined call through the
    /// put/get dispatch table, answering the printed values.
    pub fn ex_4_03() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.3" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_03() {
    let values = ex_4_03::ex_4_03().expect("solved");
    assert_eq!(values, vec!["(a b)", "42", "49"]);
}
