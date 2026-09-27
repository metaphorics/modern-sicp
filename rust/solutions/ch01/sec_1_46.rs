// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.46: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_46 {
    use std::rc::Rc;

    use ch01::sec_1_1::average;
    use ch01::sec_1_3::TOLERANCE;

    /// The book's `iterative_improve`: the returned procedure runs the
    /// improve step in a loop until the guess is good enough. The loop
    /// lives inside the closure because a closure cannot call itself by
    /// name.
    pub fn iterative_improve(
        good_enough: impl Fn(f64) -> bool,
        improve: impl Fn(f64) -> f64,
    ) -> impl Fn(f64) -> f64 {
        move |mut guess| {
            while !good_enough(guess) {
                guess = improve(guess);
            }
            guess
        }
    }

    /// The `sqrt` of 1.1.7, rebuilt on `iterative_improve`.
    pub fn sqrt(x: f64) -> f64 {
        iterative_improve(|g| (g * g - x).abs() < 0.001, move |g| average(g, x / g))(1.0)
    }

    /// The `fixed_point` of 1.3.3, rebuilt on `iterative_improve`. The
    /// function `f` appears in both arguments, so it is shared through
    /// one reference-counted cell.
    pub fn fixed_point(f: impl Fn(f64) -> f64 + 'static, first_guess: f64) -> f64 {
        let shared: Rc<dyn Fn(f64) -> f64> = Rc::new(f);
        let for_check = Rc::clone(&shared);
        iterative_improve(
            move |g| (g - for_check(g)).abs() < TOLERANCE,
            move |g| shared(g),
        )(first_guess)
    }

    /// Exercise 1.46: iterative improvement
    ///
    /// Returns the square root of 9 computed with `iterative_improve`
    /// first, and the fixed point of the cosine computed with
    /// `iterative_improve` second.
    pub fn ex_1_46() -> (f64, f64) {
        (sqrt(9.0), fixed_point(f64::cos, 1.0))
    }
}

#[test]
fn ex_1_46() {
    let (root, cos_fixed) = ex_1_46::ex_1_46();
    assert!((root - 3.0).abs() < 1e-3);
    assert!((cos_fixed - 0.739_082_298_522_402_4).abs() < 1e-5);
}
