// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.71: explicit delay in simple-query.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_71 {
    //! Exercise 4.71: why simple-query and disjoin carry explicit delays.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_4_71() -> Result<String, Pending> {
        Err(Pending { exercise: "4.71" })
    }
}

mod ex_4_71a {
    //! Exercise 4.71a (edition addition): the divergence probe. Find a
    //! query where Louis's undelayed simple-query or disjoin gives a
    //! different result than the delayed engine.

    use super::Pending;

    /// Answers the addition's result once the solution lands.
    pub fn ex_4_71a() -> Result<String, Pending> {
        Err(Pending { exercise: "4.71a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_71() {
    let answer = ex_4_71::ex_4_71().expect("solved");
    assert!(!answer.is_empty());
}

#[test]
#[ignore = "pending solution"]
fn ex_4_71a() {
    let answer = ex_4_71a::ex_4_71a().expect("solved");
    assert!(!answer.is_empty());
}
