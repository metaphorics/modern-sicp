// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.64, one module and one ignored
//! test.

mod ex_2_64 {
    use sicp_runtime::Pending;

    /// Exercise 2.64: `list->tree`
    ///
    /// Part a: returns the printed tree `list_to_tree` builds for
    /// `[1, 3, 5, 7, 9, 11]`. Part b: returns the number of tree
    /// nodes `partial_tree` builds for an 11-element list and for a
    /// 101-element list, in that order.
    pub fn ex_2_64() -> Result<(String, u64, u64), Pending> {
        Err(Pending::new("2.64"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_64() {
    assert_eq!(
        ex_2_64::ex_2_64(),
        Ok((
            "(5 (1 () (3 () ())) (9 (7 () ()) (11 () ())))".to_string(),
            11,
            101
        ))
    );
}
