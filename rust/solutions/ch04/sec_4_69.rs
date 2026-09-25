// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.69: the greats chain. The rule
//! `((great . ?rel) ?x ?y)` descends one son step and re-asks the rest
//! of the relationship, whose variable-led query unifies against every
//! rule conclusion; the `((grandson) ?x ?y)` rule anchors the recursion.
//! The book's checks come out -- Irad is Adam's great-grandson, Jabal
//! and Jubal are the great-great-great-great-great-grandsons -- beside
//! the partial matches the variable-led query also exposes.

use ch04::sec_4_4::{Engine, QueryEngine};

mod ex_4_69 {
    //! Exercise 4.69: the greats rules over the Genesis data base.

    use super::*;

    /// The Genesis data base and rules of 4.63 plus the greats rules.
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
            "(rule ((grandson) ?x ?y) (grandson ?x ?y))",
            "(rule ((great . ?rel) ?x ?y) (and (son ?x ?z) (?rel ?z ?y)))",
        ]);
        engine
    }
}

#[test]
fn ex_4_69() {
    // The book's first check: every great-grandson pair.
    assert_eq!(
        ex_4_69::engine().answers("((great grandson) ?g ?ggs)"),
        [
            "((great grandson) Adam Irad)",
            "((great grandson) Cain Mehujael)",
            "((great grandson) Enoch Methushael)",
            "((great grandson) Irad Lamech)",
            "((great grandson) Mehujael Jabal)",
            "((great grandson) Mehujael Jubal)",
        ]
    );
    // The book's second check: every relationship Adam has to Irad. The
    // genuine answer is (great grandson); the dotted and son-tailed
    // shapes are the partial matches the variable-led query exposes --
    // the greats rule's ?rel and a son rule reached one great short --
    // each a renamed-rule reading the same facts.
    assert_eq!(
        ex_4_69::engine().answers("(?relationship Adam Irad)"),
        [
            "((great . grandson) Adam Irad)",
            "((great grandson) Adam Irad)",
            "((great great . son) Adam Irad)",
        ]
    );
    // The book's headline: Jabal at five greats, exactly as the prose
    // counts them.
    assert_eq!(
        ex_4_69::engine().answers("((great great great great great grandson) Adam ?who)"),
        [
            "((great great great great great grandson) Adam Jabal)",
            "((great great great great great grandson) Adam Jubal)",
        ]
    );
}
