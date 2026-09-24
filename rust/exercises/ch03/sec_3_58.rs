// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.58: the long-division digit
//! stream of `expand`.

mod ex_3_58 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.58`.
        pub exercise: &'static str,
    }

    /// Exercise 3.58: expand computes long division digits
    ///
    /// Answers the first 8 digits the stream of `(expand 1 7 10)`
    /// produces and the first 6 of `(expand 3 8 10)`.
    pub fn ex_3_58() -> Result<(Vec<i128>, Vec<i128>), Pending> {
        Err(Pending { exercise: "3.58" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_58() {
    let (sevenths, eighths) = ex_3_58::ex_3_58().expect("solved");
    assert_eq!(sevenths, [1, 4, 2, 8, 5, 7, 1, 4]);
    assert_eq!(eighths, [3, 7, 5, 0, 0, 0]);
}
