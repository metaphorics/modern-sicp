// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.50: ramble random choice.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_50 {
    //! Exercise 4.50: ramble random choice.

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_4_50() -> Result<String, Pending> {
        Err(Pending { exercise: "4.50" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_50() {
    let answer = ex_4_50::ex_4_50().expect("solved");
    assert!(!answer.is_empty());
}

mod ex_4_50a {
    //! Exercise 4.50a (tailored): seed the host RNG for reproducible ramb.

    use super::Pending;

    /// Answers the tailored exercise's result once the solution lands.
    pub fn ex_4_50a() -> Result<String, Pending> {
        Err(Pending { exercise: "4.50a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_50a() {
    let answer = ex_4_50a::ex_4_50a().expect("solved");
    assert!(!answer.is_empty());
}
