// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.15: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_15 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.15`.
        pub exercise: &'static str,
    }

    /// Exercise 1.15: the sine reduction process
    ///
    /// Returns the number of times `p` is applied while evaluating
    /// `sine(12.15)` first, and the value of `sine(12.15)` second.
    pub fn ex_1_15() -> Result<(u64, f64), Pending> {
        Err(Pending { exercise: "1.15" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_15() {
    let (applications, value) = ex_1_15::ex_1_15().unwrap_or((0, f64::NAN));
    assert_eq!(applications, 5);
    assert!((value - 12.15_f64.sin()).abs() < 1e-2);
}
