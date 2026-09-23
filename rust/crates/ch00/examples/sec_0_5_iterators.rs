// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.5, listing: an iterator chain, then a `HashMap` built by hand.

use ch00::sec_0_5::{word_counts, word_lengths};

fn main() {
    let lengths = word_lengths("ready set go");
    println!("{lengths:?}");
    // => [5, 3, 2]
    assert_eq!(lengths, vec![5, 3, 2]);

    let words = ["ready", "set", "go", "set"];
    let counts = word_counts(&words);
    println!("{}", counts["set"]);
    // => 2
    assert_eq!(counts["set"], 2);
}
