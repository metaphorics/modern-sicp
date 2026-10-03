// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.59: meeting rules combine a
//! time slot with the meeting type.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_4::{Database, qeval};
use support::{answer_text, atom, fact, list, relation, rule, var};

fn meetings() -> Database {
    let mut database = Database::new();
    database.assert(fact(
        "meeting",
        vec![atom("accounting"), list(vec![atom("Monday"), atom("9am")])],
    ));
    database.assert(fact(
        "meeting",
        vec![atom("computer"), list(vec![atom("Wednesday"), atom("3pm")])],
    ));
    database.add_rule(rule(
        fact("meeting_time", vec![var("day"), var("time")]),
        vec![relation(
            "meeting",
            vec![var("type"), list(vec![var("day"), var("time")])],
        )],
    ));
    database
}

#[test]
fn ex_4_59() {
    let outcome = qeval(
        &meetings(),
        &relation("meeting_time", vec![var("day"), var("time")]),
    );
    assert_eq!(outcome.answers.len(), 2);
    assert_eq!(answer_text(&outcome.answers[0], "day"), "Monday");
}
