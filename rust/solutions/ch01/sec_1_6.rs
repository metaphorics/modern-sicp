// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.6: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_06 {
    use ch01::sec_1_1::{good_enough, improve};

    /// Exercise 1.6: Eva's `new-if`, an ordinary function with ordinary
    /// eager arguments
    ///
    /// As a plain function it returns the right value; the difference from
    /// the special form is in the arguments, not the body. The type
    /// parameter stands in for Scheme's untyped values: the same
    /// `new_if` is called with plain numbers below and with the `f64`
    /// guesses of the rewritten `sqrt-iter`.
    pub fn new_if<T>(predicate: bool, then_clause: T, else_clause: T) -> T {
        if predicate { then_clause } else { else_clause }
    }

    /// Alyssa's `sqrt-iter` rewritten on `new_if`
    ///
    /// Calling this never returns: the recursive call
    /// `sqrt_iter_new_if(improve(guess, x), x)` is evaluated before
    /// `new_if` can look at its predicate, so every call starts another
    /// call, and the stack grows until it overflows. The test only
    /// references the function; running it would hang.
    #[allow(unconditional_recursion)]
    pub fn sqrt_iter_new_if(guess: f64, x: f64) -> f64 {
        new_if(
            good_enough(guess, x),
            guess,
            sqrt_iter_new_if(improve(guess, x), x),
        )
    }
}

#[test]
fn ex_1_06() {
    let _: fn(f64, f64) -> f64 = ex_1_06::sqrt_iter_new_if;
    assert_eq!(ex_1_06::new_if(2 == 3, 0, 5), 5);
    assert_eq!(ex_1_06::new_if(1 == 1, 0, 5), 0);
    let evaluated = std::cell::Cell::new(0);
    let count = |value: i64| {
        evaluated.set(evaluated.get() + 1);
        value
    };
    assert_eq!(ex_1_06::new_if(true, count(0), count(5)), 0);
    assert_eq!(evaluated.get(), 2);
}
