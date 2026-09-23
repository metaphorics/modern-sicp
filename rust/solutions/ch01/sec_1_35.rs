// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.35: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_35 {
    use ch01::sec_1_3::fixed_point;

    /// Exercise 1.35: the golden ratio as a fixed point
    ///
    /// Returns the fixed point of `x` mapped to `1 + 1 / x`.
    pub fn ex_1_35() -> f64 {
        fixed_point(|x| 1.0 + 1.0 / x, 1.0)
    }
}

#[test]
fn ex_1_35() {
    // The golden ratio: x = 1 + 1/x rearranges to x^2 = x + 1.
    let phi = ex_1_35::ex_1_35();
    assert!((phi - 1.618_033_988_749_895).abs() < 1e-5);
    assert!((phi * phi - (phi + 1.0)).abs() < 1e-4);
}
