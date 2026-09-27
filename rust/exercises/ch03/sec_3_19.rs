// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of exercise 3.19 and its tailored addition
//! 3.19a: tortoise-hare cycle detection in constant space, and
//! shared-suffix detection by the same pointer alignment.

mod ex_3_19 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.19`.
        pub exercise: &'static str,
    }

    /// Exercise 3.19: tortoise-hare cycle detection in constant space
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((bool, bool))`; the pending body reports [`Pending`].
    pub fn ex_3_19() -> Result<(bool, bool), Pending> {
        Err(Pending { exercise: "3.19" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_19() {
    let _ = ex_3_19::ex_3_19();
}

mod ex_3_19a {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.19a`.
        pub exercise: &'static str,
    }

    /// Exercise 3.19a: shared suffix detection
    ///
    /// The solved entry point returns the exercise's answers as
    /// `((bool, bool))`; the pending body reports [`Pending`].
    pub fn ex_3_19a() -> Result<(bool, bool), Pending> {
        Err(Pending { exercise: "3.19a" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_19a() {
    let _ = ex_3_19a::ex_3_19a();
}
