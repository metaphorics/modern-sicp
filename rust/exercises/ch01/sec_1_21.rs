// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.21: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_21 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.21`.
        pub exercise: &'static str,
    }

    /// Exercise 1.21: smallest divisors
    ///
    /// Returns the smallest divisors of 199, 1999, and 19999 in that
    /// order.
    pub fn ex_1_21() -> Result<[u64; 3], Pending> {
        Err(Pending { exercise: "1.21" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_21() {
    let values = ex_1_21::ex_1_21().unwrap_or([0; 3]);
    assert_eq!(values, [199, 1999, 7]);
}
