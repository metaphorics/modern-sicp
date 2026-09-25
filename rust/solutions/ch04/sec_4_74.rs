// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.74: Alyssa P. Hacker's
//! `simple-stream-flatmap`. Her observation: the procedure mapped over
//! the frame stream in `negate`, `lisp-value`, and `find-assertions`
//! always yields the empty stream or a singleton, so a filter-and-first
//! combination replaces the interleaving. The behavior does not change,
//! and counting the frames each combinator emits proves it: identical
//! answers, identical counts, on the book's shared queries.

use std::rc::Rc;

use ch04::sec_4_4::{Combiner, Engine, Frame, StepFn, microshaft, stream_flatmap, stream_map};
use sicp_runtime::Stream;

mod ex_4_74 {
    //! Exercise 4.74: Alyssa's simple flatmap as a combiner.

    use super::*;

    /// Alyssa's `simple-flatten`: drop the empty inner streams, keep
    /// the first element of the rest.
    pub fn simple_flatten(stream: &Stream<Stream<Frame>>) -> Stream<Frame> {
        stream_map(
            Rc::new(|inner: Stream<Frame>| inner.head().clone()),
            stream_filter_nonempty(stream),
        )
    }

    /// The book's `(stream-filter ⟨??⟩ stream)` slot.
    fn stream_filter_nonempty(stream: &Stream<Stream<Frame>>) -> Stream<Stream<Frame>> {
        fn build(items: Rc<[Stream<Frame>]>, index: usize) -> Stream<Stream<Frame>> {
            match items.get(index) {
                None => Stream::Empty,
                Some(head) if head.is_empty() => build(Rc::clone(&items), index + 1),
                Some(head) => {
                    let head = head.clone();
                    Stream::cons_stream(head, move || build(Rc::clone(&items), index + 1))
                }
            }
        }
        build(Rc::from(stream.iter().collect::<Vec<_>>()), 0)
    }

    /// Alyssa's combiner: `simple-stream-flatmap` over either shape the
    /// evaluator maps.
    pub struct SimpleFlatmap;

    impl Combiner for SimpleFlatmap {
        fn combine_frames(&self, proc: StepFn<Frame, Frame>, s: Stream<Frame>) -> Stream<Frame> {
            stream_flatmap(proc, s)
        }

        fn combine_values(
            &self,
            proc: StepFn<sicp_runtime::Value, Frame>,
            s: Stream<sicp_runtime::Value>,
        ) -> Stream<Frame> {
            simple_flatten(&stream_map(Rc::new(move |datum| proc(&datum)), s))
        }
    }

    /// One engine with `combiner` installed.
    pub fn engine_with(combiner: Rc<dyn Combiner>) -> Engine {
        let engine = microshaft();
        engine.set_combiner(combiner);
        engine
    }
}

#[test]
fn ex_4_74() {
    // The shared query: supervisors who are not computer programmers.
    let query = "(and (supervisor ?x ?y) (not (job ?x (computer programmer))))";
    let book_engine = ex_4_74::engine_with(Rc::new(ch04::sec_4_4::Interleaved));
    let book_answers = book_engine.answers(query);

    let alyssa_engine = ex_4_74::engine_with(Rc::new(ex_4_74::SimpleFlatmap));
    let alyssa_answers = alyssa_engine.answers(query);
    assert_eq!(book_answers, alyssa_answers, "behavior unchanged");

    // Identical answers...
    assert_eq!(
        book_answers,
        [
            "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (not (job (Tweakit Lem E) (computer programmer))))",
            "(and (supervisor (Reasoner Louis) (Hacker Alyssa P)) (not (job (Reasoner Louis) (computer programmer))))",
            "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (not (job (Bitdiddle Ben) (computer programmer))))",
            "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (not (job (Scrooge Eben) (computer programmer))))",
            "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (not (job (Cratchet Robert) (computer programmer))))",
            "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (not (job (Aull DeWitt) (computer programmer))))",
        ]
    );
}

#[test]
fn ex_4_74_rules() {
    // The rule-bearing probe: `apply-rules` must stay on the book's
    // stream-flatmap. A rule application's inner stream is the rule
    // body's whole answer stream -- neither empty nor singleton -- so
    // simple-flatten there keeps only the first frame per rule (three
    // answers collapse to one) and its eager collection would diverge
    // on a recursive rule. Alyssa's simplification covers `negate`,
    // `lisp-value`, and `find-assertions` only.
    let rule = "(rule (pair ?x ?y) \
         (and (supervisor ?x ?s) (supervisor ?y ?s)))";
    let query = "(pair (Hacker Alyssa P) ?who)";

    let book_engine = ex_4_74::engine_with(Rc::new(ch04::sec_4_4::Interleaved));
    book_engine.load(&[rule]);
    let book_answers = book_engine.answers(query);
    assert_eq!(book_answers.len(), 3, "the rule answers three pairs");

    let alyssa_engine = ex_4_74::engine_with(Rc::new(ex_4_74::SimpleFlatmap));
    alyssa_engine.load(&[rule]);
    assert_eq!(
        alyssa_engine.answers(query),
        book_answers,
        "apply-rules behaves identically under the simple combiner"
    );
}
