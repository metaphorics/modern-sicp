// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.2, listing: the same sum, recursive and iterative.

use ch00::sec_0_2::{sum_to_iterative, sum_to_recursive};

fn main() {
    let by_recursion = sum_to_recursive(100);
    println!("{by_recursion}");
    // => 5050
    assert_eq!(by_recursion, 5050);

    let by_loop = sum_to_iterative(100);
    println!("{by_loop}");
    // => 5050
    assert_eq!(by_loop, by_recursion);
}
