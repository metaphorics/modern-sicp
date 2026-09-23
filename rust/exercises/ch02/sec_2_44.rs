// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.44, one module and one ignored
//! test.

mod ex_2_44 {
    use sicp_runtime::Pending;

    /// Exercise 2.44: up-split
    ///
    /// Returns the segment counts that `right_split` and the newly defined
    /// `up_split` each paint for the `wave` painter at depth 1, and the
    /// `wave` painter's own count, in that order.
    pub fn ex_2_44() -> Result<(usize, usize, usize), Pending> {
        Err(Pending::new("2.44"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_44() {
    assert_eq!(ex_2_44::ex_2_44(), Ok((54, 54, 18)));
}
