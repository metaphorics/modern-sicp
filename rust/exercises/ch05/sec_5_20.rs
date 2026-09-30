// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 5.20: the box-and-pointer and
//! memory-vector drawings of two definitions run on the
//! list-structure memory.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `5.20`.
    pub exercise: &'static str,
}

mod ex_5_20 {
    //! Exercise 5.20: draw the box-and-pointer and memory-vector
    //! representations of the structure produced by allocating the
    //! pair `x` from 1 and 2, then the pair `y` from `x` and `x`, with
    //! the free pointer initially `p1`. What is the final value of
    //! `free`? What pointers represent the values of `x` and `y`?

    use super::Pending;

    /// Answers the exercise's result once the solution lands.
    pub fn ex_5_20() -> Result<String, Pending> {
        Err(Pending { exercise: "5.20" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_5_20() {
    let answer = ex_5_20::ex_5_20().expect("solved");
    assert!(!answer.is_empty());
}
