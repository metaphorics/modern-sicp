// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.68, one module and one ignored
//! test.

mod ex_2_68 {
    use sicp_runtime::Pending;

    /// Exercise 2.68: `encode-symbol`
    ///
    /// Returns the printed bits of encoding exercise 2.67's decoded
    /// message back against `sample-tree`, which must equal the
    /// original `sample-message`, and whether encoding a symbol not in
    /// the tree is an error, in that order.
    pub fn ex_2_68() -> Result<(String, bool), Pending> {
        Err(Pending::new("2.68"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_68() {
    assert_eq!(ex_2_68::ex_2_68(), Ok(("011001010111".to_string(), true)));
}
