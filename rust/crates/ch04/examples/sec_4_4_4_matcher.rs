// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Sections 4.4.4.3 and 4.4.4.4: the matcher and the unifier, with the
//! book's worked examples.

use ch04::sec_4_4::{Frame, pattern_match, query_syntax_process, unify_match};
use sicp_runtime::read;

fn q(text: &str) -> sicp_runtime::Value {
    query_syntax_process(&read(text).expect("parses"))
}

fn main() {
    // ((a b) c (a b)) matches (?x c ?x) with ?x = (a b).
    let frame =
        pattern_match(&q("(?x c ?x)"), &q("((a b) c (a b))"), &Frame::new()).expect("matches");
    assert_eq!(
        ch04::sec_4_4::instantiate_query(&q("(?x c ?x)"), &frame),
        q("((a b) c (a b))")
    );
    // (?x a ?y) does not match.
    assert!(pattern_match(&q("(?x a ?y)"), &q("((a b) c (a b))"), &Frame::new()).is_none());

    // Unification: (?x a ?y) against (?y ?z a) binds all three to a.
    let frame = unify_match(&q("(?x a ?y)"), &q("(?y ?z a)"), &Frame::new()).expect("unifies");
    // The frame stores chains (?x -> ?y -> a); instantiation chases them.
    for var in ["?x", "?y", "?z"] {
        assert_eq!(
            ch04::sec_4_4::instantiate_query(&q(var), &frame),
            q("a"),
            "{var}"
        );
    }
    // (?x ?y a) against (?x b ?y) fails.
    assert!(unify_match(&q("(?x ?y a)"), &q("(?x b ?y)"), &Frame::new()).is_none());
    // (?x ?x) against ((a ?y c) (a b ?z)) forces ?x = (a b c): the
    // frame stores the chain and instantiation chases it.
    let frame =
        unify_match(&q("(?x ?x)"), &q("((a ?y c) (a b ?z))"), &Frame::new()).expect("unifies");
    assert_eq!(frame.binding_in_frame(&q("?x")), Some(q("(a ?y c)")));
    assert_eq!(
        ch04::sec_4_4::instantiate_query(&q("?x"), &frame),
        q("(a b c)")
    );
    // A partial binding: ?x is pinned to a pattern still holding ?y
    // (the book's ((b ?y) ?z) example -- the unifier stores the pattern).
    let frame = unify_match(&q("(?x a)"), &q("((b ?y) ?z)"), &Frame::new()).expect("unifies");
    assert_eq!(frame.binding_in_frame(&q("?x")), Some(q("(b ?y)")));
    assert_eq!(ch04::sec_4_4::instantiate_query(&q("?z"), &frame), q("a"));
}
