// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.4, listing: a `Box`-linked list, a recursive enum whose one
//! self-referential variant needs an indirection to have a known size.

use ch00::sec_0_4::{List, list_sum};

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    let sum = list_sum(&list);
    println!("{sum}");
    // => 6
}
