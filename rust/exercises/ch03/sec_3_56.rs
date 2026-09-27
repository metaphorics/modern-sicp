// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.56: the Hamming stream, 1 consed
//! onto the merge of the stream's own scalings by 2, 3, and 5.

mod ex_3_56 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.56`.
        pub exercise: &'static str,
    }

    /// Exercise 3.56: Hamming numbers via merge
    ///
    /// Answers the first 15 elements of the ascending stream, with no
    /// repetitions, of the positive integers whose only prime factors
    /// are 2, 3, and 5.
    pub fn ex_3_56() -> Result<Vec<i128>, Pending> {
        Err(Pending { exercise: "3.56" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_56() {
    let first = ex_3_56::ex_3_56().expect("solved");
    assert_eq!(first, [1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24]);
}
