// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.68: `reverse` as two rules over
//! `append-to-form`. The forward direction answers the book's query and
//! stops; the backward direction does derive the genuine answer -- the
//! unified append chain pulls the list apart -- but the stream never
//! runs dry, so the bounded sample pins the first answer and the fuel
//! probe pins the endless tail.

use std::cell::Cell;
use std::rc::Rc;

use ch04::sec_4_4::{Engine, microshaft};
use sicp_runtime::Stream;

mod ex_4_68 {
    //! Exercise 4.68: reverse rules, forward and backward.

    use super::*;

    /// One Microshaft engine carrying `append-to-form` and `reverse`.
    pub fn engine() -> Engine {
        let engine = microshaft();
        engine.load(&[
            "(rule (append-to-form () ?y ?y))",
            "(rule (append-to-form (?u . ?v) ?y (?u . ?z)) (append-to-form ?v ?y ?z))",
            "(rule (reverse () ()))",
            "(rule (reverse (?u . ?v) ?y) (and (reverse ?v ?z) (append-to-form ?z (?u) ?y)))",
        ]);
        engine
    }

    /// The backward query under a simple-query fuel bound: the fallback
    /// stops after `fuel` invocations and raises the flag.
    pub fn fueled_backward(fuel: usize) -> (Engine, Rc<Cell<bool>>) {
        let engine = engine();
        let exhausted = Rc::new(Cell::new(false));
        let counter = Rc::new(Cell::new(0usize));
        let standard = engine.simple_query_proc();
        let flag = Rc::clone(&exhausted);
        let count = Rc::clone(&counter);
        engine.set_fallback(Some(Rc::new(move |eng, pattern, frames| {
            let n = count.get() + 1;
            count.set(n);
            if n > fuel {
                flag.set(true);
                return Stream::Empty;
            }
            standard(eng, pattern, frames)
        })));
        (engine, exhausted)
    }
}

#[test]
fn ex_4_68() {
    // Forward: the one answer, then the stream runs dry.
    assert_eq!(
        ex_4_68::engine().answers("(reverse (1 2 3) ?x)"),
        ["(reverse (1 2 3) (3 2 1))"]
    );
    // Backward: the genuine answer (3 2 1) does come out -- the append
    // constraints and the reverse recursion meet -- but the stream then
    // generates without end, so no second answer arrives within the
    // fuel and the sample pins exactly one.
    let (engine, exhausted) = ex_4_68::fueled_backward(2000);
    assert_eq!(
        engine.answers_upto("(reverse ?x (1 2 3))", 2),
        ["(reverse (3 2 1) (1 2 3))"]
    );
    assert!(exhausted.get(), "the backward stream never runs dry");
}
