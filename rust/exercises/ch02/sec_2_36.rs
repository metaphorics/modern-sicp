// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.36, one module and one ignored
//! test.

mod ex_2_36 {
    use sicp_runtime::Pending;

    /// Exercise 2.36: accumulate-n
    ///
    /// Returns the column sums of `((1 2 3) (4 5 6) (7 8 9) (10 11 12))`,
    /// which are `(22 26 30)`.
    pub fn ex_2_36() -> Result<Vec<i64>, Pending> {
        Err(Pending::new("2.36"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_36() {
    assert_eq!(ex_2_36::ex_2_36(), Ok(vec![22, 26, 30]));
}
