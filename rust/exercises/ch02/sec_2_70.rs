// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.70, one module and one ignored
//! test.

mod ex_2_70 {
    use sicp_runtime::Pending;

    /// Exercise 2.70: encoding a rock song's lyrics
    ///
    /// Generates a Huffman tree for the eight-symbol "So-o rock"
    /// alphabet (`NA` 16, `YIP` 9, `SHA` 3, `A` 2, `GET` 2, `JOB` 2,
    /// `BOOM` 1, `WAH` 1) and encodes the 36-word lyric line. Returns
    /// the number of bits the Huffman encoding uses, and the number
    /// of bits a fixed-length code would need for the same 36
    /// symbols, in that order.
    pub fn ex_2_70() -> Result<(usize, usize), Pending> {
        Err(Pending::new("2.70"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_70() {
    assert_eq!(ex_2_70::ex_2_70(), Ok((84, 108)));
}
