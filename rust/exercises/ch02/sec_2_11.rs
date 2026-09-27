// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.11, one module and one ignored
//! test.

mod ex_2_11 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.11`.
        pub exercise: &'static str,
    }

    /// Exercise 2.11: `mul-interval` by nine sign-tested cases
    ///
    /// Returns the bounds of `[2, 6] * [-3, 5]` (a case that spans
    /// zero), computed by the sign-tested procedure.
    pub fn ex_2_11() -> Result<(f64, f64), Pending> {
        Err(Pending { exercise: "2.11" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_11() {
    assert_eq!(ex_2_11::ex_2_11(), Ok((-18.0, 30.0)));
}
