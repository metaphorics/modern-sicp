// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.54, one module and one ignored
//! test.

mod ex_2_54 {
    use sicp_runtime::Pending;

    /// Exercise 2.54: `equal?`
    ///
    /// Returns whether `sub([this, is, a, list])` is equal to itself
    /// by the exercise's own recursive `equal?`, and whether it is
    /// equal to `sub([this, sub([is, a]), list])`, in that order.
    pub fn ex_2_54() -> Result<(bool, bool), Pending> {
        Err(Pending::new("2.54"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_54() {
    assert_eq!(ex_2_54::ex_2_54(), Ok((true, false)));
}
