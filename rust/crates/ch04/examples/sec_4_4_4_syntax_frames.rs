// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.4

//! Sections 4.4.4.7 and 4.4.4.8: the query syntax procedures and the
//! frames, with the book's syntax examples.

use ch04::sec_4_4::{
    conclusion, contract_question_mark, is_rule, is_var, make_new_variable, query_syntax_process,
};

fn main() {
    // The internal variable format: (? x) for ?x.
    let processed = query_syntax_process(&read("(job ?x ?y)").expect("parses"));
    assert!(is_var(&cadr(&processed)));
    assert_eq!(
        contract_question_mark(&cadr(&processed)),
        sicp_runtime::Value::sym("?x")
    );

    // Renaming: (? x) becomes (? id x) and prints as ?x-id.
    let renamed = make_new_variable(&cadr(&processed), 7);
    assert_eq!(
        contract_question_mark(&renamed),
        sicp_runtime::Value::sym("?x-7")
    );

    // Rules: shape, conclusion, body.
    let rule = sicp_runtime::read("(rule (wheel ?p) (and (a) (b)))").expect("parses");
    assert!(is_rule(&rule));
    assert_eq!(conclusion(&rule), read("(wheel ?p)").expect("parses"));
    assert!(!ch04::sec_4_4::rule_body(&rule).is_nil());

    // A bodyless rule's body is (always-true).
    let same = sicp_runtime::read("(rule (same ?x ?x))").expect("parses");
    assert!(!ch04::sec_4_4::rule_body(&same).is_nil());
}

fn cadr(v: &sicp_runtime::Value) -> sicp_runtime::Value {
    ch04::sec_4_4::split_list(&ch04::sec_4_4::split_list(v).1).0
}

use sicp_runtime::read;
