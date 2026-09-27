// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.24, one module and one ignored
//! test.

mod ex_2_24 {
    use sicp_runtime::Pending;

    /// Exercise 2.24: the structure of a nested list
    ///
    /// Returns the printed form of `list(1, list(2, list(3, 4)))`, the
    /// replacement exercise's predicted output.
    pub fn ex_2_24() -> Result<String, Pending> {
        Err(Pending::new("2.24"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_24() {
    assert_eq!(ex_2_24::ex_2_24(), Ok("(1 (2 (3 4)))".to_string()));
}
