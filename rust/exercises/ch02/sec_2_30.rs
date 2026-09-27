// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.30, one module and one ignored
//! test.

mod ex_2_30 {
    use sicp_runtime::Pending;

    /// Exercise 2.30: square-tree
    ///
    /// Returns the rendered square of the tree `(1 (2 (3 4) 5) (6 7))` by the
    /// direct recursion and by the `map`-and-recursion version, in that order.
    pub fn ex_2_30() -> Result<(String, String), Pending> {
        Err(Pending::new("2.30"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_30() {
    assert_eq!(
        ex_2_30::ex_2_30(),
        Ok((
            "(1 (4 (9 16) 25) (36 49))".to_string(),
            "(1 (4 (9 16) 25) (36 49))".to_string()
        ))
    );
}
