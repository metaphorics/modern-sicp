// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.76: `and` as a merge of two
//! independently produced frame streams instead of a series
//! combination. `merge-frames` folds one frame's bindings into the
//! other through `extend-if-consistent`; a compatibility check either
//! yields the merged frame or nothing. The counters pin the factor-of-k
//! saving the exercise analyzes.

use std::cell::Cell;
use std::rc::Rc;

use ch04::sec_4_4::{Engine, Frame, QProc, microshaft, pattern_match, qeval_engine, split_list};
use sicp_runtime::{Stream, Value, cons_cell};

/// Builds a frame stream from collected items.
#[must_use]
pub fn stream_of(items: impl IntoIterator<Item = Frame>) -> Stream<Frame> {
    fn build(items: Rc<[Frame]>, index: usize) -> Stream<Frame> {
        match items.get(index) {
            None => Stream::Empty,
            Some(head) => {
                let head = head.clone();
                Stream::cons_stream(head, move || build(Rc::clone(&items), index + 1))
            }
        }
    }
    build(Rc::from(items.into_iter().collect::<Vec<_>>()), 0)
}

mod ex_4_76 {
    //! Exercise 4.76: the merging `and`.

    use super::*;

    /// The book's `merge-frames`: the bindings of `f2` extended onto
    /// `f1` when every one is consistent; `None` is incompatibility.
    pub fn merge_frames(f1: &Frame, f2: &Frame) -> Option<Frame> {
        let mut merged = f1.clone();
        for (variable, value) in f2.bindings() {
            merged = pattern_match(&variable, &value, &merged)?;
        }
        Some(merged)
    }

    /// The merging `and`: each conjunct's frames computed against the
    /// input stream, then all compatible pairs merged. The cell counts
    /// compatibility checks.
    pub fn merging_conjoin(checks: Rc<Cell<usize>>) -> QProc {
        Rc::new(move |engine, conjuncts, frames| merge_conjoin(engine, conjuncts, frames, &checks))
    }

    fn merge_conjoin(
        engine: &Engine,
        conjuncts: &Value,
        frames: Stream<Frame>,
        checks: &Rc<Cell<usize>>,
    ) -> Stream<Frame> {
        let (first, rest) = split_list(conjuncts);
        if rest.is_nil() {
            return qeval_engine(engine, &first, frames);
        }
        let s1 = qeval_engine(engine, &first, frames.clone());
        let rest_query = Value::Pair(cons_cell(Value::sym("and"), rest));
        let s2 = qeval_engine(engine, &rest_query, frames);
        let collected: Vec<(Frame, Frame)> = s1
            .iter()
            .flat_map(|f1| s2.iter().map(move |f2| (f1.clone(), f2)))
            .collect();
        checks.set(checks.get() + collected.len());
        stream_of(
            collected
                .into_iter()
                .filter_map(|(f1, f2)| merge_frames(&f1, &f2)),
        )
    }

    /// One engine with the merging `and` installed over the counter.
    pub fn engine() -> (Engine, Rc<Cell<usize>>) {
        let engine = microshaft();
        engine.load(&[
            "(rule (can-replace ?person-1 ?person-2) \
             (and (job ?person-1 ?job-1) (job ?person-2 ?job-2) \
             (or (same ?job-1 ?job-2) (can-do-job ?job-1 ?job-2)) \
             (not (same ?person-1 ?person-2))))",
            "(rule (same ?x ?x))",
        ]);
        let checks = Rc::new(Cell::new(0usize));
        engine.put("and", merging_conjoin(Rc::clone(&checks)));
        (engine, checks)
    }
}

#[test]
fn ex_4_76() {
    // The supervisors-with-jobs query: the series combination and the
    // merging one answer identically, and the merge costs 8 x 9 = 72
    // compatibility checks where the series pays 8 scans.
    let query = "(and (supervisor ?x ?y) (job ?x ?job))";
    let series_answers = microshaft().answers(query);
    let (merged, checks) = ex_4_76::engine();
    let merged_answers = merged.answers(query);
    assert_eq!(merged_answers, series_answers);
    assert_eq!(merged_answers.len(), 8);
    assert_eq!(checks.get(), 72);

    // The addresses-of-supervisees query: 8 x 9 = 72 checks under the
    // merge, same answers as the series combination.
    let query = "(and (supervisor ?x ?y) (address ?x ?where))";
    let series_answers = microshaft().answers(query);
    let (merged, checks) = ex_4_76::engine();
    let merged_answers = merged.answers(query);
    assert_eq!(merged_answers, series_answers);
    assert_eq!(merged_answers.len(), 8);
    assert_eq!(checks.get(), 72);
}
