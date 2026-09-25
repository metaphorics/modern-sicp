// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.32: chapter 3 streams versus lazy lists.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_32 {
    //! Exercise 4.32: chapter 3 streams versus lazy lists.

    use super::Pending;

    /// Answers the lazy skip of an armed slot and the strict constructor's error over the same pair.
    pub fn ex_4_32() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.32" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_32() {
    let (lazy, strict) = ex_4_32::ex_4_32().expect("solved");
    assert_eq!(lazy, "7");
    assert!(strict.contains("division by zero"));
}
mod ex_4_32a {
    //! Exercise 4.32a: build lazy tree, force selectively.

    use super::Pending;

    /// Answers the printable transcript of the selectively forced lazy
    /// tree, whose counters pin which subtrees computed.
    pub fn ex_4_32a() -> Result<String, Pending> {
        Err(Pending { exercise: "4.32a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_32a() {
    let transcript = ex_4_32a::ex_4_32a().expect("solved");
    assert!(transcript.contains(
        ";;; L-Eval value: 2\
    "
    ));
}
