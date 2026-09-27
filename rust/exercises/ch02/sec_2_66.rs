// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.66, one module and one ignored
//! test.

mod ex_2_66 {
    use sicp_runtime::Pending;

    /// Exercise 2.66: `lookup` for a tree of records
    ///
    /// Returns `lookup(5, &tree)` and `lookup(9, &tree)` on a small
    /// tree of records keyed `2`, `5`, and `8`, in that order.
    pub fn ex_2_66() -> Result<(Option<String>, Option<String>), Pending> {
        Err(Pending::new("2.66"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_66() {
    assert_eq!(ex_2_66::ex_2_66(), Ok((Some("bob".to_string()), None)));
}
