// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffolds of exercise 2.51 and the edition's addition
//! 2.51a, one module and one ignored test per exercise.

mod ex_2_51 {
    use sicp_runtime::Pending;

    /// Exercise 2.51: below two ways
    ///
    /// Returns the rendered segment sets of a probe pair painted by the
    /// direct `below` and by the rotation-composed `below`, which must
    /// agree.
    pub fn ex_2_51() -> Result<(String, String), Pending> {
        Err(Pending::new("2.51"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_51() {
    assert!(matches!(
        ex_2_51::ex_2_51(),
        Ok((direct, rotated)) if direct == rotated && !direct.is_empty()
    ));
}

mod ex_2_51a {
    use sicp_runtime::Pending;

    /// Exercise 2.51a (this edition): the below-via-rotate composition
    /// check
    ///
    /// Returns the two SVG documents for `below(wave, wave)`, the
    /// direct construction and the rotation-composed one, which must be
    /// byte-identical.
    pub fn ex_2_51a() -> Result<(String, String), Pending> {
        Err(Pending::new("2.51a"))
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_2_51a() {
    assert!(matches!(
        ex_2_51a::ex_2_51a(),
        Ok((direct, rotated)) if direct == rotated && !direct.is_empty()
    ));
}
