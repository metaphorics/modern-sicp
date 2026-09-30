// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.5: typed cond clauses with a
//! recipient arrow and an empty-body clause.

/// Shared typed support for this exercise.
pub mod support;

enum Clause {
    Arrow {
        test: Option<i64>,
        recipient: fn(i64) -> i64,
    },
    Body {
        test: bool,
        value: i64,
    },
}

fn first_clause(clauses: &[Clause]) -> Option<i64> {
    clauses.iter().find_map(|clause| match clause {
        Clause::Arrow {
            test: Some(value),
            recipient,
        } => Some(recipient(*value)),
        Clause::Body { test: true, value } => Some(*value),
        Clause::Arrow { test: None, .. } | Clause::Body { test: false, .. } => None,
    })
}

#[test]
fn ex_4_05() {
    let clauses = [
        Clause::Body {
            test: false,
            value: 0,
        },
        Clause::Arrow {
            test: Some(2),
            recipient: |value| value * value,
        },
        Clause::Body {
            test: true,
            value: 99,
        },
    ];
    assert_eq!(first_clause(&clauses), Some(4));
}
