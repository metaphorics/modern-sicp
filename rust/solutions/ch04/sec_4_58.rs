// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.58: the `big-shot` rule. A person
//! is a big shot in a division when they work there and their supervisor
//! works in a different division; the rule compares the two leading
//! division symbols with `same` under a `not`.

use ch04::sec_4_4::{Engine, microshaft};

mod ex_4_58 {
    //! Exercise 4.58: the big-shot rule and its query.

    use super::*;

    /// One engine carrying the `big-shot` rule.
    pub fn engine() -> Engine {
        let engine = microshaft();
        engine.load(&[
            "(rule (big-shot ?person ?division) \
             (and (job ?person (?division . ?rest)) \
             (supervisor ?person ?boss) \
             (job ?boss (?boss-division . ?boss-rest)) \
             (not (same ?division ?boss-division))))",
            "(rule (same ?x ?x))",
        ]);
        engine
    }
}

#[test]
fn ex_4_58() {
    // The two big shots: Ben reports to the administration big wheel
    // while working in the computer division, and Scrooge reports to
    // Warbucks while heading accounting. Everyone else's supervisor sits
    // in the same division, and Warbucks himself has no supervisor row,
    // so the rule cannot fire for him.
    assert_eq!(
        ex_4_58::engine().answers("(big-shot ?person ?division)"),
        [
            "(big-shot (Bitdiddle Ben) computer)",
            "(big-shot (Scrooge Eben) accounting)",
        ]
    );
}
