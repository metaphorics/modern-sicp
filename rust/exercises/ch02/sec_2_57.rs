// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.57, one module and one ignored
//! test.

mod ex_2_57 {
    use sicp_runtime::Pending;

    /// Exercise 2.57: sums and products of arbitrary numbers of terms
    ///
    /// Returns the printed derivative of `x * y * (x + 3)` with
    /// respect to `x`, using a three-term product and a two-term sum.
    pub fn ex_2_57() -> Result<String, Pending> {
        Err(Pending::new("2.57"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_57() {
    assert_eq!(
        ex_2_57::ex_2_57(),
        Ok("(+ (* x y) (* y (+ x 3)))".to_string())
    );
}
