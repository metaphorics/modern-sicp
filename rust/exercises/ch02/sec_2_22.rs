// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.22, one module and one ignored
//! test.

mod ex_2_22 {
    use sicp_runtime::Pending;

    /// Exercise 2.22: the iterative square-list bug
    ///
    /// Returns the rendered answer of Louis Reasoner's first iterative rewrite
    /// (which comes out reversed) and the rendered shape his `cons`-swapped
    /// second rewrite accumulates instead of a flat list.
    pub fn ex_2_22() -> Result<(String, String), Pending> {
        Err(Pending::new("2.22"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_22() {
    assert_eq!(
        ex_2_22::ex_2_22(),
        Ok(("(16 9 4 1)".to_string(), "((((() 1) 4) 9) 16)".to_string()))
    );
}
