// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.12, one module and one ignored
//! test.

mod ex_2_12 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.12`.
        pub exercise: &'static str,
    }

    /// Exercise 2.12: `make-center-percent` and `percent`
    ///
    /// Returns the lower bound, upper bound, and recovered percentage
    /// tolerance of the interval built from center `100.0` and tolerance
    /// `5.0` percent.
    pub fn ex_2_12() -> Result<(f64, f64, f64), Pending> {
        Err(Pending { exercise: "2.12" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_12() {
    assert_eq!(ex_2_12::ex_2_12(), Ok((95.0, 105.0, 5.0)));
}
