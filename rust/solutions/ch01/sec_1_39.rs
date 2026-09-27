// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.39: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_39 {
    /// Lambert's continued fraction for the tangent, folded back to
    /// front: the numerators are `x` once and then `x^2`, and the
    /// denominators are the odd numbers, subtracted rather than added.
    pub fn tan_cf(x: f64, k: u32) -> f64 {
        let numerator = |i: u32| if i == 1 { x } else { x * x };
        let denominator = |i: u32| 2.0 * f64::from(i) - 1.0;
        let mut acc = 0.0;
        for i in (1..=k).rev() {
            acc = numerator(i) / (denominator(i) - acc);
        }
        acc
    }

    /// Exercise 1.39: Lambert's continued fraction for the tangent
    ///
    /// Returns `tan_cf(pi / 6, 20)`, whose true value is `1 / sqrt(3)`.
    pub fn ex_1_39() -> f64 {
        tan_cf(std::f64::consts::FRAC_PI_6, 20)
    }
}

#[test]
fn ex_1_39() {
    let tangent = ex_1_39::ex_1_39();
    let expected = 1.0 / 3.0f64.sqrt();
    assert!((tangent - expected).abs() < 1e-5);

    // The fraction stays accurate away from the poles of the tangent.
    let quarter_pi = ex_1_39::tan_cf(std::f64::consts::FRAC_PI_4, 20);
    assert!((quarter_pi - 1.0).abs() < 1e-5);
}
