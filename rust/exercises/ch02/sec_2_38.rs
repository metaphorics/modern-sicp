// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.38, one module and one ignored
//! test.

mod ex_2_38 {
    use sicp_runtime::Pending;

    /// Exercise 2.38: fold-left versus fold-right
    ///
    /// Returns the `fold_right` and `fold_left` divisions of `(1 2 3)` into 1,
    /// and the nested-list shapes each folding direction builds over
    /// `(1 2 3)`, in that order.
    pub fn ex_2_38() -> Result<(f64, f64, String, String), Pending> {
        Err(Pending::new("2.38"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_38() {
    assert_eq!(
        ex_2_38::ex_2_38(),
        Ok((
            1.5,
            1.0 / 6.0,
            "(1 (2 (3 ())))".to_string(),
            "(((() 1) 2) 3)".to_string()
        ))
    );
}
