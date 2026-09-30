// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Sections 4.4.4.5 and 4.4.4.6: the indexed, chronological data base
//! and the stream operations. Assertions stay in filing order, which
//! is the production order of the answer stream; a variable-led
//! pattern falls back to the whole data base. Delayed streams
//! interleave fairly while the undelayed engine concatenates
//! depth-first, so the exercises can compare which strategy yields
//! the first answers within equal fuel.

pub mod common;

use ch04::sec_4_4::{
    Query, qeval, qeval_bounded, qeval_prefix, qeval_undelayed_bounded, query_append,
    reference_answers,
};
use common::{microshaft, var};
use sicp_runtime::host::query::Term;

fn relation(name: &str, arguments: Vec<Term>) -> Query {
    Query::Relation {
        name: name.to_owned(),
        arguments,
    }
}

/// The relation name heading one assertion term, if any.
fn head_name(term: &Term) -> Option<&str> {
    match term {
        Term::Pair(head, _) => match head.as_ref() {
            Term::Atom(name) => Some(name),
            _ => None,
        },
        _ => None,
    }
}

fn main() {
    let database = microshaft();

    // The full Microshaft data base: nine people times address, job,
    // and salary, plus eight supervisor and four `can-do-job`
    // assertions, filed in the book's order.
    assert_eq!(database.assertions.len(), 39);
    // The book's opening assertion is Ben's address.
    assert_eq!(head_name(&database.assertions[0]), Some("address"));
    // The supervisor bucket holds eight assertions; a variable-led
    // pattern falls back to the whole chronological store.
    let supervisors = database
        .assertions
        .iter()
        .filter(|term| head_name(term) == Some("supervisor"))
        .count();
    assert_eq!(supervisors, 8);

    // The stream operations over frames: `or` interleaves fairly. The
    // append rule answers all five splits of a four-element list in
    // production order, and the reference model agrees.
    let (append_db, append_query) = query_append();
    let answers = qeval(&append_db, &append_query).answers;
    assert_eq!(answers.len(), 5);
    assert_eq!(reference_answers(&append_db, &append_query), answers);

    // Bounded observation: two units of fuel yield the first two
    // splits without exhausting the stream on either engine.
    let delayed = qeval_bounded(&append_db, &append_query, 2);
    assert_eq!(delayed.answers.len(), 2);
    assert!(!delayed.exhausted);
    let undelayed = qeval_undelayed_bounded(&append_db, &append_query, 2);
    assert_eq!(undelayed.answers.len(), 2);

    // Generous fuel exhausts the finite rule set on both engines with
    // the same five answers.
    let delayed = qeval_bounded(&append_db, &append_query, 64);
    assert!(delayed.exhausted);
    assert_eq!(delayed.answers.len(), 5);
    let undelayed = qeval_undelayed_bounded(&append_db, &append_query, 64);
    assert!(undelayed.exhausted);
    assert_eq!(undelayed.answers, delayed.answers);

    // Prefix discipline on the full data base: the first two
    // supervisor pairs without running the whole stream.
    let prefix = qeval_prefix(
        &database,
        &relation("supervisor", vec![var("x"), var("y")]),
        2,
    );
    assert_eq!(prefix.answers.len(), 2);
}
