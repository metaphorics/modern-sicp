// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.26, one module and one ignored
//! test.

mod ex_2_26 {
    use sicp_runtime::Pending;

    /// Exercise 2.26: sequence concatenation, pair construction, and
    /// nested sequence construction over two sequences.
    ///
    /// Returns the printed results of concatenating `x` and `y`, pairing
    /// `x` with `y`, and building a sequence whose elements are `x` and
    /// `y`, for `x = (1 2 3)` and `y = (4 5 6)`.
    pub fn ex_2_26() -> Result<(String, String, String), Pending> {
        Err(Pending::new("2.26"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_26() {
    assert_eq!(
        ex_2_26::ex_2_26(),
        Ok((
            "(1 2 3 4 5 6)".to_string(),
            "((1 2 3) 4 5 6)".to_string(),
            "((1 2 3) (4 5 6))".to_string()
        ))
    );
}
