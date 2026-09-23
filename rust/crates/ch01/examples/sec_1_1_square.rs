// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 1.1, listing 3: `square`, `sum_of_squares`, and `f`, with the
//! substitution walk of `f(5)`.

use ch01::sec_1_1::{f, square, sum_of_squares};

fn main() {
    println!("{}", square(21));
    // => 441
    assert_eq!(square(21), 441);

    println!("{}", square(2 + 5));
    // => 49
    assert_eq!(square(2 + 5), 49);

    println!("{}", square(square(3)));
    // => 81
    assert_eq!(square(square(3)), 81);

    println!("{}", sum_of_squares(3, 4));
    // => 25
    assert_eq!(sum_of_squares(3, 4), 25);

    println!("{}", f(5));
    // => 136
    assert_eq!(f(5), 136);
}
