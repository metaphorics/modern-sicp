// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.23, one module and one ignored
//! test.

mod ex_2_23 {
    use sicp_runtime::Pending;

    /// Exercise 2.23: for-each
    ///
    /// Returns the items `for_each` applied its action to, in order, for the
    /// list `(57 321 88)`, together with its `()` return value flag.
    pub fn ex_2_23() -> Result<(Vec<i128>, bool), Pending> {
        Err(Pending::new("2.23"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_23() {
    assert_eq!(ex_2_23::ex_2_23(), Ok((vec![57, 321, 88], true)));
}
