// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.33: quote produces lazy lists.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_33 {
    //! Exercise 4.33: quote produces lazy lists.

    use super::Pending;

    /// Answers the plain evaluator's error on a quoted list and the lifted-quote session values.
    pub fn ex_4_33() -> Result<(String, Vec<String>), Pending> {
        Err(Pending { exercise: "4.33" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_33() {
    let (plain, lifted) = ex_4_33::ex_4_33().expect("solved");
    assert!(plain.contains("not a procedure"));
    assert_eq!(lifted.first(), Some(&"a".to_owned()));
}
