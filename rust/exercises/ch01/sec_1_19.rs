// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.19: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_19 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.19`.
        pub exercise: &'static str,
    }

    /// Exercise 1.19: Fibonacci by transformation squaring
    ///
    /// Returns `fib(90)` as the logarithmic process computes it, with the
    /// state carried in `i128` because the pair outgrows 64-bit integers
    /// near `Fib(93)`, together with whether it agrees with a linear
    /// reference for every `n` from 0 through 90.
    pub fn ex_1_19() -> Result<(i128, bool), Pending> {
        Err(Pending { exercise: "1.19" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_19() {
    let (value, agrees) = ex_1_19::ex_1_19().unwrap_or((0, false));
    assert!(agrees);
    assert_eq!(value, 2_880_067_194_370_816_120);
}
