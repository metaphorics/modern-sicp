// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.11.

mod ex_3_11 {

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.11`.
        pub exercise: &'static str,
    }

    /// Exercise 3.11: where account state lives
    ///
    /// Runs the book's interaction on one account, probes object
    /// identity with `same_object_as` between an alias and the original
    /// and between two separately built accounts, deposits through the
    /// alias and reads through the original, and reports whether the
    /// two accounts' model frames are distinct while both extending the
    /// same global frame. Returns `(alias is the same object, second
    /// account is the same object, balance seen through the original,
    /// frames distinct, frames share the global frame)`.
    pub fn ex_3_11() -> Result<(bool, bool, i128, bool, bool), Pending> {
        Err(Pending { exercise: "3.11" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_11() {
    let (alias_same, acc2_same, balance, frames_distinct, frames_share_global) =
        ex_3_11::ex_3_11().expect("solved");
    assert!(alias_same);
    assert!(!acc2_same);
    assert_eq!(balance, 30);
    assert!(frames_distinct);
    assert!(frames_share_global);
}
