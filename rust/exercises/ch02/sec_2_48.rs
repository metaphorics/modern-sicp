// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.48, one module and one ignored
//! test.

mod ex_2_48 {
    use sicp_runtime::Pending;

    /// Exercise 2.48: directed line segments
    ///
    /// Returns the rendered start and end vectors selected back out of the
    /// segment from `(1, 2)` to `(3, 4)`.
    pub fn ex_2_48() -> Result<(String, String), Pending> {
        Err(Pending::new("2.48"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_48() {
    assert_eq!(
        ex_2_48::ex_2_48(),
        Ok(("(1, 2)".to_string(), "(3, 4)".to_string()))
    );
}
