// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.34, one module and one ignored
//! test.

mod ex_2_34 {
    use sicp_runtime::Pending;

    /// Exercise 2.34: Horner's rule
    ///
    /// Returns `1 + 3x + 5x^3 + x^5` evaluated at `x = 2` by the
    /// `accumulate`-based `horner_eval` over `(1 3 0 5 0 1)`.
    pub fn ex_2_34() -> Result<i128, Pending> {
        Err(Pending::new("2.34"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_34() {
    assert_eq!(ex_2_34::ex_2_34(), Ok(79));
}
