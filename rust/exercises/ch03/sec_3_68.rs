// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.68: Louis Reasoner's pairs, which
//! appends the whole infinite first row to the recursive call, driven
//! under a step budget that measures the construction never finishing.

mod ex_3_68 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.68`.
        pub exercise: &'static str,
    }

    /// Exercise 3.68: Louis pairs infinite recursion
    ///
    /// Answers the value of the first element, whether the attempt to
    /// build past it died in the metered budget panic, and the meter's
    /// final count: `((1, 1), true, budget)` on the working run.
    pub fn ex_3_68() -> Result<((i128, i128), bool, u32), Pending> {
        Err(Pending { exercise: "3.68" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_68() {
    let (first, starved, tail_steps) = ex_3_68::ex_3_68().expect("solved");
    // The first element's value is the row's head pair (1, 1); the
    // tail-building for index 1 never finishes: it burned the full
    // budget and produced no element, because stream-append cannot
    // reach past the infinite first row.
    assert_eq!(first, (1, 1));
    assert!(starved);
    assert_eq!(tail_steps, 4_000);
}
