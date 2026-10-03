// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.1: operand evaluation order under the two operand-framing orders.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_01 {
    //! Exercise 4.1: operand evaluation order, left to right and right to left.

    use super::Pending;

    /// Runs `f` applied to two printing blocks, `{ print!("1"); 1 }` and
    /// `{ print!("2"); 2 }`, under left-to-right and right-to-left operand
    /// evaluation, answering the two printed transcripts.
    pub fn ex_4_01() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.1" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_01() {
    let (left_to_right, right_to_left) = ex_4_01::ex_4_01().expect("solved");
    assert_eq!(left_to_right, "12");
    assert_eq!(right_to_left, "21");
}
