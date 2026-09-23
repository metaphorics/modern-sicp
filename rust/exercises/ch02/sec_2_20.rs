// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.20, one module and one ignored
//! test.

mod ex_2_20 {
    use sicp_runtime::Pending;

    /// Exercise 2.20: same-parity with a rest parameter
    ///
    /// Returns the rendered argument lists that share the first argument's parity
    /// for the calls `same_parity(1, [2, 3, 4, 5, 6, 7])` and
    /// `same_parity(2, [3, 4, 5, 6, 7])`, in that order.
    pub fn ex_2_20() -> Result<(String, String), Pending> {
        Err(Pending::new("2.20"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_20() {
    assert_eq!(
        ex_2_20::ex_2_20(),
        Ok(("(1 3 5 7)".to_string(), "(2 4 6)".to_string()))
    );
}
