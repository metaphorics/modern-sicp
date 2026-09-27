// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.58: `expand` performs long
//! division. Each step emits the integer quotient of `num x radix` by
//! `den` -- the next digit of `num/den` in the given radix -- and
//! continues with the remainder scaled back into the dividend's place
//! by the same radix. The stream is therefore the radix representation
//! of the fraction: `(expand 1 7 10)` emits the decimal digits of 1/7
//! (period 142857, so the walk past the period repeats), and
//! `(expand 3 8 10)` emits 3 for the integer part, then the digits of
//! 0.75, then zeros forever -- 3/8 terminates in base 10.

use ch03::sec_3_5::{Stream, cons_stream};

/// The book's `expand`: the digit stream of `num/den` in `radix`, one
/// long-division step per element.
fn expand(num: i128, den: i128, radix: i128) -> Stream<i128> {
    let product = num * radix;
    cons_stream(product / den, move || expand(product % den, den, radix))
}

mod ex_3_58 {
    use super::expand;

    /// Exercise 3.58: expand computes long division digits
    ///
    /// Answers the first 8 digits of `(expand 1 7 10)` and the first 6
    /// of `(expand 3 8 10)`.
    #[must_use]
    pub fn ex_3_58() -> (Vec<i128>, Vec<i128>) {
        let sevenths = expand(1, 7, 10).iter().take(8).collect();
        let eighths = expand(3, 8, 10).iter().take(6).collect();
        (sevenths, eighths)
    }
}

#[test]
fn ex_3_58() {
    let (sevenths, eighths) = ex_3_58::ex_3_58();
    // 1/7 = 0.142857142857...: the division steps walk the repeating
    // period, so the eighth digit starts the period over.
    assert_eq!(sevenths, [1, 4, 2, 8, 5, 7, 1, 4]);
    // 3/8 = 3.75: the integer part 3 first, then 7 and 5, then the
    // remainder hits 0 and every later digit is 0.
    assert_eq!(eighths, [3, 7, 5, 0, 0, 0]);
}
