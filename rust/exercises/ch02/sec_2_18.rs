// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.18, one module and one ignored
//! test.

mod ex_2_18 {
    use sicp_runtime::Pending;

    /// Exercise 2.18: reverse
    ///
    /// Returns the rendered reverse of `(1 4 9 16 25)`.
    pub fn ex_2_18() -> Result<String, Pending> {
        Err(Pending::new("2.18"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_18() {
    assert_eq!(ex_2_18::ex_2_18(), Ok("(25 16 9 4 1)".to_string()));
}
