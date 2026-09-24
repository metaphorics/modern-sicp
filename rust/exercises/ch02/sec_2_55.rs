// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.55, one module and one ignored
//! test.

mod ex_2_55 {
    use sicp_runtime::Pending;

    /// Exercise 2.55 (replacement): quote in edition AST terms
    ///
    /// Returns the printed value of `''abracadabra` (the edition's
    /// tiny quoted-AST type), and the printed `car` of that value, in
    /// that order.
    pub fn ex_2_55() -> Result<(String, String), Pending> {
        Err(Pending::new("2.55"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_55() {
    assert_eq!(
        ex_2_55::ex_2_55(),
        Ok(("(quote abracadabra)".to_string(), "quote".to_string()))
    );
}
