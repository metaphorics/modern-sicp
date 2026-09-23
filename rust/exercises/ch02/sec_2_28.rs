// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.28, one module and one ignored
//! test.

mod ex_2_28 {
    use sicp_runtime::Pending;

    /// Exercise 2.28: fringe
    ///
    /// Returns the rendered leaves of `x = ((1 2) (3 4))` and of
    /// `list(x, x)`, in that order.
    pub fn ex_2_28() -> Result<(String, String), Pending> {
        Err(Pending::new("2.28"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_28() {
    assert_eq!(
        ex_2_28::ex_2_28(),
        Ok(("(1 2 3 4)".to_string(), "(1 2 3 4 1 2 3 4)".to_string()))
    );
}
