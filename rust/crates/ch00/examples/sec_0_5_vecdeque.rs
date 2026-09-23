// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.5, listing: `VecDeque`, pushing and popping from both ends
//! in constant time.

use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_front(0);
    println!("{queue:?}");
    // => [0, 1, 2]

    let front = queue.pop_front();
    println!("{front:?}");
    // => Some(0)
    assert_eq!(front, Some(0));
    assert_eq!(queue, VecDeque::from([1, 2]));
}
