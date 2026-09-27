// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.1, listing 2: checked multiplication instead of wrapping.

use ch00::sec_0_1::checked_square;

fn main() {
    let ok = checked_square(1_000_000_000_000);
    println!("{ok:?}");
    // => Some(1000000000000000000000000)
    assert_eq!(ok, Some(1_000_000_000_000_000_000_000_000));

    let overflow = checked_square(i128::MAX);
    println!("{overflow:?}");
    // => None
    assert_eq!(overflow, None);
}
