// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.6: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_2_06 {
    use ch02::sec_2_1::{Church, Step};

    /// Exercise 2.6: `one`, defined directly (not as `add-1` applied to
    /// `zero`): applies `f` exactly once.
    fn one() -> Church {
        Church::from_fn(|f: Step| Step::from_fn(move |x: &mut i128| f.apply(x)))
    }

    /// Exercise 2.6: `two`, defined directly: applies `f` exactly twice.
    fn two() -> Church {
        Church::from_fn(|f: Step| {
            Step::from_fn(move |x: &mut i128| {
                f.apply(x);
                f.apply(x);
            })
        })
    }

    /// Exercise 2.6: `+`, defined directly (not by repeated `add-1`):
    /// applies `f` via `a`'s composition, then via `b`'s, in sequence.
    fn church_add(a: &Church, b: &Church) -> Church {
        let a = a.clone();
        let b = b.clone();
        Church::from_fn(move |f: Step| {
            let step_a = a.apply(f.clone());
            let step_b = b.apply(f.clone());
            Step::from_fn(move |x: &mut i128| {
                step_a.apply(x);
                step_b.apply(x);
            })
        })
    }

    /// Exercise 2.6: Church numerals `one` and `two`, and `+`, all
    /// defined directly
    ///
    /// Returns `one`, `two`, and `one + two`, each converted to an
    /// ordinary integer.
    pub fn ex_2_06() -> (i128, i128, i128) {
        let one = one();
        let two = two();
        let sum = church_add(&one, &two);
        (one.to_i128(), two.to_i128(), sum.to_i128())
    }
}

#[test]
fn ex_2_06() {
    assert_eq!(ex_2_06::ex_2_06(), (1, 2, 3));
}
