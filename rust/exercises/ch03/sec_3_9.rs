// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.9.

mod ex_3_09 {

    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.9`.
        pub exercise: &'static str,
    }

    /// Exercise 3.9: environment structures of two factorials
    ///
    /// Returns the value and the deepest frame depth of the recursive
    /// factorial at 6, then the value and the deepest depth of the loop
    /// version.
    pub fn ex_3_09() -> Result<(u128, u64, u128, u64), Pending> {
        Err(Pending { exercise: "3.9" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_09() {
    let (recursive_value, recursive_depth, loop_value, loop_depth) =
        ex_3_09::ex_3_09().expect("solved");
    assert_eq!((recursive_value, loop_value), (720, 720));
    assert_eq!((recursive_depth, loop_depth), (6, 1));
}
