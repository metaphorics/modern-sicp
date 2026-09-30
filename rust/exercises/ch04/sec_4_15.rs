// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.15: the halting diagonal over step budgets.

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_15 {
    //! Exercise 4.15: the halting diagonal over step budgets.

    use super::Pending;

    /// Answers the two outcomes of the self-applied `halts` probe under
    /// two fixed verdicts: the step budget that proves the first run
    /// never halts, and the `halted` value that contradicts the second.
    pub fn ex_4_15() -> Result<(String, String), Pending> {
        Err(Pending { exercise: "4.15" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_15() {
    let (optimistic, pessimistic) = ex_4_15::ex_4_15().expect("solved");
    assert!(optimistic.contains("step budget"));
    assert_eq!(pessimistic, "halted");
}
