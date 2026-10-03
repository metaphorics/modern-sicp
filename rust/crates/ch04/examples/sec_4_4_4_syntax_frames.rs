// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Sections 4.4.4.7 and 4.4.4.8: the query syntax and the frames.
//! A pattern variable is `Term::Variable` and the printer is the only
//! place the book's `?x` notation exists; a rule is a conclusion with
//! its conditions, and a bodyless rule's empty condition list means
//! it succeeds for every matching pair. A frame is a substitution:
//! bindings chase through chains to their final values.

pub mod common;

use ch04::sec_4_4::{Query, Rule, Substitution, extend, qeval};
use common::{atom, fact, instantiate, list, microshaft, person, render, resolve, var};

fn main() {
    assert_eq!(render(&var("x")), "?x");
    assert_eq!(render(&atom("job")), "job");

    let wheel = Rule {
        conclusion: fact("wheel", vec![var("person")]),
        conditions: vec![
            Query::Relation {
                name: String::from("supervisor"),
                arguments: vec![var("middle-manager"), var("person")],
            },
            Query::Relation {
                name: String::from("supervisor"),
                arguments: vec![var("x"), var("middle-manager")],
            },
        ],
    };
    assert_eq!(wheel.conditions.len(), 2);

    let mut database = microshaft();
    database.add_rule(Rule {
        conclusion: fact("same", vec![var("x"), var("x")]),
        conditions: Vec::new(),
    });
    let answers = qeval(
        &database,
        &Query::Relation {
            name: String::from("same"),
            arguments: vec![person(&["Bitdiddle", "Ben"]), person(&["Bitdiddle", "Ben"])],
        },
    )
    .answers;
    assert_eq!(answers.len(), 1);

    let frame = extend(&Substitution::new(), "y", atom("a"));
    let frame = extend(&frame, "x", var("y"));
    assert_eq!(resolve(&frame, "x"), atom("a"));
    assert_eq!(
        instantiate(&list(&[var("x"), atom("c"), var("x")]), &frame),
        list(&[atom("a"), atom("c"), atom("a")])
    );
    assert_eq!(resolve(&Substitution::new(), "free"), var("free"));
}
