// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.1, listing 1: shadowing versus mutation.

fn main() {
    let x = 5;
    let x = x + 1;
    println!("{x}");
    // => 6
    assert_eq!(x, 6);

    let mut y = 5;
    y += 1;
    println!("{y}");
    // => 6
    assert_eq!(y, 6);

    let is_even: bool = y % 2 == 0;
    println!("{is_even}");
    // => true
    assert!(is_even);

    let initial: char = 'r';
    println!("{initial}");
    // => r
    assert_eq!(initial, 'r');
}
