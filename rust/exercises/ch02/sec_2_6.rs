// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.6, one module and one ignored test.

mod ex_2_06 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.6`.
        pub exercise: &'static str,
    }

    /// Exercise 2.6: Church numerals `one` and `two`, and `+`, all
    /// defined directly
    ///
    /// Returns `one`, `two`, and `one + two`, each converted to an
    /// ordinary integer.
    pub fn ex_2_06() -> Result<(i128, i128, i128), Pending> {
        Err(Pending { exercise: "2.6" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_06() {
    assert_eq!(ex_2_06::ex_2_06(), Ok((1, 2, 3)));
}
