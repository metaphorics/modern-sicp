// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.60: the `lives-near` pairs. The
//! book's rule answers each same-town pair twice, once per order; the
//! edition's deduplicated rule keeps the pair whose first member sorts
//! before the second under a host `name<` predicate applied with
//! `lisp-value`.

use std::rc::Rc;

use ch04::sec_4_4::{Engine, microshaft};
use sicp_runtime::{Handler, SchemeError, Value, print_value};

mod ex_4_60 {
    //! Exercise 4.60: the duplicated pairs and the once-only rule.

    use super::*;

    /// The book's `lives-near` rule, which lists each pair twice.
    pub fn duplicated() -> Engine {
        let engine = microshaft();
        engine.load(&[
            "(rule (lives-near ?person-1 ?person-2) \
             (and (address ?person-1 (?town . ?rest-1)) \
             (address ?person-2 (?town . ?rest-2)) \
             (not (same ?person-1 ?person-2))))",
            "(rule (same ?x ?x))",
        ]);
        engine
    }

    /// The deduplicated rule: person-1 must print before person-2.
    pub fn deduplicated() -> Engine {
        // A fresh engine without the duplicated rule, plus the ordered
        // one and the host comparison it applies.
        let ordered = microshaft();
        ordered.load(&["(rule (same ?x ?x))"]);
        ordered.install_predicate("name<", name_less_than());
        ordered.load(&["(rule (lives-near ?person-1 ?person-2) \
             (and (address ?person-1 (?town . ?rest-1)) \
             (address ?person-2 (?town . ?rest-2)) \
             (lisp-value name< ?person-1 ?person-2)))"]);
        ordered
    }

    /// The host predicate `lisp-value` applies: printed-name order.
    fn name_less_than() -> Handler {
        Rc::new(|args: &[Value]| match args {
            [a, b] => Ok(Value::boolean(print_value(a) < print_value(b))),
            _ => Err(SchemeError::TypeMismatch("name< wants two people".into())),
        })
    }
}

#[test]
fn ex_4_60() {
    // The book's rule lists every pair twice, once per order.
    assert_eq!(
        ex_4_60::duplicated()
            .answers("(lives-near ?person-1 ?person-2)")
            .len(),
        8,
        "four same-town pairs, each in both orders"
    );
    assert_eq!(
        ex_4_60::duplicated().answers("(lives-near ?person-1 ?person-2)"),
        [
            "(lives-near (Bitdiddle Ben) (Reasoner Louis))",
            "(lives-near (Fect Cy D) (Hacker Alyssa P))",
            "(lives-near (Bitdiddle Ben) (Aull DeWitt))",
            "(lives-near (Hacker Alyssa P) (Fect Cy D))",
            "(lives-near (Reasoner Louis) (Bitdiddle Ben))",
            "(lives-near (Reasoner Louis) (Aull DeWitt))",
            "(lives-near (Aull DeWitt) (Bitdiddle Ben))",
            "(lives-near (Aull DeWitt) (Reasoner Louis))",
        ]
    );
    // The ordered rule lists each pair once.
    assert_eq!(
        ex_4_60::deduplicated().answers("(lives-near ?person-1 ?person-2)"),
        [
            "(lives-near (Bitdiddle Ben) (Reasoner Louis))",
            "(lives-near (Fect Cy D) (Hacker Alyssa P))",
            "(lives-near (Aull DeWitt) (Bitdiddle Ben))",
            "(lives-near (Aull DeWitt) (Reasoner Louis))",
        ]
    );
}
