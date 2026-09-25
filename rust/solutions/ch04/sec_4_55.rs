// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.55: three simple queries over the
//! Microshaft data base. Each is one pattern handed to the driver; the
//! answers are the data-base rows in insertion order.

use ch04::sec_4_4::microshaft;

mod ex_4_55 {
    //! Exercise 4.55: simple queries -- Ben's supervisees, the accounting
    //! division, and the Slumerville residents.

    use super::*;

    /// Runs `query` over a fresh Microshaft engine.
    pub fn answers(query: &str) -> Vec<String> {
        microshaft().answers(query)
    }
}

#[test]
fn ex_4_55() {
    // a. everyone supervised by Ben Bitdiddle.
    assert_eq!(
        ex_4_55::answers("(supervisor ?name (Bitdiddle Ben))"),
        [
            "(supervisor (Hacker Alyssa P) (Bitdiddle Ben))",
            "(supervisor (Fect Cy D) (Bitdiddle Ben))",
            "(supervisor (Tweakit Lem E) (Bitdiddle Ben))",
        ]
    );
    // b. the names and jobs of the accounting division. The dotted
    // pattern matches the whole division, so both the chief accountant
    // and the scrivener come back.
    assert_eq!(
        ex_4_55::answers("(job ?name (accounting . ?type))"),
        [
            "(job (Scrooge Eben) (accounting chief accountant))",
            "(job (Cratchet Robert) (accounting scrivener))",
        ]
    );
    // c. the names and addresses of everyone in Slumerville.
    assert_eq!(
        ex_4_55::answers("(address ?name (Slumerville . ?where))"),
        [
            "(address (Bitdiddle Ben) (Slumerville (Ridge Road) 10))",
            "(address (Reasoner Louis) (Slumerville (Pine Tree Road) 80))",
            "(address (Aull DeWitt) (Slumerville (Onion Square) 5))",
        ]
    );
}
