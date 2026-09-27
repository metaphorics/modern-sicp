// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.28: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_28 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.28`.
        pub exercise: &'static str,
    }

    /// Exercise 1.28: the Miller-Rabin test
    ///
    /// Returns whether every prime below 100 passes the Miller-Rabin test
    /// for every witness first, and whether every one of the six
    /// Carmichael numbers is rejected by at least one witness second.
    pub fn ex_1_28() -> Result<(bool, bool), Pending> {
        Err(Pending { exercise: "1.28" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_28() {
    let (primes_pass, carmichaels_rejected) = ex_1_28::ex_1_28().unwrap_or((false, false));
    assert!(primes_pass);
    assert!(carmichaels_rejected);
}
