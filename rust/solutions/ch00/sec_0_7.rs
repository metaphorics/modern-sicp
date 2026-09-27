// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution tests of section 0.7: one test per
//! exercise with its statement in a doc comment, and shared code in
//! the matching src module.

/// The reference solution of exercise 0.4: the stub and the
/// exercise-named test share one module so both carry the exercise's
/// name.
mod ex_0_04 {
    use std::cell::Cell;
    use std::rc::Rc;

    /// Exercise 0.4: an accumulator built from two closures
    ///
    /// Returns a pair of closures that share one running total through
    /// `Rc<Cell<i128>>`: `Rc` gives both closures a handle to the same
    /// cell, and `Cell` lets a `move` closure mutate through a shared
    /// (non-`mut`) capture. A plain `i128` moved into both closures would
    /// give each its own copy instead: `Copy` types are duplicated by
    /// `move`, not shared.
    pub fn make_accumulator() -> ch00::sec_0_7::Accumulator {
        let total = Rc::new(Cell::new(0i128));

        let add = {
            let total = Rc::clone(&total);
            move |amount: i128| {
                let new_total = total.get() + amount;
                total.set(new_total);
                new_total
            }
        };

        let reset = move || {
            let old_total = total.get();
            total.set(0);
            old_total
        };

        (Box::new(add), Box::new(reset))
    }
}

#[test]
fn ex_0_04() {
    let (add, reset) = ex_0_04::make_accumulator();
    assert_eq!(add(10), 10);
    assert_eq!(add(15), 25);
    assert_eq!(reset(), 25);
    assert_eq!(add(1), 1);
}
