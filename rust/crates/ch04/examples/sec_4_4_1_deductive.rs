// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Section 4.4.1: deductive information retrieval. The Microshaft
//! transcripts: simple and compound queries, rules, and the logic as
//! programs samples, each result pinned by assertion.

use ch04::sec_4_4::microshaft;

fn main() {
    let engine = microshaft();

    // The book's first query and its two answers.
    let answers = engine.answers("(job ?x (computer programmer))");
    println!("@{answers:?}");
    assert_eq!(
        answers,
        [
            "(job (Hacker Alyssa P) (computer programmer))",
            "(job (Fect Cy D) (computer programmer))",
        ]
    );

    // All the addresses: one answer per address assertion.
    assert_eq!(engine.answers("(address ?x ?y)").len(), 9);

    // Patterns without variables: the query is a yes/no question, and
    // the empty pattern matches everything.
    assert_eq!(engine.answers("(job ?x ?y)").len(), 9);

    // A rule: Ben's supervisees living near him (the book's listing).
    let with_rules = microshaft();
    with_rules.load(&[
        "(rule (lives-near ?person-1 ?person-2) \
         (and (address ?person-1 (?town . ?rest-1)) \
         (address ?person-2 (?town . ?rest-2)) \
         (not (same ?person-1 ?person-2))))",
        "(rule (same ?x ?x))",
    ]);
    let near = with_rules.answers("(lives-near ?x (Bitdiddle Ben))");
    println!("@{near:?}");
    assert_eq!(
        near,
        [
            "(lives-near (Reasoner Louis) (Bitdiddle Ben))",
            "(lives-near (Aull DeWitt) (Bitdiddle Ben))",
        ]
    );

    // The wheel rule of 4.4.1.
    with_rules.load(&["(rule (wheel ?person) \
         (and (supervisor ?middle-manager ?person) \
         (supervisor ?x ?middle-manager)))"]);
    let wheels = with_rules.answers("(wheel ?who)");
    println!("@{wheels:?}");
    assert_eq!(wheels.len(), 5);

    // Logic as programs: append-to-form answers in every direction.
    with_rules.load(&[
        "(rule (append-to-form () ?y ?y))",
        "(rule (append-to-form (?u . ?v) ?y (?u . ?z)) (append-to-form ?v ?y ?z))",
    ]);
    assert_eq!(
        with_rules.answers("(append-to-form (a b) (c d) ?z)"),
        ["(append-to-form (a b) (c d) (a b c d))"]
    );
    let all = with_rules.answers("(append-to-form ?x ?y (a b c d))");
    assert_eq!(all.len(), 5);
    assert_eq!(all[0], "(append-to-form () (a b c d) (a b c d))");
}
