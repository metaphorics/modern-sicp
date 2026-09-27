// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.16, one module and one ignored
//! test.

mod ex_2_16 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.16`.
        pub exercise: &'static str,
    }

    /// Exercise 2.16: equivalent algebraic expressions can give different
    /// answers
    ///
    /// Returns the lower and upper bounds of `x - x` for `x` at 10.0
    /// with 5 percent tolerance, the simplest case of the dependency
    /// problem underlying exercises 2.14 and 2.15.
    pub fn ex_2_16() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "2.16" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_16() {
    assert_eq!(ex_2_16::ex_2_16(), Ok((-1.0, 1.0)));
}
