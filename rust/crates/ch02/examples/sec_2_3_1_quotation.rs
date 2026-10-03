// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 2.3.1

//! Section 2.3.1: quotation. Quotation disappears into constructors: a
//! symbol is a Rust string literal converted to `Symbol`, and a quoted
//! list is built with `leaf`/`sub` rather than read by an interpreter.

use ch02::sec_2_2::{List, Nest, leaf, sub};
use ch02::sec_2_3::memq;
use sicp_runtime::Symbol;

fn sym(s: &str) -> Symbol {
    Symbol::from(s)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A flat list of symbols, built from the symbols themselves.
    let letters: List<Symbol> = List::from_iter([sym("a"), sym("b"), sym("c")]);
    println!("{letters}");
    // => (a b c)
    assert_eq!(letters.to_string(), "(a b c)");

    // A list containing one list: `Nest`
    // holds this shape directly, the same tree type 2.2 built for
    // nested list structure.
    let nested: Nest<Symbol> = sub(&[sub(&[leaf(sym("george"))])]);
    println!("{nested}");
    // => ((george))
    assert_eq!(nested.to_string(), "((george))");

    // `memq` scans for a symbol by `eq?`-style comparison and returns
    // the sublist starting at the first match, or `None`.
    let inventory: List<Symbol> = List::from_iter([sym("apple"), sym("pear"), sym("banana")]);
    let found = memq(&sym("pear"), &inventory);
    println!("{}", found.as_ref().map_or("false".to_string(), ToString::to_string));
    // => (pear banana)
    assert_eq!(found.map(|l| l.to_string()), Some("(pear banana)".to_string()));

    let absent = memq(&sym("plum"), &inventory);
    println!("{}", absent.is_none());
    // => true
    assert!(absent.is_none());

    Ok(())
}
