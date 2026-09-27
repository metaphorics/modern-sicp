// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.41: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_41 {
    /// The book's `double`: the procedure that applies `f` twice. The
    /// closure owns `f`, which is what `move` says.
    fn double(f: impl Fn(f64) -> f64) -> impl Fn(f64) -> f64 {
        move |x| f(f(x))
    }

    /// The book's `inc`.
    fn inc(x: f64) -> f64 {
        x + 1.0
    }

    /// `double` around `inc` once, the base case the statement names.
    pub fn double_inc() -> impl Fn(f64) -> f64 {
        double(inc)
    }

    /// Exercise 1.41: the `double` procedure
    ///
    /// Returns the value the book's `(((double (double double)) inc) 5)`
    /// computes, rebuilt with as many nestings of `double` around `inc`
    /// as the book's self-application builds: each of the three
    /// `double`s in the book's expression doubles the count of the one
    /// inside it, for sixteen applications of `inc` in all.
    pub fn ex_1_41() -> f64 {
        double(double(double(double(inc))))(5.0)
    }
}

#[test]
fn ex_1_41() {
    let value = ex_1_41::ex_1_41();
    assert!((value - 21.0).abs() < 1e-9);

    // The book's base case: `double(inc)` adds 2.
    assert!((ex_1_41::double_inc()(5.0) - 7.0).abs() < 1e-9);
}
