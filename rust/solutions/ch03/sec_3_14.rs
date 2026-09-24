// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.14: the book's `mystery`, which
//! walks a chain once and turns it around in place by reversing every
//! `cdr` pointer. Classified A: the mutation is `set-cdr!` through the
//! shared `Rc` cells.

use ch03::sec_3_3::first_pair;
use sicp_runtime::{Pair, Value, cons_cell};

mod ex_3_14 {
    use super::{Value, first_pair, reverse_in_place};

    /// Exercise 3.14: mystery turns a chain around in place
    ///
    /// Runs the book's interaction on `v = (a b c d)` and reports how
    /// `v` and the returned `w` print afterwards. Both answers are
    /// stringified when read, so each shows the structure as it stood.
    #[must_use]
    pub fn ex_3_14() -> (String, String) {
        let v = Value::list(vec![
            Value::sym("a"),
            Value::sym("b"),
            Value::sym("c"),
            Value::sym("d"),
        ]);
        let w = reverse_in_place(&first_pair(&v));
        let v_now = v.to_string();
        let w_now = Value::Pair(w).to_string();
        (v_now, w_now)
    }
}

/// The book's `mystery`: one pass down the chain, pointing every cell's
/// `cdr` back at the previous cell. The old head ends as the last cell
/// of the reversed chain, so `v` prints as a one-element list.
#[must_use]
pub fn reverse_in_place(x: &Pair) -> Pair {
    let mut rest = Some(Pair::clone(x));
    let mut done: Option<Pair> = None;
    while let Some(cell) = rest {
        let next = cell.cdr.borrow().clone();
        let pointed_back = match &done {
            Some(previous) => Value::Pair(Pair::clone(previous)),
            None => Value::Nil,
        };
        *cell.cdr.borrow_mut() = pointed_back;
        done = Some(cell);
        rest = match next {
            Value::Pair(cell) => Some(cell),
            _ => None,
        };
    }
    match done {
        Some(cell) => cell,
        None => Pair::clone(x),
    }
}

/// A fresh `(a b c d)` for the tests.
fn abcd() -> Value {
    Value::list(vec![
        Value::sym("a"),
        Value::sym("b"),
        Value::sym("c"),
        Value::sym("d"),
    ])
}

#[test]
fn ex_3_14() {
    // w is the chain read backwards; v has been consumed down to (a).
    assert_eq!(
        ex_3_14::ex_3_14(),
        ("(a)".to_string(), "(d c b a)".to_string())
    );

    // The aliases agree: every one of the four original cells is still
    // reachable, only the pointers between them turned around.
    let v = abcd();
    let w = reverse_in_place(&first_pair(&v));
    assert_eq!(Value::Pair(w).to_string(), "(d c b a)");
    assert_eq!(v.to_string(), "(a)");

    // Mystery of a one-element list answers that element's cell.
    let single = Value::list(vec![Value::sym("a")]);
    let back = reverse_in_place(&first_pair(&single));
    assert_eq!(Value::Pair(back).to_string(), "(a)");
    let _ = cons_cell(Value::Nil, Value::Nil);
}
