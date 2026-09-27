// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.77: `not` and `lisp-value`
//! deferred until their variables are bound. A promised filter extends
//! the frame with a `*promise*` binding holding the tagged query; the
//! fulfillment pass discharges every promise once the rest of the
//! conjunction has bound the variables. The book's 4.4.3 wrong-answer
//! cases come out right, and the deferred/fulfilled counters pin the
//! machinery.

use std::cell::Cell;
use std::rc::Rc;

use ch04::sec_4_1::is_true;
use ch04::sec_4_4::{Engine, Frame, QProc, is_var, microshaft, singleton_stream, split_list};
use sicp_runtime::{SchemeError, Stream, Value, print_value};

mod ex_4_77 {
    //! Exercise 4.77: promised filters and their fulfillment.

    use super::*;

    /// The frame variable a promised filter binds.
    pub const PROMISE_VAR: &str = "*promise*";
    /// The tag of a deferred `not`.
    pub const NOT_TAG: &str = "deferred-not";
    /// The tag of a deferred `lisp-value`.
    pub const LISP_TAG: &str = "deferred-lisp-value";

    /// Whether `query` still names a variable the frame leaves unbound.
    pub fn has_unbound(query: &Value, frame: &Frame) -> bool {
        fn walk(exp: &Value, frame: &Frame) -> bool {
            if is_var(exp) {
                return frame.binding_in_frame(exp).is_none();
            }
            match exp {
                Value::Pair(cell) => {
                    walk(&cell.car.borrow(), frame) || walk(&cell.cdr.borrow(), frame)
                }
                _ => false,
            }
        }
        walk(query, frame)
    }

    /// The promised `not`: filter now when every variable is bound,
    /// promise otherwise.
    pub fn promised_negate(pending: Rc<Cell<usize>>) -> QProc {
        Rc::new(move |engine, operands, frames| {
            let (negated, _) = split_list(operands);
            let engine2 = Rc::clone(engine);
            let negated2 = negated.clone();
            let pending2 = Rc::clone(&pending);
            engine.flatmap(
                Rc::new(move |frame: &Frame| {
                    if !has_unbound(&negated2, frame) {
                        return filter_not(&engine2, &negated2, frame);
                    }
                    pending2.set(pending2.get() + 1);
                    singleton_stream(promise(frame, NOT_TAG, &negated2))
                }),
                frames,
            )
        })
    }

    /// The promised `lisp-value`: same shape, the call deferred.
    pub fn promised_lisp_value(pending: Rc<Cell<usize>>) -> QProc {
        Rc::new(move |engine, call, frames| {
            let call2 = call.clone();
            let engine2 = Rc::clone(engine);
            let pending2 = Rc::clone(&pending);
            engine.flatmap(
                Rc::new(move |frame: &Frame| {
                    if !has_unbound(&call2, frame) {
                        return filter_lisp_value(&engine2, &call2, frame);
                    }
                    pending2.set(pending2.get() + 1);
                    singleton_stream(promise(frame, LISP_TAG, &call2))
                }),
                frames,
            )
        })
    }

    /// Extends the frame's `*promise*` binding with one more promise.
    fn promise(frame: &Frame, tag: &str, query: &Value) -> Frame {
        let tagged = Value::tagged(tag, query.clone());
        let existing = frame
            .binding_in_frame(&Value::sym(PROMISE_VAR))
            .and_then(|value| value.list_items().ok())
            .unwrap_or_default();
        let mut promises = existing;
        promises.push(tagged);
        frame.extend(Value::sym(PROMISE_VAR), Value::list(promises))
    }

    /// The book's `negate` filter.
    fn filter_not(engine: &Engine, negated: &Value, frame: &Frame) -> Stream<Frame> {
        if engine
            .qeval(negated, singleton_stream(frame.clone()))
            .is_empty()
        {
            singleton_stream(frame.clone())
        } else {
            Stream::Empty
        }
    }

    /// The book's `lisp-value` filter; an unbound variable here only
    /// when a promise discharges early, which fails the frame.
    fn filter_lisp_value(engine: &Engine, call: &Value, frame: &Frame) -> Stream<Frame> {
        let instantiated = ch04::sec_4_4::instantiate(call, frame, &|var: &Value| {
            ch04::sec_4_4::contract_question_mark(var)
        });
        let holds = engine.execute(&instantiated).is_ok_and(|v| is_true(&v));
        if holds {
            singleton_stream(frame.clone())
        } else {
            Stream::Empty
        }
    }

    /// One Microshaft engine with the promised filters installed, plus
    /// the shared deferred counter.
    pub fn engine() -> (Engine, Rc<Cell<usize>>) {
        let engine = microshaft();
        let pending = Rc::new(Cell::new(0usize));
        engine.put("not", promised_negate(Rc::clone(&pending)));
        engine.put("lisp-value", promised_lisp_value(Rc::clone(&pending)));
        (engine, pending)
    }

    /// Runs `query` under the promised engine, then discharges every
    /// promise the surviving frames carry, counting the fulfillments,
    /// and answers the instantiated query strings.
    pub fn answers_fulfilled(
        engine: &Engine,
        fulfilled: &Rc<Cell<usize>>,
        query: &str,
    ) -> Vec<String> {
        let processed = ch04::sec_4_4::read_query(query);
        let frames: Vec<Frame> = engine.query_frames(&processed).iter().collect();
        frames
            .into_iter()
            .filter_map(|frame| discharge(engine, frame, fulfilled))
            .map(|frame| print_value(&ch04::sec_4_4::instantiate_query(&processed, &frame)))
            .collect()
    }

    fn discharge(engine: &Engine, frame: Frame, fulfilled: &Rc<Cell<usize>>) -> Option<Frame> {
        let Some(promises) = frame.binding_in_frame(&Value::sym(PROMISE_VAR)) else {
            return Some(frame);
        };
        let current = strip_promise(&frame);
        for promise in promises.list_items().unwrap_or_default() {
            let Value::Tagged { tag, data } = &promise else {
                continue;
            };
            fulfilled.set(fulfilled.get() + 1);
            let query = (**data).clone();
            let survives = match &**tag {
                t if t == NOT_TAG => engine
                    .qeval(&query, singleton_stream(current.clone()))
                    .is_empty(),
                t if t == LISP_TAG => {
                    let instantiated = ch04::sec_4_4::instantiate(&query, &current, &|var| {
                        ch04::sec_4_4::contract_question_mark(var)
                    });
                    engine.execute(&instantiated).is_ok_and(|v| is_true(&v))
                }
                _ => true,
            };
            if !survives {
                return None;
            }
        }
        Some(current)
    }

    fn strip_promise(frame: &Frame) -> Frame {
        let mut out = Frame::new();
        for (variable, value) in frame.bindings() {
            if variable != Value::sym(PROMISE_VAR) {
                out = out.extend(variable, value);
            }
        }
        out
    }
}

#[test]
fn ex_4_77() {
    // The naive filters on the book's 4.4.3 cases: the `not` sees the
    // empty frame, filters it, and the conjunction answers nothing.
    assert!(
        microshaft()
            .answers("(and (not (job ?x (computer programmer))) (supervisor ?x ?y))")
            .is_empty()
    );
    // The `lisp-value` raises the unknown-variable error outright.
    let (answers, error) = microshaft().answers_checked(
        "(and (lisp-value > ?amount 30000) (salary ?person ?amount))",
        5,
    );
    assert!(answers.is_empty());
    assert!(
        matches!(error, Some(SchemeError::TypeMismatch(text)) if text.contains("Unknown pat var"))
    );

    // The promised filters: same two queries, correct answers.
    let (engine, deferred) = ex_4_77::engine();
    let fulfilled = Rc::new(Cell::new(0usize));
    let not_answers = ex_4_77::answers_fulfilled(
        &engine,
        &fulfilled,
        "(and (not (job ?x (computer programmer))) (supervisor ?x ?y))",
    );
    assert_eq!(
        not_answers.len(),
        6,
        "the six supervisors outside programming"
    );
    assert_eq!(deferred.get(), 1, "one promise appended to the empty frame");
    assert_eq!(
        fulfilled.get(),
        8,
        "every supervisor frame's promise checked"
    );

    let lisp_answers = ex_4_77::answers_fulfilled(
        &engine,
        &fulfilled,
        "(and (lisp-value > ?amount 30000) (salary ?person ?amount))",
    );
    assert_eq!(lisp_answers.len(), 5, "the five salaries above 30000");
    assert_eq!(deferred.get(), 2, "one more promise on the empty frame");
    assert_eq!(fulfilled.get(), 17, "eight plus nine salary frames checked");
}
