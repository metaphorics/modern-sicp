// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 2.50, one module and one ignored
//! test.

mod ex_2_50 {
    use sicp_runtime::Pending;

    /// Exercise 2.50: flip-horiz and the rotations
    ///
    /// Returns the rendered segment set of the probe segment `(0.25, 0.5)`
    /// to `(0.75, 0.875)` under `flip_horiz`, `rotate_180`, and
    /// `rotate_270`, each built directly with `transform_painter`.
    pub fn ex_2_50() -> Result<(String, String, String), Pending> {
        Err(Pending::new("2.50"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_50() {
    assert_eq!(
        ex_2_50::ex_2_50(),
        Ok((
            "(0.75, 0.5) -> (0.25, 0.875)".to_string(),
            "(0.75, 0.5) -> (0.25, 0.125)".to_string(),
            "(0.5, 0.75) -> (0.875, 0.25)".to_string()
        ))
    );
}
