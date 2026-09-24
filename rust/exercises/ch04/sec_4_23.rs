// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.23: Exercise 4.23: the two `analyze-sequence` versions..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_23 {
    //! Exercise 4.23: the two `analyze-sequence` versions.

    use super::Pending;

    /// Answers how many analysis invocations a one-expression body
    /// costs under each version, and the shared values of one- and
    /// two-expression bodies.
    pub fn ex_4_23() -> Result<(u64, u64, Vec<String>), Pending> {
        Err(Pending { exercise: "4.23" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_23() {
    let (text_analyses, alyssa_analyses, values) = ex_4_23::ex_4_23().expect("solved");
    assert_eq!(text_analyses, alyssa_analyses);
    assert_eq!(values, vec!["7", "7", "3", "3"]);
}

mod ex_4_23a {
    //! Exercise 4.23a (this edition): counting the sequencing work.

    use super::Pending;

    /// Answers how many times each version's sequence execution
    /// procedure runs when two one-expression bodies execute twice.
    pub fn ex_4_23a() -> Result<(u64, u64), Pending> {
        Err(Pending { exercise: "4.23a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_23a() {
    let (text_execs, alyssa_execs) = ex_4_23a::ex_4_23a().expect("solved");
    assert_eq!((text_execs, alyssa_execs), (0, 4));
}
