// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.21, one module and one ignored
//! test.

mod ex_2_21 {
    use sicp_runtime::Pending;

    /// Exercise 2.21: square-list two ways
    ///
    /// Returns the rendered squares of `(1 2 3 4)` computed by the direct
    /// recursive definition and by the `map`-based one, in that order.
    pub fn ex_2_21() -> Result<(String, String), Pending> {
        Err(Pending::new("2.21"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_21() {
    assert_eq!(
        ex_2_21::ex_2_21(),
        Ok(("(1 4 9 16)".to_string(), "(1 4 9 16)".to_string()))
    );
}
