// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.31, one module and one ignored
//! test.

mod ex_2_31 {
    use sicp_runtime::Pending;

    /// Exercise 2.31: tree-map
    ///
    /// Returns the rendered square of `(1 (2 (3 4) 5) (6 7))` produced by
    /// `square_tree` defined as a `tree_map` call.
    pub fn ex_2_31() -> Result<String, Pending> {
        Err(Pending::new("2.31"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_31() {
    assert_eq!(
        ex_2_31::ex_2_31(),
        Ok("(1 (4 (9 16) 25) (36 49))".to_string())
    );
}
