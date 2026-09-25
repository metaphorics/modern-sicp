// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.63: the Genesis rules. The book's
//! data base lists sons and one wife; the `grandson` rule composes two
//! `son` steps, and the wife rule makes a man's wife's sons his own, which
//! is how Jabal and Jubal become Lamech's sons.

use ch04::sec_4_4::{Engine, QueryEngine};

mod ex_4_63 {
    //! Exercise 4.63: the genealogy of the descendants of Adam.

    use super::*;

    /// One engine carrying the Genesis data base and the two rules.
    pub fn engine() -> Engine {
        let engine = QueryEngine::new();
        engine.load(&[
            "(son Adam Cain)",
            "(son Cain Enoch)",
            "(son Enoch Irad)",
            "(son Irad Mehujael)",
            "(son Mehujael Methushael)",
            "(son Methushael Lamech)",
            "(wife Lamech Ada)",
            "(son Ada Jabal)",
            "(son Ada Jubal)",
            "(rule (grandson ?g ?s) (and (son ?f ?s) (son ?g ?f)))",
            "(rule (son ?m ?s) (and (wife ?m ?w) (son ?w ?s)))",
        ]);
        engine
    }
}

#[test]
fn ex_4_63() {
    // The grandson of Cain is Irad.
    assert_eq!(
        ex_4_63::engine().answers("(grandson Cain ?who)"),
        ["(grandson Cain Irad)"]
    );
    // The wife rule gives Lamech his wife's sons.
    assert_eq!(
        ex_4_63::engine().answers("(son Lamech ?who)"),
        ["(son Lamech Jabal)", "(son Lamech Jubal)"]
    );
    // And so the grandsons of Methushael are Jabal and Jubal, in the
    // data-base order of the son assertions.
    assert_eq!(
        ex_4_63::engine().answers("(grandson Methushael ?who)"),
        ["(grandson Methushael Jabal)", "(grandson Methushael Jubal)"]
    );
}
