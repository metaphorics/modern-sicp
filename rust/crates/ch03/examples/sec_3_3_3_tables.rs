// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 3.3.3

//! Section 3.3.3: tables — the one-dimensional table over mutable
//! records, the two-dimensional table of Figure 3.23, and the local
//! table whose operations the chapter's `put` and `get` use.

use ch03::sec_3_3::{Table, insert_2d, lookup_2d};
use sicp_runtime::Value;

fn main() {
    // The one-dimensional table of Figure 3.21: a `*table*` header over
    // records, holding a: 1, b: 2, c: 3.
    let table = Table::new();
    table.insert(Value::sym("a"), Value::int(1));
    table.insert(Value::sym("b"), Value::int(2));
    table.insert(Value::sym("c"), Value::int(3));

    let answer = table.lookup(&Value::sym("b"));
    let shown = answer.clone().map_or("#f".into(), |v| v.to_string());
    println!("{shown}");
    // => 2
    assert_eq!(answer, Some(Value::int(2)));

    // A miss answers false in the book, `None` here.
    let answer = table.lookup(&Value::sym("d"));
    let shown = answer.clone().map_or("#f".into(), |v| v.to_string());
    println!("{shown}");
    // => #f
    assert_eq!(answer, None);

    // insert! replaces the value of an existing record in place.
    table.insert(Value::sym("b"), Value::int(20));
    let answer = table.lookup(&Value::sym("b"));
    let shown = answer.clone().map_or("#f".into(), |v| v.to_string());
    println!("{shown}");
    // => 20
    assert_eq!(answer, Some(Value::int(20)));

    // The two-dimensional table of Figure 3.23: letters a: 97, b: 98;
    // math +: 43, -: 45, *: 42.
    let t2 = Table::new();
    insert_2d(Value::sym("letters"), Value::sym("a"), Value::int(97), &t2);
    insert_2d(Value::sym("letters"), Value::sym("b"), Value::int(98), &t2);
    insert_2d(Value::sym("math"), Value::sym("+"), Value::int(43), &t2);
    insert_2d(Value::sym("math"), Value::sym("-"), Value::int(45), &t2);
    insert_2d(Value::sym("math"), Value::sym("*"), Value::int(42), &t2);

    let answer = lookup_2d(&Value::sym("letters"), &Value::sym("b"), &t2);
    let shown = answer.clone().map_or("#f".into(), |v| v.to_string());
    println!("{shown}");
    // => 98
    assert_eq!(answer, Some(Value::int(98)));
    let answer = lookup_2d(&Value::sym("math"), &Value::sym("*"), &t2);
    let shown = answer.clone().map_or("#f".into(), |v| v.to_string());
    println!("{shown}");
    // => 42
    assert_eq!(answer, Some(Value::int(42)));
    let answer = lookup_2d(&Value::sym("math"), &Value::sym("/"), &t2);
    let shown = answer.clone().map_or("#f".into(), |v| v.to_string());
    println!("{shown}");
    // => #f
    assert_eq!(answer, None);

    // The local table of the `put`/`get` dispatch: the table object
    // keeps its records in its own state, and the two operations close
    // over it. The chapter's operation table is exactly this, keyed by
    // an operation name and a tag list.
    let operation_table = Table::new();
    let put = |key1: Value, key2: Value, value: Value| {
        insert_2d(key1, key2, value, &operation_table);
    };
    let get = |key1: Value, key2: Value| lookup_2d(&key1, &key2, &operation_table);

    put(
        Value::sym("real-part"),
        Value::sym("rectangular"),
        Value::int(0),
    );
    let answer = get(Value::sym("real-part"), Value::sym("rectangular"));
    let shown = answer.clone().map_or("#f".into(), |v| v.to_string());
    println!("{shown}");
    // => 0
    assert_eq!(answer, Some(Value::int(0)));
    let answer = get(Value::sym("real-part"), Value::sym("polar"));
    let shown = answer.clone().map_or("#f".into(), |v| v.to_string());
    println!("{shown}");
    // => #f
    assert_eq!(answer, None);
}
