// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.38: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_38 {
    use std::rc::Rc;

    /// The denominators of Euler's expansion: 1, 2, 1, 1, 4, 1, 1, 6,
    /// ...; every third term, starting with the second, is twice its
    /// position divided by three.
    fn euler_d(i: u32) -> f64 {
        if (i + 1).is_multiple_of(3) {
            2.0 * f64::from((i + 1) / 3)
        } else {
            1.0
        }
    }

    /// The loop spelling of the continued fraction of exercise 1.37,
    /// folded back to front.
    fn cont_frac_iter(n: &Rc<dyn Fn(u32) -> f64>, d: &Rc<dyn Fn(u32) -> f64>, k: u32) -> f64 {
        let mut acc = 0.0;
        for i in (1..=k).rev() {
            acc = n(i) / (d(i) + acc);
        }
        acc
    }

    /// Exercise 1.38: Euler's expansion for `e`
    ///
    /// Returns the approximation of `e` from Euler's continued fraction
    /// at `k = 10`.
    pub fn ex_1_38() -> f64 {
        let one: Rc<dyn Fn(u32) -> f64> = Rc::new(|_i| 1.0);
        let d: Rc<dyn Fn(u32) -> f64> = Rc::new(euler_d);
        2.0 + cont_frac_iter(&one, &d, 10)
    }
}

#[test]
fn ex_1_38() {
    let e = ex_1_38::ex_1_38();
    assert!((e - std::f64::consts::E).abs() < 1e-6);
}
