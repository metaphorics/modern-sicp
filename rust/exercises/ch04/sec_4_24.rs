// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 4.24: Exercise 4.24: analysis versus execution time..

/// The typed pending report of an unsolved scaffold: the body returns
/// this instead of panicking, so the failure names its origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pending {
    /// The exercise number the scaffold belongs to, as `4.1`.
    pub exercise: &'static str,
}

mod ex_4_24 {
    //! Exercise 4.24: analysis versus execution time.

    use super::Pending;

    /// Answers the per-call nanoseconds of the base evaluator and of a
    /// pre-analyzed execution over the same fib workload, with warmup.
    pub fn ex_4_24() -> Result<(u128, u128), Pending> {
        Err(Pending { exercise: "4.24" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_4_24() {
    let (base_ns, analyzed_ns) = ex_4_24::ex_4_24().expect("solved");
    assert!(
        analyzed_ns < base_ns,
        "analysis amortizes: {analyzed_ns} < {base_ns}"
    );
}
