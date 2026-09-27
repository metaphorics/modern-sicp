// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.58, one module and one ignored
//! test.

mod ex_2_58 {
    use sicp_runtime::Pending;

    /// Exercise 2.58: infix notation
    ///
    /// Part a: returns the printed derivative of the fully
    /// parenthesized infix expression `(x + (3 * (x + (y + 2))))`
    /// with respect to `x`. Part b: returns the printed derivative of
    /// the same expression in ordinary infix notation,
    /// `x + 3 * (x + y + 2)`, dropping unnecessary parentheses and
    /// giving `*` higher precedence than `+`.
    pub fn ex_2_58() -> Result<(String, String), Pending> {
        Err(Pending::new("2.58"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_58() {
    assert_eq!(ex_2_58::ex_2_58(), Ok(("4".to_string(), "4".to_string())));
}
