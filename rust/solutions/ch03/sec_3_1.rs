// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.1: an accumulator keeps a
//! running sum.

/// Exercise 3.1: an accumulator keeps a running sum
///
/// Builds the accumulator as a closure that owns its running sum and
/// sees it mutably on every call, so each accumulator made here is
/// independent of every other.
mod ex_3_01 {
    pub fn make_accumulator(initial: i128) -> impl FnMut(i128) -> i128 {
        let mut sum = initial;
        move |value| {
            sum += value;
            sum
        }
    }

    /// Exercise 3.1: an accumulator keeps a running sum
    ///
    /// Returns the running sums after the first and the second call of an
    /// accumulator started at 5 and called with 10 each time.
    #[must_use]
    pub fn ex_3_01() -> (i128, i128) {
        let mut a = make_accumulator(5);
        (a(10), a(10))
    }
}

#[test]
fn ex_3_01() {
    assert_eq!(ex_3_01::ex_3_01(), (15, 25));

    // Two accumulators started at the same value stay independent: a
    // call through one name must never move the other's sum.
    let mut first = ex_3_01::make_accumulator(0);
    let mut second = ex_3_01::make_accumulator(0);
    assert_eq!(first(3), 3);
    assert_eq!(second(100), 100);
    assert_eq!(first(4), 7);
    assert_eq!(second(1), 101);
}
