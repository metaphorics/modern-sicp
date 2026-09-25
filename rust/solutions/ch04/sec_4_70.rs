// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.70: the `let` bindings in
//! `add-assertion!` and `add-rule!`. The exercise is a model in
//! miniature: a stream cell holding the assertion stream, the book's
//! `cons-stream` prepend, with and without the binding the book's `let`
//! performs. The broken cell re-reads the variable after the
//! assignment, so the delayed tail sees the new cell -- the stream
//! contains the newest assertion forever -- while the bound cell
//! captures the old stream and the walk replays the real history. The
//! edition's engine stores assertions in an appended vector, so the
//! hazard lives in this model, not in the engine.

use std::cell::RefCell;
use std::rc::Rc;

use sicp_runtime::{Stream, Value};

mod ex_4_70 {
    //! Exercise 4.70: the cyclic-assertion model.

    use super::*;

    /// A stream cell the way the book's `THE-ASSERTIONS` behaves: a
    /// named variable holding a stream of assertions.
    pub type Assertions = Rc<RefCell<Stream<Value>>>;

    /// The book's broken `add-assertion!`: `cons-stream`'s delayed tail
    /// wraps the variable, and forcing it reads the variable after the
    /// assignment -- the cycle of the book's `ones`.
    pub fn add_broken(cell: &Assertions, assertion: Value) {
        let cell2 = Rc::clone(cell);
        *cell.borrow_mut() = Stream::cons_stream(assertion, move || cell2.borrow().clone());
    }

    /// The book's `let`-bound `add-assertion!`: the old stream is bound
    /// before the assignment, and the tail holds that value.
    pub fn add_bound(cell: &Assertions, assertion: Value) {
        let old = cell.borrow().clone();
        *cell.borrow_mut() = Stream::cons_stream(assertion, move || old);
    }

    /// The first `n` assertions of the cell, the walk a query performs.
    pub fn take(cell: &Assertions, n: usize) -> Vec<String> {
        let mut out = Vec::new();
        for item in cell.borrow().iter() {
            out.push(sicp_runtime::print_value(&item));
            if out.len() >= n {
                break;
            }
        }
        out
    }
}

#[test]
fn ex_4_70() {
    // Adds a, then b, then c: the let-bound stream yields the real
    // history, newest first.
    let bound: ex_4_70::Assertions = Rc::new(RefCell::new(Stream::Empty));
    ex_4_70::add_bound(&bound, Value::sym("a"));
    ex_4_70::add_bound(&bound, Value::sym("b"));
    ex_4_70::add_bound(&bound, Value::sym("c"));
    assert_eq!(
        ex_4_70::take(&bound, 3),
        ["c", "b", "a"],
        "the let binding preserves the old stream"
    );
    // The broken body: each cons-stream's tail re-reads the variable,
    // so the stream contains the newest assertion forever -- c c c c.
    let broken: ex_4_70::Assertions = Rc::new(RefCell::new(Stream::Empty));
    ex_4_70::add_broken(&broken, Value::sym("a"));
    ex_4_70::add_broken(&broken, Value::sym("b"));
    ex_4_70::add_broken(&broken, Value::sym("c"));
    assert_eq!(
        ex_4_70::take(&broken, 4),
        ["c", "c", "c", "c"],
        "the broken tail loops on the newest assertion"
    );
}
