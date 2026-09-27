// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.29: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

use ch01::sec_1_3::{cube, integral};

mod ex_1_29 {
    use ch01::sec_1_3::cube;

    /// Simpson's Rule over `n` (even) intervals: interior terms alternate
    /// between the weights 4 and 2, and the two ends carry weight 1.
    #[allow(clippy::many_single_char_names)]
    fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64, n: u32) -> f64 {
        let h = (b - a) / f64::from(n);
        let y = |k: u32| f(a + f64::from(k) * h);
        let weight = |k: u32| {
            if k == 0 || k == n {
                1.0
            } else if k % 2 == 1 {
                4.0
            } else {
                2.0
            }
        };
        (0..=n).map(|k| weight(k) * y(k)).sum::<f64>() * h / 3.0
    }

    /// Exercise 1.29: Simpson's Rule
    ///
    /// Returns the Simpson approximation of the integral of `cube` over
    /// 0 to 1 at `n = 100` first and `n = 1000` second.
    pub fn ex_1_29() -> (f64, f64) {
        (
            simpson(&cube, 0.0, 1.0, 100),
            simpson(&cube, 0.0, 1.0, 1000),
        )
    }
}

#[test]
fn ex_1_29() {
    let (at_100, at_1000) = ex_1_29::ex_1_29();
    assert!((at_100 - 0.25).abs() < 1e-9);
    assert!((at_1000 - 0.25).abs() < 1e-11);

    // Simpson's Rule is exact on cubics up to rounding, so both of its
    // errors stay far below the midpoint rule's error at the same
    // effort.
    let midpoint_error = (integral(&cube, 0.0, 1.0, 0.01) - 0.25).abs();
    assert!((at_100 - 0.25).abs() < midpoint_error);
}
