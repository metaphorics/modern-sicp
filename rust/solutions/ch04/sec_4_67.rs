// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.67: the loop detector. The
//! history holds one key per simple query being worked on: the query
//! pattern instantiated under the frame with every unbound variable a
//! wildcard -- the book's "history of patterns and frames", where the
//! wildcards erase the renamed rule variables and keep the shape of the
//! request. A frame whose shape is already in the history dies there;
//! new shapes are recorded and processed. The caller clears the history
//! between top-level queries, which is the detector's per-query scope.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use ch04::sec_4_4::{Engine, Frame, instantiate, microshaft};
use sicp_runtime::{Stream, Value};

mod ex_4_67 {
    //! Exercise 4.67: a loop detector installed as the simple-query
    //! fallback.

    use super::*;

    /// One engine whose untagged patterns pass a history check before
    /// the standard simple query runs. The returned cell is the
    /// history: clear it between top-level queries.
    pub fn engine_with_detector(rules: &[&str]) -> (Engine, Rc<RefCell<HashSet<String>>>) {
        let engine = microshaft();
        engine.load(rules);
        let history: Rc<RefCell<HashSet<String>>> = Rc::new(RefCell::new(HashSet::new()));
        let standard = engine.simple_query_proc();
        let keys = Rc::clone(&history);
        engine.set_fallback(Some(Rc::new(move |eng, pattern, frames| {
            let mut kept: Vec<Frame> = Vec::new();
            for frame in &frames {
                let key = wildcard_key(pattern, &frame);
                if keys.borrow_mut().insert(key) {
                    kept.push(frame);
                }
            }
            if kept.is_empty() {
                return Stream::Empty;
            }
            standard(eng, pattern, stream_of_frames(kept))
        })));
        (engine, history)
    }

    /// The book's key: the pattern instantiated under the frame, every
    /// unbound variable a wildcard, so renamed rule variables collapse
    /// to the shape of the request.
    pub fn wildcard_key(pattern: &Value, frame: &Frame) -> String {
        let wildcards = instantiate(pattern, frame, &|_var| Value::sym("*"));
        sicp_runtime::print_value(&wildcards)
    }

    /// A frame stream over stored frames, in order.
    fn stream_of_frames(items: Vec<Frame>) -> Stream<Frame> {
        fn build(items: Rc<[Frame]>, index: usize) -> Stream<Frame> {
            match items.get(index) {
                None => Stream::Empty,
                Some(head) => {
                    let head = head.clone();
                    Stream::cons_stream(head, move || build(Rc::clone(&items), index + 1))
                }
            }
        }
        build(Rc::from(items), 0)
    }
}

#[test]
fn ex_4_67() {
    // The married cycle: the rule's body re-asks the original query with
    // renamed variables, whose wildcard shape repeats, so the detector
    // cuts the recursion and the stream drains with the one genuine
    // answer.
    let (engine, history) = ex_4_67::engine_with_detector(&[
        "(married Minnie Mickey)",
        "(rule (married ?x ?y) (married ?y ?x))",
    ]);
    assert_eq!(
        engine.answers("(married Mickey ?who)"),
        ["(married Mickey Minnie)"]
    );
    history.borrow_mut().clear();
    // Ordinary queries are untouched under the same detector: the
    // supervisor query answers three, and the wheel rule composes its
    // two supervisor steps without tripping the history.
    assert_eq!(
        engine.answers("(supervisor ?name (Bitdiddle Ben))").len(),
        3
    );
    history.borrow_mut().clear();
    engine.load(&["(rule (wheel ?person) \
         (and (supervisor ?middle-manager ?person) \
         (supervisor ?x ?middle-manager)))"]);
    assert_eq!(engine.answers("(wheel ?who)").len(), 5);
}
