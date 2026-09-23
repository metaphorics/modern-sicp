// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.33, one module and one ignored
//! test.

mod ex_2_33 {
    use sicp_runtime::Pending;

    /// Exercise 2.33: list operations as accumulations
    ///
    /// Returns the rendered `map` of squaring over `(1 2 3 4)`, the rendered
    /// `append` of `(1 2 3)` and `(4 5)`, and the `length` of `(1 2 3)`, all
    /// built on `accumulate`, in that order.
    pub fn ex_2_33() -> Result<(String, String, usize), Pending> {
        Err(Pending::new("2.33"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_33() {
    assert_eq!(
        ex_2_33::ex_2_33(),
        Ok(("(1 4 9 16)".to_string(), "(1 2 3 4 5)".to_string(), 3))
    );
}
