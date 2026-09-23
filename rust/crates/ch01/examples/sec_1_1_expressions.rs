// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 1.1, listing 1: primitive expressions and combinations.

fn main() {
    let value = 486;
    println!("{value}");
    // => 486
    assert_eq!(value, 486);

    let value = 137 + 349;
    println!("{value}");
    // => 486
    assert_eq!(value, 486);

    let value = 1000 - 334;
    println!("{value}");
    // => 666
    assert_eq!(value, 666);

    let value = 5 * 99;
    println!("{value}");
    // => 495
    assert_eq!(value, 495);

    let value = 10 / 5;
    println!("{value}");
    // => 2
    assert_eq!(value, 2);

    let value: f64 = 2.7 + 10.0;
    println!("{value}");
    // => 12.7
    assert!((value - 12.7).abs() < 1e-9);

    let value = 21 + 35 + 12 + 7;
    println!("{value}");
    // => 75
    assert_eq!(value, 75);

    let value = 25 * 4 * 12;
    println!("{value}");
    // => 1200
    assert_eq!(value, 1200);

    let value = (3 * 5) + (10 - 6);
    println!("{value}");
    // => 19
    assert_eq!(value, 19);

    let value = 3 * (2 * 4 + (3 + 5)) + (10 - 7 + 6);
    println!("{value}");
    // => 57
    assert_eq!(value, 57);
}
