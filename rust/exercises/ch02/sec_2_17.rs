// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.17, one module and one ignored
//! test.

mod ex_2_17 {
    use sicp_runtime::Pending;

    /// Exercise 2.17: last-pair
    ///
    /// Returns the rendered list holding only the last element of `(23 72 149 34)`.
    pub fn ex_2_17() -> Result<String, Pending> {
        Err(Pending::new("2.17"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_17() {
    assert_eq!(ex_2_17::ex_2_17(), Ok("(34)".to_string()));
}
