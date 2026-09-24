// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.8: operand evaluation order is
//! defined.

/// Exercise 3.8: operand evaluation order is defined
///
/// Builds the probe the book's `f` becomes: it answers its argument on
/// the first call and 0 on every later call, so the sum of two calls
/// records which operand went first.
mod ex_3_08 {
    pub fn make_order_probe() -> impl FnMut(i128) -> i128 {
        let mut calls = 0_u32;
        move |x| {
            calls += 1;
            if calls == 1 { x } else { 0 }
        }
    }

    /// Exercise 3.8: operand evaluation order is defined
    ///
    /// Returns the value of `f(0) + f(1)` and of a fresh probe's
    /// `f(1) + f(0)`. Rust fixes operand order left to right, so the first
    /// sum is 0 and the second is 1: the order is defined, and this
    /// observes it.
    #[must_use]
    pub fn ex_3_08() -> (i128, i128) {
        let mut f = make_order_probe();
        let left_to_right = f(0) + f(1);
        let mut g = make_order_probe();
        let reversed = g(1) + g(0);
        (left_to_right, reversed)
    }
}

#[test]
fn ex_3_08() {
    assert_eq!(ex_3_08::ex_3_08(), (0, 1));
}
