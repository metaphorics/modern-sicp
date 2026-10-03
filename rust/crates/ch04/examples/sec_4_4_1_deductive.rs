// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Section 4.4.1: deductive information retrieval. The Microshaft
//! transcripts: simple and compound queries, rules, and the logic as
//! programs samples, each result pinned by assertion.

pub mod common;

use ch04::sec_4_4::{Query, Rule, Term, qeval};
use common::{atom, fact, list, microshaft, person, render_query, resolve, var};

fn relation(name: &str, arguments: Vec<Term>) -> Query {
    Query::Relation {
        name: name.to_owned(),
        arguments,
    }
}

fn named(answers: &[ch04::sec_4_4::Substitution], name: &str) -> Vec<Term> {
    answers.iter().map(|answer| resolve(answer, name)).collect()
}

fn main() {
    let database = microshaft();
    simple_query(&database);
    assertion_counts(&database);

    // The rules of the book's listing, accumulated over one data
    // base: Ben's neighbors, the wheel rule, and `append-to-form`.
    let mut ruled = microshaft();
    lives_near_rule(&mut ruled);
    wheel_rule(&mut ruled);
    append_to_form(&mut ruled);
}

/// The book's first query: the job pattern naming the computer
/// programmers, with its two answers.
fn simple_query(database: &ch04::sec_4_4::Database) {
    let query = relation(
        "job",
        vec![var("person"), list(&[atom("computer"), atom("programmer")])],
    );
    let answers = qeval(database, &query).answers;
    println!(
        "@{:?}",
        answers
            .iter()
            .map(|a| render_query("job", &[var("person")], a))
            .collect::<Vec<String>>()
    );
    assert_eq!(
        named(&answers, "person"),
        [
            person(&["Hacker", "Alyssa", "P"]),
            person(&["Fect", "Cy", "D"])
        ]
    );
}

/// One answer per assertion: the addresses and the jobs of the
/// whole data base.
fn assertion_counts(database: &ch04::sec_4_4::Database) {
    let addresses = qeval(
        database,
        &relation("address", vec![var("where"), var("what")]),
    )
    .answers;
    let jobs = qeval(database, &relation("job", vec![var("who"), var("what")])).answers;
    assert_eq!(addresses.len(), 9);
    assert_eq!(jobs.len(), 9);
}

/// Ben's neighbors: the book's `lives-near` rule with the unguarded
/// `same` rule it needs, and the two answers.
fn lives_near_rule(ruled: &mut ch04::sec_4_4::Database) {
    // The `same` rule has no conditions, so it succeeds for every
    // matching pair.
    ruled.add_rule(Rule {
        conclusion: fact("same", vec![var("x"), var("x")]),
        conditions: Vec::new(),
    });
    let town = Term::Pair(Box::new(var("town")), Box::new(var("rest-1")));
    let other_town = Term::Pair(Box::new(var("town")), Box::new(var("rest-2")));
    ruled.add_rule(Rule {
        conclusion: fact("lives-near", vec![var("person-1"), var("person-2")]),
        conditions: vec![
            relation("address", vec![var("person-1"), town]),
            relation("address", vec![var("person-2"), other_town]),
            Query::Not(Box::new(relation(
                "same",
                vec![var("person-1"), var("person-2")],
            ))),
        ],
    });
    let query = relation(
        "lives-near",
        vec![var("near"), person(&["Bitdiddle", "Ben"])],
    );
    let answers = qeval(ruled, &query).answers;
    println!(
        "@{:?}",
        answers
            .iter()
            .map(|a| render_query("lives-near", &[var("near")], a))
            .collect::<Vec<String>>()
    );
    assert_eq!(
        named(&answers, "near"),
        [person(&["Reasoner", "Louis"]), person(&["Aull", "DeWitt"])]
    );
}

/// The wheel rule of 4.4.1: anyone who supervises a supervisor.
fn wheel_rule(ruled: &mut ch04::sec_4_4::Database) {
    ruled.add_rule(Rule {
        conclusion: fact("wheel", vec![var("person")]),
        conditions: vec![
            relation("supervisor", vec![var("middle-manager"), var("person")]),
            relation("supervisor", vec![var("x"), var("middle-manager")]),
        ],
    });
    let query = relation("wheel", vec![var("who")]);
    let answers = qeval(ruled, &query).answers;
    let wheels = named(&answers, "who");
    println!(
        "@{:?}",
        answers
            .iter()
            .map(|a| render_query("wheel", &[var("who")], a))
            .collect::<Vec<String>>()
    );
    // => 5 wheels: four derivations name the big wheel, one names Ben.
    assert_eq!(wheels.len(), 5);
    assert_eq!(
        wheels
            .iter()
            .filter(|who| **who == person(&["Warbucks", "Oliver"]))
            .count(),
        4
    );
    assert_eq!(
        wheels
            .iter()
            .filter(|who| **who == person(&["Bitdiddle", "Ben"]))
            .count(),
        1
    );
}

/// Logic as programs: `append-to-form` answers in every direction.
fn append_to_form(ruled: &mut ch04::sec_4_4::Database) {
    let steps = Rule {
        conclusion: fact(
            "append-to-form",
            vec![
                Term::Pair(Box::new(var("u")), Box::new(var("v"))),
                var("y"),
                Term::Pair(Box::new(var("u")), Box::new(var("z"))),
            ],
        ),
        conditions: vec![relation(
            "append-to-form",
            vec![var("v"), var("y"), var("z")],
        )],
    };
    ruled.add_rule(Rule {
        conclusion: fact("append-to-form", vec![Term::Empty, var("y"), var("y")]),
        conditions: Vec::new(),
    });
    ruled.add_rule(steps);

    let query = relation(
        "append-to-form",
        vec![
            list(&[atom("a"), atom("b")]),
            list(&[atom("c"), atom("d")]),
            var("z"),
        ],
    );
    let answers = qeval(ruled, &query).answers;
    assert_eq!(
        named(&answers, "z"),
        [list(&[atom("a"), atom("b"), atom("c"), atom("d")])]
    );

    let query = relation(
        "append-to-form",
        vec![
            var("x"),
            var("y"),
            list(&[atom("a"), atom("b"), atom("c"), atom("d")]),
        ],
    );
    let answers = qeval(ruled, &query).answers;
    println!(
        "@{:?}",
        answers
            .iter()
            .map(|a| render_query("append-to-form", &[var("x"), var("y")], a))
            .collect::<Vec<String>>()
    );
    // The five splits of a four-element list.
    assert_eq!(answers.len(), 5);
    assert_eq!(named(&answers, "x")[0], Term::Empty);
    let mut lengths: Vec<usize> = named(&answers, "x")
        .iter()
        .map(|term| {
            let mut count = 0;
            let mut cursor = term;
            while let Term::Pair(_, rest) = cursor {
                count += 1;
                cursor = rest;
            }
            count
        })
        .collect();
    lengths.sort_unstable();
    assert_eq!(lengths, [0, 1, 2, 3, 4]);
}
