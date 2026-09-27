// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.62, one module and one ignored
//! test.

mod ex_2_62 {
    use sicp_runtime::Pending;

    /// Exercise 2.62: a Θ(n) `union-set` for ordered-list sets
    ///
    /// Returns `union_set_ordered(&[1, 3, 6, 10], &[2, 3, 7])`.
    pub fn ex_2_62() -> Result<Vec<i128>, Pending> {
        Err(Pending::new("2.62"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_62() {
    assert_eq!(ex_2_62::ex_2_62(), Ok(vec![1, 2, 3, 6, 7, 10]));
}
