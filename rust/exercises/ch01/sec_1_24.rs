// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.24: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_24 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.24`.
        pub exercise: &'static str,
    }

    /// Exercise 1.24: the timed Fermat test
    ///
    /// Returns whether the timed Fermat test reports every one of the 12
    /// primes found in exercise 1.22 as prime, drawing its witnesses from
    /// the seeded generator of 1.2.6.
    pub fn ex_1_24() -> Result<bool, Pending> {
        Err(Pending { exercise: "1.24" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_24() {
    let all_reported_prime = ex_1_24::ex_1_24().unwrap_or(false);
    assert!(all_reported_prime);
}
