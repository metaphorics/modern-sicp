// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.49, one module and one ignored
//! test.

mod ex_2_49 {
    use sicp_runtime::Pending;

    /// Exercise 2.49: primitive painters
    ///
    /// Returns the number of segments each primitive painter draws in a
    /// frame: the outline, the X, the diamond, and the wave, in that order.
    pub fn ex_2_49() -> Result<[usize; 4], Pending> {
        Err(Pending::new("2.49"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_49() {
    assert_eq!(ex_2_49::ex_2_49(), Ok([4, 2, 4, 18]));
}
