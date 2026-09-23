// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.23: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_23 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.23`.
        pub exercise: &'static str,
    }

    /// Exercise 1.23: skipping even divisors
    ///
    /// Returns the three smallest primes larger than 1,000,000 as the
    /// `next`-stepped `smallest_divisor` finds them: the same values
    /// `smallest_divisor` finds, by fewer divisor tests.
    pub fn ex_1_23() -> Result<[u64; 3], Pending> {
        Err(Pending { exercise: "1.23" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_23() {
    let values = ex_1_23::ex_1_23().unwrap_or([0; 3]);
    assert_eq!(values, [1_000_003, 1_000_033, 1_000_037]);
}
