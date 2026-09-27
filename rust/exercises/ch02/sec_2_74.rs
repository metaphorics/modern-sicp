// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.74, one module and one ignored
//! test.

mod ex_2_74 {
    use sicp_runtime::Pending;

    /// Exercise 2.74: Insatiable Enterprises' division files
    ///
    /// Two divisions keep personnel files in different formats, and
    /// headquarters reads them through the operation table. Returns the
    /// salaries found for `Bitdiddle` in the north division's file, for
    /// `Hacker` in the south division's file, and for `Hacker` when
    /// headquarters searches both files for the record.
    pub fn ex_2_74() -> Result<(i128, i128, i128), Pending> {
        Err(Pending::new("2.74"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_74() {
    assert_eq!(ex_2_74::ex_2_74(), Ok((60000, 75000, 75000)));
}
