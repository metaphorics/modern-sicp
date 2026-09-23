// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.13: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_13 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.13`.
        pub exercise: &'static str,
    }

    /// Exercise 1.13: `Fib(n)` is the closest integer to `phi^n / sqrt(5)`
    ///
    /// The exercise is a proof; the code companion verifies its numerical
    /// edge: it returns whether `Fib(n)` is the closest integer to
    /// `phi^n / sqrt(5)` for every `n` from 0 through 70, where the
    /// floating-point argument is still exact enough for the rounding
    /// claim to bite.
    pub fn ex_1_13() -> Result<bool, Pending> {
        Err(Pending { exercise: "1.13" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_13() {
    let holds = ex_1_13::ex_1_13().unwrap_or(false);
    assert!(holds);
}
