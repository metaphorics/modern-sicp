// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.22: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_22 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.22`.
        pub exercise: &'static str,
    }

    /// Exercise 1.22: the timed prime search
    ///
    /// Returns the three smallest primes larger than 1000, then the three
    /// smallest primes larger than 1,000,000.
    pub fn ex_1_22() -> Result<[u64; 6], Pending> {
        Err(Pending { exercise: "1.22" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_22() {
    let values = ex_1_22::ex_1_22().unwrap_or([0; 6]);
    assert_eq!(values, [1009, 1013, 1019, 1_000_003, 1_000_033, 1_000_037]);
}
