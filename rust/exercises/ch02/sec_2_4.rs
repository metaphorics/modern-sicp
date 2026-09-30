// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.4, one module and one ignored test.

mod ex_2_04 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `2.4`.
        pub exercise: &'static str,
    }

    /// Exercise 2.4: a procedural representation of pairs
    ///
    /// Returns the first and second components recovered from the
    /// closure-backed pair created with `(3, 4)`.
    pub fn ex_2_04() -> Result<(i64, i64), Pending> {
        Err(Pending { exercise: "2.4" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_04() {
    assert_eq!(ex_2_04::ex_2_04(), Ok((3, 4)));
}
