// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.47, one module and one ignored
//! test.

mod ex_2_47 {
    use sicp_runtime::Pending;

    /// Exercise 2.47: frame constructors
    ///
    /// Returns the rendered origin and edges extracted from a list-shaped
    /// frame constructor and from a cons-shaped one, both built for the
    /// unit square, in that order.
    pub fn ex_2_47() -> Result<(String, String), Pending> {
        Err(Pending::new("2.47"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_47() {
    assert_eq!(
        ex_2_47::ex_2_47(),
        Ok((
            "(0, 0) (1, 0) (0, 1)".to_string(),
            "(0, 0) (1, 0) (0, 1)".to_string()
        ))
    );
}
