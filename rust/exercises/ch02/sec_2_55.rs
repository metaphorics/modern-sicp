// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.55, one module and one ignored
//! test.

mod ex_2_55 {
    use sicp_runtime::Pending;

    /// Exercise 2.55 (replacement): quote in edition AST terms
    ///
    /// Returns the quotation datum a doubly quoted name denotes in the
    /// edition's `Quoted` data, and the head of that datum, each in
    /// the explicit constructor rendering.
    pub fn ex_2_55() -> Result<(String, String), Pending> {
        Err(Pending::new("2.55"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_55() {
    assert_eq!(
        ex_2_55::ex_2_55(),
        Ok((
            "Pair(Sym(\"quote\"), Pair(Sym(\"abracadabra\"), Nil))".to_string(),
            "Sym(\"quote\")".to_string(),
        ))
    );
}
