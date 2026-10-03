// SPDX-License-Identifier: GPL-3.0-only

//! The query data language of section 4.4 (grammar §7): terms,
//! queries, and substitutions as Rust values. Query operators are
//! constructors, never host syntax, and answer order is always the
//! order of an explicit `Vec`, never `HashMap` iteration.

use std::collections::HashMap;

/// One term of the query language.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    /// A pattern variable.
    Variable(String),
    /// An integer datum.
    Integer(i64),
    /// A text datum.
    Text(String),
    /// A symbolic atom.
    Atom(String),
    /// A pair: the query language's cons cell.
    Pair(Box<Term>, Box<Term>),
    /// The empty list.
    Empty,
}

/// One query.
#[derive(Debug, Clone)]
pub enum Query {
    /// Unify two terms.
    Unify(Term, Term),
    /// A relation application.
    Relation {
        /// The relation's name.
        name: String,
        /// The arguments.
        arguments: Vec<Term>,
    },
    /// Conjunction of sub-queries.
    And(Vec<Query>),
    /// Disjunction of sub-queries.
    Or(Vec<Query>),
    /// Negation as failure of one sub-query.
    Not(Box<Query>),
    /// The unique answers of one sub-query.
    Unique(Box<Query>),
    /// A host predicate over bound terms (the `lisp-value` lesson).
    Value(Predicate, Vec<Term>),
    /// The answers unique in the listed variables (exercise 4.74).
    UniqueBy(Vec<String>, Box<Query>),
}

/// One host-side predicate: the `lisp-value` discipline as explicit
/// data (grammar §4.4). In query context a pattern variable resolves
/// through the substitution (unbound fails the frame); in the named
/// search experiment it resolves through the trail bindings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Predicate {
    /// `a == b`.
    Eq(Term, Term),
    /// `a != b`.
    Ne(Term, Term),
    /// `a < b`.
    Lt(Term, Term),
    /// `a <= b`.
    Le(Term, Term),
    /// `a > b`.
    Gt(Term, Term),
    /// `a >= b`.
    Ge(Term, Term),
    /// Lexicographic order of the terms' string renderings.
    TextLt(Term, Term),
    /// `sum(terms) == bound`.
    SumEq(Vec<Term>, i64),
    /// The term resolves to a value.
    Bound(Term),
    /// Disjunction of sub-predicates.
    Or(Vec<Predicate>),
    /// `a - b == c - d` over four binding names.
    DiffEq(String, String, String, String),
    /// `a * a + b * b == c * c` over three binding names.
    Pythagorean(String, String, String),
}

/// A substitution: pattern variables to terms. The map is an index
/// only; answer order never comes from its iteration.
pub type Substitution = HashMap<String, Term>;

/// Extends a substitution with one binding.
#[must_use]
pub fn extend(substitution: &Substitution, name: &str, term: Term) -> Substitution {
    let mut next = substitution.clone();
    next.insert(name.to_owned(), term);
    next
}
