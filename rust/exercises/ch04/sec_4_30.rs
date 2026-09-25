// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.30: forcing in eval-sequence.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_30 {
    //! Exercise 4.30: forcing in eval-sequence.

    use super::Pending;

    /// Answers the value of `(p2 1)` under the text's sequence and under Cy's forced sequence.
    pub fn ex_4_30() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.30" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_30() {
    let (text_sequence, cy_sequence) = ex_4_30::ex_4_30().expect("solved");
    assert_eq!(text_sequence, "1");
    assert_eq!(cy_sequence, "(1 2)");
}
