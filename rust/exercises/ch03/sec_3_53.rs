// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.53: the self-referential stream
//! defined as 1 consed onto its own elementwise doubling, answered by
//! the prefix the description predicts.

mod ex_3_53 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.53`.
        pub exercise: &'static str,
    }

    /// Exercise 3.53: predict self-referential doubling stream
    ///
    /// Answers the first eight elements of `s`, defined as 1 consed onto
    /// the elementwise sum of `s` with itself.
    pub fn ex_3_53() -> Result<Vec<i128>, Pending> {
        Err(Pending { exercise: "3.53" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_53() {
    let prefix = ex_3_53::ex_3_53().expect("solved");
    // The tail is s + s, and adding a stream to itself doubles every
    // element: after the consed 1 the elements are the powers of two.
    assert_eq!(prefix, vec![1, 2, 4, 8, 16, 32, 64, 128]);
}
