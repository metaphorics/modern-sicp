// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.56, one module and one ignored
//! test.

mod ex_2_56 {
    use sicp_runtime::Pending;

    /// Exercise 2.56: extending `deriv` with exponentiation
    ///
    /// Returns the printed derivative of `x^3` with respect to `x`.
    pub fn ex_2_56() -> Result<String, Pending> {
        Err(Pending::new("2.56"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_56() {
    assert_eq!(ex_2_56::ex_2_56(), Ok("(* 3 (** x 2))".to_string()));
}
