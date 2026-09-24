// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 3.55: the `partial-sums` combinator,
//! answered by the statement's own example and cross-checked against the
//! section module's version.

mod ex_3_55 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `3.55`.
        pub exercise: &'static str,
    }

    /// Exercise 3.55: partial-sums combinator
    ///
    /// Answers the first five partial sums of `integers` from the local
    /// definition and from the section's `partial_sums`, which must
    /// agree over the prefix.
    pub fn ex_3_55() -> Result<(Vec<i128>, Vec<i128>), Pending> {
        Err(Pending { exercise: "3.55" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_3_55() {
    let (local, library) = ex_3_55::ex_3_55().expect("solved");
    // The n-th partial sum of the integers is the triangular number
    // 1 + ... + (n + 1), the statement's example stream.
    assert_eq!(local, vec![1, 3, 6, 10, 15]);
    assert_eq!(library, vec![1, 3, 6, 10, 15]);
}
