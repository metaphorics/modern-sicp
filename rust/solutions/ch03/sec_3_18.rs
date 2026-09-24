// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.18: deciding whether a chain
//! contains a cycle. The book's own answer walks with a history of
//! visited cells; the Rust history is a `HashSet` of addresses.

use ch03::sec_3_3::{first_pair, has_cycle, make_cycle};
use sicp_runtime::{Pair, Value};

mod ex_3_18 {
    use super::{Value, first_pair, has_cycle, make_cycle};

    /// Exercise 3.18: detect whether a list contains a cycle
    ///
    /// Answers the question for a plain three-pair chain and for the
    /// same chain after `make-cycle` closes it.
    #[must_use]
    pub fn ex_3_18() -> (bool, bool) {
        let plain = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
        let plain_answer = has_cycle(&first_pair(&plain));

        let ring = Value::list(vec![Value::sym("a"), Value::sym("b"), Value::sym("c")]);
        make_cycle(&first_pair(&ring));
        let ring_answer = has_cycle(&first_pair(&ring));

        (plain_answer, ring_answer)
    }
}

/// The `n`th pair of a chain, or the last one when the chain is short.
fn nth_pair(v: &Value, n: usize) -> Pair {
    let mut cursor = first_pair(v);
    for _ in 0..n {
        let next = cursor.cdr.borrow().clone();
        let Value::Pair(cell) = next else {
            return cursor;
        };
        cursor = cell;
    }
    cursor
}

fn set_cdr_to(cell: &Pair, value: Value) {
    *cell.cdr.borrow_mut() = value;
}

#[test]
fn ex_3_18() {
    assert_eq!(ex_3_18::ex_3_18(), (false, true));
}

/// A cycle joined far down the chain is still found, and the plain
/// prefix alone is not reported as one.
#[test]
fn detects_a_cycle_joined_far_down_the_chain() {
    let long = Value::list(vec![
        Value::sym("a"),
        Value::sym("b"),
        Value::sym("c"),
        Value::sym("d"),
        Value::sym("e"),
    ]);
    assert!(!has_cycle(&first_pair(&long)));
    set_cdr_to(&nth_pair(&long, 4), Value::Pair(first_pair(&long)));
    assert!(has_cycle(&first_pair(&long)));
}
