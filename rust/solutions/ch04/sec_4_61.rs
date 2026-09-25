// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.61: the book's `next-to` rules
//! over lists. The base rule fires when the list starts with the two
//! elements; the recursive rule strips the head; the interleaved rule
//! streams pin the answer order.

use ch04::sec_4_4::{Engine, QueryEngine};

mod ex_4_61 {
    //! Exercise 4.61: the `next-to` relation over two-element adjacencies.

    use super::*;

    /// One engine carrying the book's two rules.
    pub fn engine() -> Engine {
        let engine = QueryEngine::new();
        engine.load(&[
            "(rule (?x next-to ?y in (?x ?y . ?u)))",
            "(rule (?x next-to ?y in (?v . ?z)) (?x next-to ?y in ?z))",
        ]);
        engine
    }
}

#[test]
fn ex_4_61() {
    // The book's first query: the base rule answers the head pair, and
    // the recursion answers the tail pair; the interleaved streams put
    // the head pair first.
    assert_eq!(
        ex_4_61::engine().answers("(?x next-to ?y in (1 (2 3) 4))"),
        [
            "(1 next-to (2 3) in (1 (2 3) 4))",
            "((2 3) next-to 4 in (1 (2 3) 4))",
        ]
    );
    // The book's second query: both adjacencies of the element 1.
    assert_eq!(
        ex_4_61::engine().answers("(?x next-to 1 in (2 1 3 1))"),
        ["(2 next-to 1 in (2 1 3 1))", "(3 next-to 1 in (2 1 3 1))",]
    );
}
