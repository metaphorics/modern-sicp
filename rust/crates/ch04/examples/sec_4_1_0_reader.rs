// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 4.1 given machinery: the S-expression reader and the corpus
//! printer over the section's syntax. The reader is the book's `read`;
//! the printer is the one contract the conformance output is held to.

use sicp_runtime::{display_value, print_value, read, read_program};

fn main() {
    // Atoms: integers, floats, booleans, symbols, strings.
    println!("{}", print_value(&read("42").expect("read")));
    // => 42
    assert_eq!(print_value(&read("42").expect("read")), "42");
    println!("{}", print_value(&read("2.5").expect("read")));
    // => 2.5
    assert_eq!(print_value(&read("2.5").expect("read")), "2.5");
    println!("{}", print_value(&read("1.5e-7").expect("read")));
    // => 1.5e-7
    assert_eq!(print_value(&read("1.5e-7").expect("read")), "1.5e-7");
    println!("{}", print_value(&read("#t").expect("read")));
    // => #t
    assert_eq!(print_value(&read("#t").expect("read")), "#t");
    println!("{}", print_value(&read("set!").expect("read")));
    // => set!
    assert_eq!(print_value(&read("set!").expect("read")), "set!");

    // A string's value form is quoted and escaped; `display` prints it
    // bare.
    let text = read("\"a \\\"quoted\\\" word\"").expect("read");
    println!("{}", print_value(&text));
    // => "a \"quoted\" word"
    assert_eq!(print_value(&text), "\"a \\\"quoted\\\" word\"");
    println!("{}", display_value(&text));
    // => a "quoted" word
    assert_eq!(display_value(&text), "a \"quoted\" word");

    // Lists, quote sugar, and dotted pairs round-trip.
    let form = read("'(a b 1.5)").expect("read");
    println!("{}", print_value(&form));
    // => (quote (a b 1.5))
    assert_eq!(print_value(&form), "(quote (a b 1.5))");
    let dotted = read("(cons 1 2)").expect("read");
    println!("{}", print_value(&dotted));
    // => (cons 1 2)
    assert_eq!(print_value(&dotted), "(cons 1 2)");

    // A program reads as a list of forms; comments and whitespace are
    // atmosphere.
    let program = read_program(";; header\n(define x 2)\n(* x x) ; trailing\n").expect("read");
    println!("forms={}", program.len());
    // => forms=2
    assert_eq!(program.len(), 2);
    let rendered = program
        .iter()
        .map(print_value)
        .collect::<Vec<_>>()
        .join("; ");
    println!("{rendered}");
    // => (define x 2); (* x x)
    assert_eq!(rendered, "(define x 2); (* x x)");
}
