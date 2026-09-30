// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.70: assertion construction
//! binds its value before the database stores it.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, qeval};
use support::{atom, fact, relation};

fn add_assertion(database: &mut Database, name: &str, value: i64) {
    let assertion = fact(
        name,
        vec![
            atom("record"),
            sicp_runtime::host::query::Term::Integer(value),
        ],
    );
    database.assert(assertion);
}

#[test]
fn ex_4_70() {
    let mut database = Database::new();
    add_assertion(&mut database, "record", 1);
    add_assertion(&mut database, "record", 2);
    let outcome = qeval(
        &database,
        &relation(
            "record",
            vec![
                atom("record"),
                sicp_runtime::host::query::Term::Variable("value".to_owned()),
            ],
        ),
    );
    assert_eq!(outcome.answers.len(), 2);
}
