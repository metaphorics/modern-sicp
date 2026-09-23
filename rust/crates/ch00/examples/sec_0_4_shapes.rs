// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.4, listing: a sum-of-products `Shape`, and an `Option` search.

use ch00::sec_0_4::{Shape, first_even};

fn main() {
    let circle = Shape::Circle { radius: 2.0 };
    let area = circle.area();
    println!("{area:.2}");
    // => 12.57
    assert!((area - 12.566_370_614_359_172).abs() < 1e-9);

    let found = first_even(&[1, 3, 4, 5]);
    println!("{found:?}");
    // => Some(4)
    assert_eq!(found, Some(4));

    let none = first_even(&[1, 3, 5]);
    println!("{none:?}");
    // => None
    assert_eq!(none, None);
}
