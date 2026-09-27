// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.67, one module and one ignored
//! test.

mod ex_2_67 {
    use sicp_runtime::Pending;

    /// Exercise 2.67: decoding the sample message
    ///
    /// Returns the symbols `decode` produces for the book's
    /// `sample-message` against `sample-tree`.
    pub fn ex_2_67() -> Result<Vec<String>, Pending> {
        Err(Pending::new("2.67"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_67() {
    assert_eq!(
        ex_2_67::ex_2_67(),
        Ok(vec![
            "A".to_string(),
            "D".to_string(),
            "A".to_string(),
            "B".to_string(),
            "B".to_string(),
            "C".to_string(),
        ])
    );
}
