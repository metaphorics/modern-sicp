// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Sections 4.4.4.3 and 4.4.4.4: the matcher and the unifier, with the
//! book's worked examples. Matching unifies a pattern against ground
//! data; unification extends one substitution binding variables on
//! either side, chasing chains on every lookup. The occurs check
//! refuses to bind a variable inside its own value.

pub mod common;

use ch04::sec_4_4::{Substitution, unify};
use common::{atom, instantiate, list, resolve, var};
use sicp_runtime::host::query::Term;

fn datum(words: &[&str]) -> Term {
    let parts: Vec<Term> = words.iter().map(|word| atom(word)).collect();
    list(&parts)
}

fn main() {
    let empty = Substitution::new();

    let pattern = list(&[var("x"), atom("c"), var("x")]);
    let target = list(&[datum(&["a", "b"]), atom("c"), datum(&["a", "b"])]);
    let frame = unify(&pattern, &target, &empty).expect("matches");
    assert_eq!(resolve(&frame, "x"), datum(&["a", "b"]));
    assert_eq!(instantiate(&pattern, &frame), target);
    let other = list(&[var("x"), atom("a"), var("y")]);
    assert!(unify(&other, &target, &empty).is_none());

    let left = list(&[var("x"), atom("a"), var("y")]);
    let right = list(&[var("y"), var("z"), atom("a")]);
    let frame = unify(&left, &right, &empty).expect("unifies");
    for name in ["x", "y", "z"] {
        assert_eq!(instantiate(&var(name), &frame), atom("a"), "{name}");
    }
    let left = list(&[var("x"), var("y"), atom("a")]);
    let right = list(&[var("x"), atom("b"), var("y")]);
    assert!(unify(&left, &right, &empty).is_none());

    let pattern = list(&[var("x"), var("x")]);
    let target = list(&[
        list(&[atom("a"), var("y"), atom("c")]),
        list(&[atom("a"), atom("b"), var("z")]),
    ]);
    let frame = unify(&pattern, &target, &empty).expect("unifies");
    assert_eq!(
        frame.get("x"),
        Some(&list(&[atom("a"), var("y"), atom("c")]))
    );
    assert_eq!(instantiate(&var("x"), &frame), datum(&["a", "b", "c"]));

    let left = list(&[var("x"), atom("a")]);
    let right = list(&[list(&[atom("b"), var("y")]), var("z")]);
    let frame = unify(&left, &right, &empty).expect("unifies");
    assert_eq!(frame.get("x"), Some(&list(&[atom("b"), var("y")])));
    assert_eq!(instantiate(&var("z"), &frame), atom("a"));

    let cyclic = Term::Pair(Box::new(var("x")), Box::new(var("y")));
    assert!(unify(&var("x"), &cyclic, &empty).is_none());
}
