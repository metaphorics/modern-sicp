// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.3.1

//! Section 2.3.1: quotation and symbols, demonstrated by `memq`.

use ch02::sec_2_2::{List, leaf, sub};
use ch02::sec_2_3::memq;
use sicp_runtime::Symbol;

fn sym(s: &str) -> Symbol {
    Symbol::from(s)
}

fn main() {
    // A quoted list of symbols is built directly, with no reader: the
    // book's `(list 'a 'b 'c)`.
    let letters: List<Symbol> = List::from_iter([sym("a"), sym("b"), sym("c")]);
    println!("{letters}");
    // => (a b c)
    assert_eq!(letters.to_string(), "(a b c)");

    // A list containing a list: the book's `(list (list 'george))`.
    // Nesting is a `Nest`, exactly as 2.2's heterogeneous lists are.
    let nested = sub(&[sub(&[leaf(sym("george"))])]);
    println!("{nested}");
    // => ((george))
    assert_eq!(nested.to_string(), "((george))");

    // `memq` scans for the first `eq?` (here, content-equal `Symbol`)
    // match and returns the sublist starting there.
    let pantry: List<Symbol> = List::from_iter([sym("apple"), sym("pear"), sym("banana")]);
    let found = memq(&sym("pear"), &pantry);
    let found_str = found.map_or_else(|| "false".to_string(), |l| l.to_string());
    println!("{found_str}");
    // => (pear banana)
    assert_eq!(found_str, "(pear banana)");

    let feed: List<Symbol> = List::from_iter([sym("red"), sym("shoes"), sym("blue"), sym("socks")]);
    let miss = memq(&sym("red"), &feed);
    let miss_str = miss.map_or_else(|| "false".to_string(), |l| l.to_string());
    println!("{miss_str}");
    // => (red shoes blue socks)
    assert_eq!(miss_str, "(red shoes blue socks)");
}
