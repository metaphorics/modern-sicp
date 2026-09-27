// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.69, one module and one ignored
//! test.

mod ex_2_69 {
    use sicp_runtime::Pending;

    /// Exercise 2.69: `successive-merge` and `generate-huffman-tree`
    ///
    /// Generates a Huffman tree from the pairs `[(A, 4), (B, 2),
    /// (C, 1), (D, 1)]` (the book's `sample-tree` frequencies),
    /// encodes `[A, D, A, B, B, C]` against it, and returns the
    /// number of bits used and whether decoding those bits reproduces
    /// the original message, in that order. (The book's own footnote
    /// notes that which of two equal-weight trees becomes the left or
    /// right branch is arbitrary, so this exercise checks the
    /// generated tree's own round trip and total code length rather
    /// than an exact bit-for-bit match with the hand-built
    /// `sample-tree`, which breaks that same tie the other way.)
    pub fn ex_2_69() -> Result<(usize, bool), Pending> {
        Err(Pending::new("2.69"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_69() {
    assert_eq!(ex_2_69::ex_2_69(), Ok((12, true)));
}
