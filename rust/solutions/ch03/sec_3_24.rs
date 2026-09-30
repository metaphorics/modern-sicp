// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 3.24: a table constructor whose
//! key comparison is an argument, the book's `same-key?` predicate,
//! instead of the default `equal?`. This edition makes the predicate a
//! per-table field of the section's `Table`.

use std::rc::Rc;

use ch03::sec_3_3::Table;
use sicp_runtime::Value;

mod ex_3_24 {
    use super::{Value, near_table, value_int};

    /// Exercise 3.24: a table whose key comparison is a predicate
    ///
    /// Builds a table of measurements whose keys count as the same
    /// when they lie within 0.1 of each other, stores under 100.0,
    /// and looks up with 100.05 and with 200.0.
    #[must_use]
    pub fn ex_3_24() -> (Option<i128>, Option<i128>) {
        let table = near_table(0.1);
        table.insert(Value::real(100.0), Value::int(1));
        table.insert(Value::real(200.0), Value::int(2));

        let near = table.lookup(&Value::real(100.05));
        let far = table.lookup(&Value::real(150.0));
        (value_int(near), value_int(far))
    }
}

/// A table whose records are
/// matched by the given predicate.
#[must_use]
pub fn near_table(tolerance: f64) -> Table {
    Table::with_same_key(Rc::new(move |a, b| match (value_f64(a), value_f64(b)) {
        (Some(x), Some(y)) => (x - y).abs() < tolerance,
        _ => a == b,
    }))
}

#[expect(
    clippy::cast_precision_loss,
    reason = "measurement keys are small magnitudes, so this widening cast keeps their value exactly for the tolerances compared here"
)]
fn value_f64(v: &Value) -> Option<f64> {
    match v {
        Value::Int(n) => Some(*n as f64),
        Value::Real(x) => Some(*x),
        _ => None,
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

#[test]
fn ex_3_24() {
    // 100.05 falls within the tolerance of 100.0, 150.0 does not.
    assert_eq!(ex_3_24::ex_3_24(), (Some(1), None));

    // An insertion under a near key replaces the record, exactly as the
    // book's insert! does when assoc finds a record.
    let table = near_table(0.1);
    table.insert(Value::real(100.0), Value::int(1));
    table.insert(Value::real(100.05), Value::int(9));
    assert_eq!(table.records().len(), 1);
    assert_eq!(value_int(table.lookup(&Value::real(100.0))), Some(9));

    // The default table still demands equal keys.
    let plain = Table::new();
    plain.insert(Value::real(100.0), Value::int(1));
    assert_eq!(plain.lookup(&Value::real(100.05)), None);
}
