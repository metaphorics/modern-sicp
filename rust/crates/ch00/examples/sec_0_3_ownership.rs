// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.3, listing: borrowing a `Vec` versus consuming it.

use ch00::sec_0_3::{append, consume_and_count, total_len};

fn main() {
    let words = vec![String::from("borrow"), String::from("checker")];

    let len = total_len(&words);
    println!("{len}");
    // => 13
    assert_eq!(len, 13);
    println!("{}", words.len());
    // => 2
    assert_eq!(words.len(), 2); // `words` is still owned here: `total_len` only borrowed it

    let mut words = words;
    append(&mut words, "words");
    println!("{words:?}");
    // => ["borrow", "checker", "words"]
    assert_eq!(words, vec!["borrow", "checker", "words"]);

    let count = consume_and_count(words);
    println!("{count}");
    // => 3
    assert_eq!(count, 3); // `words` moved into `consume_and_count` and is gone in this scope
}
