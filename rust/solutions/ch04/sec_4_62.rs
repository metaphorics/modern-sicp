// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.62: `last-pair` as rules. The
//! base rule matches a one-element list; the recursive rule strips the
//! head. The book's queries all answer, and the fully unbound query
//! generates an endless family of renamed spurious answers after the one
//! genuine answer, which the bounded sample pins.

use ch04::sec_4_4::{Engine, QueryEngine};

mod ex_4_62 {
    //! Exercise 4.62: last-pair as two rules.

    use super::*;

    /// One engine carrying the book's two rules.
    pub fn engine() -> Engine {
        let engine = QueryEngine::new();
        engine.load(&[
            "(rule (last-pair (?x) (?x)))",
            "(rule (last-pair (?u . ?v) ?y) (last-pair ?v ?y))",
        ]);
        engine
    }
}

#[test]
fn ex_4_62() {
    // The book's three finite queries.
    assert_eq!(
        ex_4_62::engine().answers("(last-pair (3) ?x)"),
        ["(last-pair (3) (3))"]
    );
    assert_eq!(
        ex_4_62::engine().answers("(last-pair (1 2 3) ?x)"),
        ["(last-pair (1 2 3) (3))"]
    );
    assert_eq!(
        ex_4_62::engine().answers("(last-pair (2 ?x) (3))"),
        ["(last-pair (2 3) (3))"]
    );
    // The list-less query: one genuine answer first, then the
    // recursive rule's endless renamed guesses, never a second real
    // one. Each spurious answer's tail has collapsed onto the bound
    // (3), and the head variables carry their rule-application ids.
    assert_eq!(
        ex_4_62::engine().answers_upto("(last-pair ?x (3))", 3),
        [
            "(last-pair (3) (3))",
            "(last-pair (?u-2 3) (3))",
            "(last-pair (?u-2 ?u-4 3) (3))",
        ]
    );
}
