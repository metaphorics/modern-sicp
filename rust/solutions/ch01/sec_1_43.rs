// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.43: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_43 {
    use std::rc::Rc;

    /// A reference-counted numerical function, cheap to share across the
    /// closures `repeated` builds.
    type Fun = Rc<dyn Fn(f64) -> f64>;

    /// The book's `repeated`, the loop spelling of exercise 1.43: one
    /// composition per application of `f`, so the build costs `n`
    /// composition steps and the tower of closures is `n` deep.
    pub fn repeated(f: impl Fn(f64) -> f64 + 'static, n: u32) -> Fun {
        let f: Fun = Rc::new(f);
        let mut g: Fun = Rc::new(|x| x);
        for _ in 0..n {
            let outer = Rc::clone(&f);
            let inner = Rc::clone(&g);
            g = Rc::new(move |x| outer(inner(x)));
        }
        g
    }

    /// Exercise 1.43: repeated application
    ///
    /// Returns `repeated(square, 2)(5)`, which is 625.
    pub fn ex_1_43() -> f64 {
        repeated(|x| x * x, 2)(5.0)
    }
}

#[test]
fn ex_1_43() {
    let value = ex_1_43::ex_1_43();
    assert!((value - 625.0).abs() < 1e-9);

    // The statement's other reading: repeated `inc` adds `n`.
    let inc = |x: f64| x + 1.0;
    assert!((ex_1_43::repeated(inc, 7)(0.0) - 7.0).abs() < 1e-9);
}
