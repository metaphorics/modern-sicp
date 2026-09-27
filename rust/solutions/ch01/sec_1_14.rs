// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 1.14: the implementation and the
//! exercise-named test share one module so both carry the exercise's
//! name.

mod ex_1_14 {
    use std::cell::Cell;

    /// The book's `first-denomination`, restated locally so the counted
    /// `cc` below stays self-contained.
    fn first_denomination(kinds_of_coins: i64) -> i64 {
        match kinds_of_coins {
            1 => 1,
            2 => 5,
            3 => 10,
            4 => 25,
            _ => 50,
        }
    }

    /// The book's `cc`, instrumented: every call increments `calls`
    /// before it does anything else, so `calls` ends holding the size of
    /// the tree the process explores.
    fn cc(amount: i64, kinds_of_coins: i64, calls: &Cell<u64>) -> i64 {
        calls.set(calls.get() + 1);
        if amount == 0 {
            1
        } else if amount < 0 || kinds_of_coins == 0 {
            0
        } else {
            cc(amount, kinds_of_coins - 1, calls)
                + cc(
                    amount - first_denomination(kinds_of_coins),
                    kinds_of_coins,
                    calls,
                )
        }
    }

    /// Exercise 1.14: the count-change tree and its growth
    ///
    /// Returns the number of ways to change 11 cents first, and the
    /// number of calls the tree-recursive process makes while computing
    /// it second. The tree has one leaf for every partial way to reach
    /// zero or to fail, so both the space (proportional to the tree's
    /// depth, `Theta(amount)`) and the step count (proportional to its
    /// size, `Theta(amount^5)` for five kinds of coins) grow with the
    /// amount, not with the answer.
    pub fn ex_1_14() -> (i64, u64) {
        let calls = Cell::new(0);
        let ways = cc(11, 5, &calls);
        (ways, calls.get())
    }
}

#[test]
fn ex_1_14() {
    let (ways, calls) = ex_1_14::ex_1_14();
    assert_eq!(ways, 4);
    assert_eq!(calls, 55);
}
