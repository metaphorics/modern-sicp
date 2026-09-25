// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.65: the wheel query that lists
//! Warbucks four times. The `wheel` rule composes two supervisor steps,
//! and the data base offers three middle managers (Alyssa, Ben, Scrooge)
//! whose own supervisor rows line up: Ben wheels through each of his
//! three supervisees in one route, and Warbucks through four, so the
//! stream prints Ben once and Warbucks four times -- the book's listing
//! in scan-order rows.

use ch04::sec_4_4::{Engine, microshaft};

mod ex_4_65 {
    //! Exercise 4.65: the fourfold wheel listing.

    use super::*;

    /// One Microshaft engine carrying the `wheel` rule of 4.4.1.
    pub fn engine() -> Engine {
        let engine = microshaft();
        engine.load(&["(rule (wheel ?person) \
             (and (supervisor ?middle-manager ?person) \
             (supervisor ?x ?middle-manager)))"]);
        engine
    }
}

#[test]
fn ex_4_65() {
    // The book's response, row for row the same multiset: three
    // derivation routes reach Ben (one per supervisee of his that has a
    // supervisee of their own -- just Louis) -- no: Ben appears once,
    // through Louis; Warbucks appears four times, once per supervised
    // middle manager with a supervisee (Alyssa via Louis, Ben via each
    // of Alyssa, Fect, and Tweakit, Scrooge via Cratchet).
    assert_eq!(
        ex_4_65::engine().answers("(wheel ?who)"),
        [
            "(wheel (Bitdiddle Ben))",
            "(wheel (Warbucks Oliver))",
            "(wheel (Warbucks Oliver))",
            "(wheel (Warbucks Oliver))",
            "(wheel (Warbucks Oliver))",
        ]
    );
}
