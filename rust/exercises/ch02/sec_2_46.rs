// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.46, one module and one ignored
//! test.

mod ex_2_46 {
    use sicp_runtime::Pending;

    /// Exercise 2.46: vector abstraction
    ///
    /// Returns the rendered `add_vect`, `sub_vect`, and `scale_vect` results
    /// for `(1, 2)` and `(3, 4)` with scalar 3, in that order.
    pub fn ex_2_46() -> Result<(String, String, String), Pending> {
        Err(Pending::new("2.46"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_46() {
    assert_eq!(
        ex_2_46::ex_2_46(),
        Ok((
            "(4, 6)".to_string(),
            "(-2, -2)".to_string(),
            "(6, 15)".to_string()
        ))
    );
}
