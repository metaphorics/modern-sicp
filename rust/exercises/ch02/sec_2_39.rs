// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.39, one module and one ignored
//! test.

mod ex_2_39 {
    use sicp_runtime::Pending;

    /// Exercise 2.39: reverse via folds
    ///
    /// Returns the rendered reverse of `(1 2 3 4)` built with `fold_right` and
    /// with `fold_left`, in that order.
    pub fn ex_2_39() -> Result<(String, String), Pending> {
        Err(Pending::new("2.39"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_39() {
    assert_eq!(
        ex_2_39::ex_2_39(),
        Ok(("(4 3 2 1)".to_string(), "(4 3 2 1)".to_string()))
    );
}
