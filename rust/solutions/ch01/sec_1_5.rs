// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.5: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_05 {
    /// The book's `p`: a call to `p` diverges, forever calling itself.
    #[allow(unconditional_recursion)]
    pub fn p() -> ! {
        p()
    }

    /// Exercise 1.5: the book's `test`, whose second argument is evaluated
    /// eagerly before the call
    ///
    /// Rust evaluates arguments applicative-order style, so `test(0, p())`
    /// never returns: evaluating `p()` diverges before `test` runs.
    pub fn test(x: i64, y: i64) -> i64 {
        if x == 0 { 0 } else { y }
    }

    /// The normal-order stand-in: `y` is a thunk, evaluated only when `x`
    /// is not zero, so `test_lazy(0, || ...)` never runs its argument.
    pub fn test_lazy(x: i64, y: impl FnOnce() -> i64) -> i64 {
        if x == 0 { 0 } else { y() }
    }
}

#[test]
fn ex_1_05() {
    let _: fn() -> ! = ex_1_05::p;
    assert_eq!(ex_1_05::test(0, 5), 0);
    assert_eq!(ex_1_05::test(1, 5), 5);
    assert_eq!(ex_1_05::test_lazy(0, || 5), 0);
    let called = std::cell::Cell::new(false);
    let thunk = || {
        called.set(true);
        5
    };
    assert_eq!(ex_1_05::test_lazy(0, thunk), 0);
    assert!(!called.get());
}
