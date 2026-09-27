// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.25: unless breaks under applicative order.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_25 {
    //! Exercise 4.25: unless breaks under applicative order.

    use super::Pending;

    /// Answers the value of `(factorial 5)` under the lazy evaluator, where the recursive operand delays.
    pub fn ex_4_25() -> Result<String, Pending> {
        Err(Pending { exercise: "4.25" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_25() {
    let answer = ex_4_25::ex_4_25().expect("solved");
    assert_eq!(answer, "120");
}
