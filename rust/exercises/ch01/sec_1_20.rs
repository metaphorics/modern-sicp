// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.20: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_20 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.20`.
        pub exercise: &'static str,
    }

    /// Exercise 1.20: counting the remainders of the eager `gcd`
    ///
    /// Returns the value of `gcd(206, 40)` first, and the number of
    /// remainder computations the eager evaluation performs while
    /// producing it second.
    pub fn ex_1_20() -> Result<(u64, u64), Pending> {
        Err(Pending { exercise: "1.20" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_20() {
    let (value, remainders) = ex_1_20::ex_1_20().unwrap_or((0, 0));
    assert_eq!(value, 2);
    assert_eq!(remainders, 4);
}
