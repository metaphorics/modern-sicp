// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.45, one module and one ignored
//! test.

mod ex_2_45 {
    use sicp_runtime::Pending;

    /// Exercise 2.45: split, a general splitting combinator
    ///
    /// Returns rendered segment sets of `right_split` and `up_split` for the
    /// `wave` painter at depth 1, each built by the general `split`
    /// combinator instead of by hand.
    pub fn ex_2_45() -> Result<(String, String), Pending> {
        Err(Pending::new("2.45"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_45() {
    use ch02::sec_2_2::{CaptureSink, Frame, right_split, up_split, wave};

    let painter = wave();
    let mut right = CaptureSink::new();
    right_split(&painter, 1)(&Frame::unit_square(), &mut right);
    let mut up = CaptureSink::new();
    up_split(&painter, 1)(&Frame::unit_square(), &mut up);
    assert_eq!(ex_2_45::ex_2_45(), Ok((right.to_text(), up.to_text())));
}
