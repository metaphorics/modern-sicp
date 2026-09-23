// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.6, listing: an `FnMut` closure, called through a mutable
//! reference to itself, mutating what it captured between calls.

fn call_three_times(mut f: impl FnMut() -> i32) -> Vec<i32> {
    vec![f(), f(), f()]
}

fn main() {
    let mut count = 0;
    let results = call_three_times(|| {
        count += 1;
        count
    });
    println!("{results:?}");
    // => [1, 2, 3]
    assert_eq!(results, vec![1, 2, 3]);
}
