// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.34: printing lazy pairs. With
//! `cons` as the one non-strict primitive, lazy pairs carry their
//! delayed slots in a tagged value the printer can identify; the
//! driver's print rule forces the first ten elements and prints the
//! unprinted tail as the ellipsis, so an infinite list renders
//! finitely and a finite one whole.

use ch04::eval_support::*;

mod ex_4_34 {
    use super::*;

    /// The session: the tagged pair, the infinite `ones`, a demand on
    /// it, and a nested pair.
    const SESSION: &[&str] = &[
        "(cons 1 (cons 2 '()))",
        "(define ones (cons 1 ones))",
        "ones",
        "(car ones)",
        "(cons (cons 1 '()) (cons 2 '()))",
    ];

    /// The printable transcript of the session.
    #[must_use]
    pub fn transcript() -> String {
        printable_driver_transcript(&LazyPrintable, SESSION)
    }
}

#[test]
fn ex_4_34() {
    let transcript = ex_4_34::transcript();
    let values: Vec<&str> = transcript
        .lines()
        .filter_map(|line| line.strip_prefix(";;; L-Eval value: "))
        .collect();
    // The finite tagged pair prints whole.
    assert_eq!(values.first(), Some(&"(1 2)"));
    // The self-referential definition terminates: nothing is forced.
    assert_eq!(values.get(1), Some(&"ok"));
    // The infinite list prints its ten-element prefix and the
    // ellipsis, and the printer never forces past the budget.
    assert_eq!(values.get(2), Some(&"(1 1 1 1 1 1 1 1 1 1 ...)"));
    // A demand on the list still answers the element.
    assert_eq!(values.get(3), Some(&"1"));
    // A nested lazy pair prints inside its parent's parentheses.
    assert_eq!(values.last(), Some(&"((1) 2)"));
}
