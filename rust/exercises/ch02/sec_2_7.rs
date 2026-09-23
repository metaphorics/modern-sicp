// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.7, one module and one ignored test.

mod ex_2_07 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.7`.
        pub exercise: &'static str,
    }

    /// Exercise 2.7: the interval selectors that complete `make-interval`
    ///
    /// Returns the lower and upper bounds of the interval built from
    /// `6.0` and `8.0`.
    pub fn ex_2_07() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "2.7" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_07() {
    assert_eq!(ex_2_07::ex_2_07(), Ok((6.0, 8.0)));
}
