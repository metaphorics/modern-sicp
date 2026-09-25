// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.64: Louis Reasoner's
//! `outranked-by` rule, whose recursive conjunct comes before the
//! supervisor test. The delayed engine still streams the direct
//! supervisor first, but no second answer ever arrives: the recursive
//! branch diverges. A fuel-bounded fallback counts the simple-query
//! invocations and cuts the stream when the budget is gone, which pins
//! the divergence without executing it.

use std::cell::Cell;
use std::rc::Rc;

use ch04::sec_4_4::{Engine, microshaft};
use sicp_runtime::Stream;

mod ex_4_64 {
    //! Exercise 4.64: Louis's swapped rule and its divergence.

    use super::*;

    /// The rule as Louis types it: the recursion first.
    pub const LOUIS_RULE: &str = "(rule (outranked-by ?staff-person ?boss) \
     (or (supervisor ?staff-person ?boss) \
     (and (outranked-by ?middle-manager ?boss) \
     (supervisor ?staff-person ?middle-manager))))";

    /// One Microshaft engine with Louis's rule, whose simple-query
    /// fallback stops after `fuel` invocations and raises the flag.
    pub fn fueled(fuel: usize) -> (Engine, Rc<Cell<bool>>) {
        let engine = microshaft();
        engine.load(&[LOUIS_RULE, "(rule (same ?x ?x))"]);
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
fn ex_4_64() {
    // Ben's query asks for two answers. The direct supervisor, Warbucks,
    // streams out, and the hunt for a second runs the fuel dry: the
    // recursive branch diverges, so only one answer ever arrives. The
    // delay in simple-query is what lets even that first answer out
    // (exercise 4.71a removes the delay and loses it).
    let (engine, exhausted) = ex_4_64::fueled(1000);
    assert_eq!(
        engine.answers_upto("(outranked-by (Bitdiddle Ben) ?who)", 2),
        ["(outranked-by (Bitdiddle Ben) (Warbucks Oliver))"]
    );
    assert!(exhausted.get(), "the hunt for a second answer diverges");
}
