// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.52, one module and one ignored
//! test.

mod ex_2_52 {
    use sicp_runtime::Pending;

    /// Exercise 2.52: square-limit variations
    ///
    /// Returns the segment counts of the smiled wave and of the single-copy
    /// corner split, plus a verdict that the rearranged square limit paints
    /// a different segment set from the book's arrangement.
    pub fn ex_2_52() -> Result<(usize, usize, bool), Pending> {
        Err(Pending::new("2.52"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_52() {
    assert_eq!(ex_2_52::ex_2_52(), Ok((20, 72, true)));
}
