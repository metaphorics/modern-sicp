// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.27: lazy identity with assignment.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_27 {
    //! Exercise 4.27: lazy identity with assignment.

    use super::Pending;

    /// Answers the printed session values of the lazy `id(id(10))` interaction, in order.
    pub fn ex_4_27() -> Result<Vec<String>, Pending> {
        Err(Pending { exercise: "4.27" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_27() {
    let lines = ex_4_27::ex_4_27().expect("solved");
    assert_eq!(
        &lines[..3],
        &["1".to_owned(), "10".to_owned(), "2".to_owned()]
    );
}
