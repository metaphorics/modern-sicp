// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.12: the book's copying `append`
//! against the mutating `append!`, and what each leaves in the tail of
//! `x`. The idea is classified A for this edition: the chains live in
//! shared mutable cells under `Rc`, so the exercise is the same pointer
//! argument made visible in the types.

use ch03::sec_3_3::{append_bang, first_pair};
use sicp_runtime::{Pair, Value, cons_cell, eq_pair};

mod ex_3_12 {
    use super::{Pair, Value, advance_pair, append_bang, append_value, eq_pair, first_pair};

    /// Exercise 3.12: append! mutates the tail of a shared chain
    ///
    /// Runs the book's interaction: the copying `append`, the `cdr` of
    /// `x` after it, then the mutating `append!` and the `cdr` of `x`
    /// after that. The answers are the printed forms, in order.
    #[must_use]
    pub fn ex_3_12() -> Vec<String> {
        let x = Value::list(vec![Value::sym("a"), Value::sym("b")]);
        let y = Value::list(vec![Value::sym("c"), Value::sym("d")]);

        // Each answer prints the moment it is read, as at the REPL: a
        // `Value::Pair` is a live handle, so stringifying now freezes
        // the structure as it stands.
        let z = append_value(&x, &y).to_string();
        let cdr_x_after_append = first_pair(&x).cdr.borrow().clone().to_string();

        let w = Value::Pair(append_bang(&first_pair(&x), &first_pair(&y))).to_string();
        let cdr_x_after_append_bang = first_pair(&x).cdr.borrow().clone().to_string();

        vec![z, cdr_x_after_append, w, cdr_x_after_append_bang]
    }

    /// The second name for the mutated chain sees the spliced cells:
    /// after `append!`, `x` and `w` end in the very pairs of `y`.
    #[must_use]
    pub fn x_and_w_share_their_tail(x: &Pair, w: &Pair) -> bool {
        let mut cursor = Pair::clone(x);
        let mut other = Pair::clone(w);
        loop {
            if eq_pair(&cursor, &other) {
                return true;
            }
            let next = advance_pair(&cursor);
            let other_next = advance_pair(&other);
            match (next, other_next) {
                (Some(a), Some(b)) => {
                    cursor = a;
                    other = b;
                }
                _ => return false,
            }
        }
    }
}

/// The next pair of a chain, if the chain continues.
fn advance_pair(cursor: &Pair) -> Option<Pair> {
    let next = cursor.cdr.borrow().clone();
    match next {
        Value::Pair(cell) => Some(cell),
        _ => None,
    }
}

/// The book's `append` of 2.2.1: copies the spine of `x`.
#[must_use]
pub fn append_value(x: &Value, y: &Value) -> Value {
    match x {
        Value::Pair(cell) => {
            let rest = append_value(&cell.cdr.borrow(), y);
            Value::Pair(cons_cell(cell.car.borrow().clone(), rest))
        }
        _ => y.clone(),
    }
}

#[test]
fn ex_3_12() {
    // The copying append leaves x alone; the cdr of x is still (b).
    // The mutating append! splices y onto the last pair of x, so the
    // cdr of x becomes (b c d).
    assert_eq!(
        ex_3_12::ex_3_12(),
        vec!["(a b c d)", "(b)", "(a b c d)", "(b c d)"]
    );

    // The missing-car question of the statement: w shares its final
    // cells with x, because append! mutated x's last pair in place.
    let x = first_pair(&Value::list(vec![Value::sym("a"), Value::sym("b")]));
    let y = first_pair(&Value::list(vec![Value::sym("c"), Value::sym("d")]));
    let w = append_bang(&x, &y);
    assert!(ex_3_12::x_and_w_share_their_tail(&x, &w));
}
