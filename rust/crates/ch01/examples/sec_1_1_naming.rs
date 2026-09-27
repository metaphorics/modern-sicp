// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 1.1, listing 2: naming things with `let`.

fn main() {
    let size = 2;
    println!("{size}");
    // => 2
    assert_eq!(size, 2);

    println!("{}", 5 * size);
    // => 10
    assert_eq!(5 * size, 10);

    // The book's `pi` is this five-digit approximation, not `f64::consts::PI`.
    #[allow(clippy::approx_constant)]
    let pi: f64 = 3.14159;
    let radius: f64 = 10.0;

    let area = pi * radius * radius;
    println!("{area}");
    // => 314.159
    assert!((area - 314.159).abs() < 1e-9);

    let circumference = 2.0 * pi * radius;

    println!("{circumference}");
    // => 62.8318
    assert!((circumference - 62.8318).abs() < 1e-9);
}
