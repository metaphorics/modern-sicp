// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The pending scaffold of exercise 1.6: the stub and the exercise-named
//! test share one module so both carry the exercise's name.

mod ex_1_06 {
    /// The typed pending report of an unsolved scaffold: the body returns
    /// this instead of panicking, so the failure names its origin.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pending {
        /// The exercise number the scaffold belongs to, as `1.6`.
        pub exercise: &'static str,
    }

    /// Exercise 1.6: Eva's `new-if`, an ordinary function with ordinary
    /// eager arguments
    pub fn new_if<T>(_predicate: bool, _then_clause: T, _else_clause: T) -> Result<T, Pending> {
        Err(Pending { exercise: "1.6" })
    }

    /// Alyssa's `sqrt-iter` rewritten on `new_if`; calling it diverges,
    /// because the recursive call is evaluated before `new_if` runs.
    #[allow(dead_code)]
    pub fn sqrt_iter_new_if(_guess: f64, _x: f64) -> Result<f64, Pending> {
        Err(Pending { exercise: "1.6" })
    }
}

#[test]
#[ignore = "pending solution"]
fn ex_1_06() {
    use std::cell::Cell;

    assert_eq!(ex_1_06::new_if(2 == 3, 0, 5), Ok(5));
    assert_eq!(ex_1_06::new_if(1 == 1, 0, 5), Ok(0));
    let evaluated = Cell::new(0);
    let count = |value: i64| {
        evaluated.set(evaluated.get() + 1);
        value
    };
    assert_eq!(ex_1_06::new_if(true, count(0), count(5)), Ok(0));
    assert_eq!(evaluated.get(), 2);
}
