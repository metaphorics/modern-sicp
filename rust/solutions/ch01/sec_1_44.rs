// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.44: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_44 {
    use std::rc::Rc;

    /// The book's `smooth`: the function whose value at `x` is the
    /// average of `f(x - dx)`, `f(x)`, and `f(x + dx)`. The closure owns
    /// `f`, which is what `move` says.
    pub fn smooth(f: impl Fn(f64) -> f64 + 'static, dx: f64) -> impl Fn(f64) -> f64 {
        move |x| (f(x - dx) + f(x) + f(x + dx)) / 3.0
    }

    /// The smoother lifted to own its argument, so it can be applied
    /// `n` times in a row; this is the book's `repeated(smooth, n)`
    /// reading, spelled with the sharing made explicit.
    fn smooth_owned(f: Rc<dyn Fn(f64) -> f64>, dx: f64) -> Rc<dyn Fn(f64) -> f64> {
        Rc::new(move |x| (f(x - dx) + f(x) + f(x + dx)) / 3.0)
    }

    /// The book's n-fold smoothed function: `smooth` applied `n` times.
    pub fn n_fold_smooth(
        f: impl Fn(f64) -> f64 + 'static,
        n: u32,
        dx: f64,
    ) -> Rc<dyn Fn(f64) -> f64> {
        let mut g: Rc<dyn Fn(f64) -> f64> = Rc::new(f);
        for _ in 0..n {
            g = smooth_owned(g, dx);
        }
        g
    }

    /// Exercise 1.44: smoothing and n-fold smoothing
    ///
    /// Returns the one-fold smoothing of `|x| x` at `x = 1` first, which
    /// is still 1, and the two-fold smoothing of `x^2` at `x = 1` second,
    /// which is `1 + 4 dx^2 / 3` for the smoothing's `dx = 1`.
    pub fn ex_1_44() -> (f64, f64) {
        let linear = |x: f64| x;
        let quadratic = |x: f64| x * x;
        (
            smooth(linear, 1.0)(1.0),
            n_fold_smooth(quadratic, 2, 1.0)(1.0),
        )
    }
}

#[test]
fn ex_1_44() {
    let (smoothed_linear, twofold_quadratic) = ex_1_44::ex_1_44();
    assert!((smoothed_linear - 1.0).abs() < 1e-9);
    assert!((twofold_quadratic - (1.0 + 4.0 / 3.0)).abs() < 1e-12);
}
