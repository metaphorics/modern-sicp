// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Shared constructors, transcript rendering, and the Microshaft
//! fixture for the section 4.4 query examples. The query language is
//! data: a pattern variable is `Term::Variable` whose name carries no
//! notation, and the printer is the only place the book's `?x` form
//! exists.

use ch04::sec_4_4::{Database, Substitution, Term};

/// A symbolic datum.
#[must_use]
pub fn atom(name: &str) -> Term {
    Term::Atom(name.to_owned())
}

/// A pattern variable: the notation's `?` is the printer's business.
#[must_use]
pub fn var(name: &str) -> Term {
    Term::Variable(name.to_owned())
}

/// A right-nested pair list ending in the empty list.
#[must_use]
pub fn list(items: &[Term]) -> Term {
    items.iter().rev().fold(Term::Empty, |rest, item| {
        Term::Pair(Box::new(item.clone()), Box::new(rest))
    })
}

/// A Microshaft person name: the book's `(First Middle-Parts... Last)`
/// names are lists of name-part atoms, one atom per written part
/// (the book's `Alyssa P` is the two-element list of `Alyssa` and
/// `P`), so a name can carry any number of parts.
#[must_use]
pub fn person(name_parts: &[&str]) -> Term {
    let parts: Vec<Term> = name_parts.iter().map(|part| atom(part)).collect();
    list(&parts)
}

/// One assertion of the form `(name arg ...)`.
#[must_use]
pub fn fact(name: &str, arguments: Vec<Term>) -> Term {
    let mut items = vec![atom(name)];
    items.extend(arguments);
    list(&items)
}

/// The frame's final view of one variable, chased through bindings.
#[must_use]
pub fn resolve(frame: &Substitution, name: &str) -> Term {
    let mut term = var(name);
    while let Term::Variable(chased) = &term {
        match frame.get(chased) {
            Some(bound) => term = bound.clone(),
            None => break,
        }
    }
    term
}

/// Instantiates one term through the frame.
#[must_use]
pub fn instantiate(term: &Term, frame: &Substitution) -> Term {
    match term {
        Term::Pair(first, rest) => Term::Pair(
            Box::new(instantiate(first, frame)),
            Box::new(instantiate(rest, frame)),
        ),
        Term::Variable(_) => resolve(frame, variable_name(term)),
        other => other.clone(),
    }
}

/// The name of a variable term; every other term has none to carry.
fn variable_name(term: &Term) -> &str {
    match term {
        Term::Variable(name) => name,
        _ => "",
    }
}

/// Renders one term in the book's notation.
#[must_use]
pub fn render(term: &Term) -> String {
    match term {
        Term::Variable(name) => format!("?{name}"),
        Term::Integer(value) => value.to_string(),
        Term::Text(text) => format!("\"{text}\""),
        Term::Atom(name) => name.clone(),
        Term::Empty => String::from("()"),
        Term::Pair(..) => render_list(term),
    }
}

fn render_list(term: &Term) -> String {
    let mut parts = Vec::new();
    let mut cursor = term;
    while let Term::Pair(first, rest) = cursor {
        parts.push(render(first));
        cursor = rest;
    }
    match cursor {
        Term::Empty => format!("({})", parts.join(" ")),
        tail => format!("({} . {})", parts.join(" "), render(tail)),
    }
}

/// Renders one answer as the book's instantiated query.
#[must_use]
pub fn render_query(name: &str, arguments: &[Term], frame: &Substitution) -> String {
    let parts: Vec<String> = arguments
        .iter()
        .map(|argument| render(&instantiate(argument, frame)))
        .collect();
    format!("({name} {})", parts.join(" "))
}

/// The Microshaft data base of section 4.4.1: nine people and the
/// four `can-do-job` rules of the book's listing, asserted in the
/// book's order.
#[must_use]
pub fn microshaft() -> Database {
    let mut database = Database::new();
    for (who, [place, work, pay]) in roster() {
        database.assert(fact("address", vec![who.clone(), place]));
        database.assert(fact("job", vec![who.clone(), work]));
        database.assert(fact("salary", vec![who, pay]));
    }
    for (worker, chief) in [
        (
            person(&["Hacker", "Alyssa", "P"]),
            person(&["Bitdiddle", "Ben"]),
        ),
        (person(&["Fect", "Cy", "D"]), person(&["Bitdiddle", "Ben"])),
        (
            person(&["Tweakit", "Lem", "E"]),
            person(&["Bitdiddle", "Ben"]),
        ),
        (
            person(&["Reasoner", "Louis"]),
            person(&["Hacker", "Alyssa", "P"]),
        ),
        (
            person(&["Bitdiddle", "Ben"]),
            person(&["Warbucks", "Oliver"]),
        ),
        (
            person(&["Scrooge", "Eben"]),
            person(&["Warbucks", "Oliver"]),
        ),
        (
            person(&["Cratchet", "Robert"]),
            person(&["Scrooge", "Eben"]),
        ),
        (person(&["Aull", "DeWitt"]), person(&["Warbucks", "Oliver"])),
    ] {
        database.assert(fact("supervisor", vec![worker, chief]));
    }
    for (from, to) in [
        (
            job("computer", &["wizard"]),
            job("computer", &["programmer"]),
        ),
        (
            job("computer", &["wizard"]),
            job("computer", &["technician"]),
        ),
        (
            job("computer", &["programmer"]),
            job("computer", &["programmer", "trainee"]),
        ),
        (
            job("administration", &["secretary"]),
            job("administration", &["big", "wheel"]),
        ),
    ] {
        database.assert(fact("can-do-job", vec![from, to]));
    }
    database
}

/// One `(street, town, number)` address of the book's listing; a
/// house the book shows without a street number carries none.
fn address(street: &[&str], town: &str, number: Option<i64>) -> Term {
    let street_parts: Vec<Term> = street.iter().map(|part| atom(part)).collect();
    let mut parts = vec![atom(town), list(&street_parts)];
    if let Some(number) = number {
        parts.push(Term::Integer(number));
    }
    list(&parts)
}

/// One `(division, title parts...)` job of the book's listing.
fn job(division: &str, title: &[&str]) -> Term {
    let mut parts = vec![atom(division)];
    parts.extend(title.iter().map(|part| atom(part)));
    list(&parts)
}

/// The nine `(person, [address, job, salary])` records of the book's
/// listing, in the book's order.
fn roster() -> Vec<(Term, [Term; 3])> {
    let ben = person(&["Bitdiddle", "Ben"]);
    let alyssa = person(&["Hacker", "Alyssa", "P"]);
    let cy = person(&["Fect", "Cy", "D"]);
    let lem = person(&["Tweakit", "Lem", "E"]);
    let louis = person(&["Reasoner", "Louis"]);
    let oliver = person(&["Warbucks", "Oliver"]);
    let eben = person(&["Scrooge", "Eben"]);
    let robert = person(&["Cratchet", "Robert"]);
    let dewitt = person(&["Aull", "DeWitt"]);
    vec![
        (
            ben.clone(),
            [
                address(&["Ridge", "Road"], "Slumerville", Some(10)),
                job("computer", &["wizard"]),
                Term::Integer(60000),
            ],
        ),
        (
            alyssa.clone(),
            [
                address(&["Mass", "Ave"], "Cambridge", Some(78)),
                job("computer", &["programmer"]),
                Term::Integer(40000),
            ],
        ),
        (
            cy.clone(),
            [
                address(&["Ames", "Street"], "Cambridge", Some(3)),
                job("computer", &["programmer"]),
                Term::Integer(35000),
            ],
        ),
        (
            lem.clone(),
            [
                address(&["Bay", "State", "Road"], "Boston", Some(22)),
                job("computer", &["technician"]),
                Term::Integer(25000),
            ],
        ),
        (
            louis.clone(),
            [
                address(&["Pine", "Tree", "Road"], "Slumerville", Some(80)),
                job("computer", &["programmer", "trainee"]),
                Term::Integer(30000),
            ],
        ),
        (
            oliver.clone(),
            [
                address(&["Top", "Heap", "Road"], "Swellesley", None),
                job("administration", &["big", "wheel"]),
                Term::Integer(150_000),
            ],
        ),
        (
            eben.clone(),
            [
                address(&["Shady", "Lane"], "Weston", Some(10)),
                job("accounting", &["chief", "accountant"]),
                Term::Integer(75000),
            ],
        ),
        (
            robert.clone(),
            [
                address(&["N", "Harvard", "Street"], "Allston", Some(16)),
                job("accounting", &["scrivener"]),
                Term::Integer(18000),
            ],
        ),
        (
            dewitt.clone(),
            [
                address(&["Onion", "Square"], "Slumerville", Some(5)),
                job("administration", &["secretary"]),
                Term::Integer(25000),
            ],
        ),
    ]
}
