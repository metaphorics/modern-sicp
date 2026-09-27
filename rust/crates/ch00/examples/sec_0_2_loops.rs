// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.2, listing: `while`, `loop`, and `for` side by side.

fn main() {
    // `while`: repeats while a condition holds, checked before each pass.
    let mut n = 10;
    let mut halvings = 0;
    while n > 1 {
        n /= 2;
        halvings += 1;
    }
    println!("{halvings}");
    // => 3

    // `loop`: repeats unconditionally; `break` can hand back a value.
    let mut count = 0;
    let first_over_20 = loop {
        count += 3;
        if count > 20 {
            break count;
        }
    };
    println!("{first_over_20}");
    // => 21

    // `for`: walks an iterator; here, a range.
    let mut total = 0;
    for i in 1..=5 {
        total += i;
    }
    println!("{total}");
    // => 15
}
