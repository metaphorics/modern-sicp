// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.5: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_05 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.5`.
        pub exercise: &'static str,
    }

    /// The book's `p`: a call to `p` diverges, forever calling itself.
    #[allow(unconditional_recursion)]
    pub fn p() -> ! {
        p()
    }

    /// Exercise 1.5: the book's `test`, whose second argument is evaluated
    /// eagerly before the call
    pub fn test(_x: i64, _y: i64) -> Result<i64, Pending> {
        Err(Pending { exercise: "1.5" })
    }

    /// The normal-order stand-in: `y` is a thunk, evaluated only when `x`
    /// is not zero.
    pub fn test_lazy(_x: i64, _y: impl FnOnce() -> i64) -> Result<i64, Pending> {
        Err(Pending { exercise: "1.5" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_05() {
    use std::cell::Cell;

    let _: fn() -> ! = ex_1_05::p;
    assert_eq!(ex_1_05::test(0, 5), Ok(0));
    assert_eq!(ex_1_05::test(1, 5), Ok(5));
    assert_eq!(ex_1_05::test_lazy(0, || 5), Ok(0));
    let called = Cell::new(false);
    let thunk = || {
        called.set(true);
        5
    };
    assert_eq!(ex_1_05::test_lazy(0, thunk), Ok(0));
    assert!(!called.get());
}
