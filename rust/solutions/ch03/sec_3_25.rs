// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.25: a table whose keys are
//! lists of arbitrary length. A one-key path is the section's ordinary
//! record; a longer path descends into subtables, building any missing
//! ones on the way in.

use sicp_runtime::{Pair, Value, cons_cell};

use ch03::sec_3_3::{assoc_by, structural_same_key};

mod ex_3_25 {
    use super::{Value, cons_cell, insert_path, lookup_path, value_int};

    /// Exercise 3.25: a table keyed by lists of arbitrary length
    ///
    /// Stores values under paths of two and three keys, and reports the
    /// three lookups: two hits and one miss through a prefix that
    /// exists with a different continuation.
    #[must_use]
    pub fn ex_3_25() -> (Option<i128>, Option<i128>, Option<i128>) {
        let root = cons_cell(Value::sym("*table*"), Value::Nil);
        insert_path(&[Value::sym("a"), Value::sym("b")], Value::int(1), &root);
        insert_path(
            &[Value::sym("a"), Value::sym("c"), Value::sym("d")],
            Value::int(2),
            &root,
        );

        (
            value_int(lookup_path(&[Value::sym("a"), Value::sym("b")], &root)),
            value_int(lookup_path(
                &[Value::sym("a"), Value::sym("c"), Value::sym("d")],
                &root,
            )),
            value_int(lookup_path(&[Value::sym("a"), Value::sym("c")], &root)),
        )
    }
}

#[allow(
    clippy::needless_pass_by_value,
    reason = "always called on an owned Option<Value> the caller no longer needs"
)]
fn value_int(v: Option<Value>) -> Option<i128> {
    match v {
        Some(Value::Int(n)) => Some(n),
        _ => None,
    }
}

/// The book's `lookup` over a key list: walks the subtable named by
/// every key but the last, then reads the record the last key names.
#[must_use]
pub fn lookup_path(keys: &[Value], table: &Pair) -> Option<Value> {
    let records = table.cdr.borrow().clone();
    let [only] = keys else {
        let [head, rest @ ..] = keys else {
            return None;
        };
        let subtable = assoc_by(head, &records, structural_same_key)?;
        return lookup_path(rest, &subtable);
    };
    assoc_by(only, &records, structural_same_key).map(|record| record.cdr.borrow().clone())
}

/// The book's `insert!` over a key list: descends into or builds the
/// subtables named by every key but the last, then stores the record.
pub fn insert_path(keys: &[Value], value: Value, table: &Pair) {
    let records = table.cdr.borrow().clone();
    let [only] = keys else {
        let [head, rest @ ..] = keys else {
            return;
        };
        if let Some(subtable) = assoc_by(head, &records, structural_same_key) {
            insert_path(rest, value, &subtable);
        } else {
            let fresh = cons_cell(head.clone(), build_nested(rest, value));
            *table.cdr.borrow_mut() = Value::Pair(cons_cell(Value::Pair(fresh), records));
        }
        return;
    };
    if let Some(record) = assoc_by(only, &records, structural_same_key) {
        *record.cdr.borrow_mut() = value;
    } else {
        let record = cons_cell(only.clone(), value);
        *table.cdr.borrow_mut() = Value::Pair(cons_cell(Value::Pair(record), records));
    }
}

/// The record list that stores `value` under the remaining keys: a
/// table's cdr is always a list of entries, even when it holds just
/// the one record or subtable this path needs, so every level wraps
/// its result in a one-element list rather than returning it bare.
fn build_nested(keys: &[Value], value: Value) -> Value {
    match keys {
        [] => Value::Nil,
        [only] => {
            let record = Value::Pair(cons_cell(only.clone(), value));
            Value::Pair(cons_cell(record, Value::Nil))
        }
        [head, rest @ ..] => {
            let inner_records = build_nested(rest, value);
            let subtable = Value::Pair(cons_cell(head.clone(), inner_records));
            Value::Pair(cons_cell(subtable, Value::Nil))
        }
    }
}

#[test]
fn ex_3_25() {
    assert_eq!(ex_3_25::ex_3_25(), (Some(1), Some(2), None));

    // A longer path through built subtables reads back the same, and a
    // stored value can be replaced through the same path.
    let root = cons_cell(Value::sym("*table*"), Value::Nil);
    insert_path(
        &[Value::sym("x"), Value::sym("y"), Value::sym("z")],
        Value::int(9),
        &root,
    );
    assert_eq!(
        value_int(lookup_path(
            &[Value::sym("x"), Value::sym("y"), Value::sym("z")],
            &root
        )),
        Some(9)
    );
    insert_path(&[Value::sym("x"), Value::sym("y")], Value::int(5), &root);
    assert_eq!(
        value_int(lookup_path(&[Value::sym("x"), Value::sym("y")], &root)),
        Some(5)
    );
    assert_eq!(
        value_int(lookup_path(
            &[Value::sym("x"), Value::sym("y"), Value::sym("z")],
            &root
        )),
        None
    );
}
