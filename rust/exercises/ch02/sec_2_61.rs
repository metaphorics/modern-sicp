// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.61, one module and one ignored
//! test.

mod ex_2_61 {
    use sicp_runtime::Pending;

    /// Exercise 2.61: `adjoin-set` for the ordered-list representation
    ///
    /// Returns `adjoin_set_ordered(5, &[1, 3, 6, 10])` (inserting a new
    /// element) and `adjoin_set_ordered(3, &[1, 3, 6, 10])` (adjoining
    /// an already-present element), in that order.
    pub fn ex_2_61() -> Result<(Vec<i128>, Vec<i128>), Pending> {
        Err(Pending::new("2.61"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_61() {
    assert_eq!(
        ex_2_61::ex_2_61(),
        Ok((vec![1, 3, 5, 6, 10], vec![1, 3, 6, 10]))
    );
}
