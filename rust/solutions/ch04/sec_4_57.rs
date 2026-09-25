// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.57: the `can-replace` rule. Two
//! people can replace each other when their jobs coincide under the
//! `can-do-job` relation and the people differ; the `same` rule is the
//! bodyless rule whose unification supplies the equality test.

use ch04::sec_4_4::{Engine, microshaft};

mod ex_4_57 {
    //! Exercise 4.57: the replacement rule and the paid-more query.

    use super::*;

    /// One engine carrying the `can-replace` rule.
    pub fn engine() -> Engine {
        let engine = microshaft();
        engine.load(&[
            "(rule (can-replace ?person-1 ?person-2) \
             (and (job ?person-1 ?job-1) \
             (job ?person-2 ?job-2) \
             (or (same ?job-1 ?job-2) (can-do-job ?job-1 ?job-2)) \
             (not (same ?person-1 ?person-2))))",
            "(rule (same ?x ?x))",
        ]);
        engine
    }
}

#[test]
fn ex_4_57() {
    // a. all replacement pairs, in rule-body enumeration order: Ben's
    // wizardry can replace every programmer and the technician, Alyssa
    // and Cy replace each other and the trainee, and Aull's secretarial
    // job can-do the big wheel.
    assert_eq!(
        ex_4_57::engine().answers("(can-replace ?p1 ?p2)"),
        [
            "(can-replace (Bitdiddle Ben) (Hacker Alyssa P))",
            "(can-replace (Bitdiddle Ben) (Fect Cy D))",
            "(can-replace (Hacker Alyssa P) (Fect Cy D))",
            "(can-replace (Bitdiddle Ben) (Tweakit Lem E))",
            "(can-replace (Fect Cy D) (Hacker Alyssa P))",
            "(can-replace (Hacker Alyssa P) (Reasoner Louis))",
            "(can-replace (Fect Cy D) (Reasoner Louis))",
            "(can-replace (Aull DeWitt) (Warbucks Oliver))",
        ]
    );
    // b. pairs where the replacement earns less than the replaced: Cy
    // Fect replaces his better-paid colleague Alyssa Hacker, and Aull
    // DeWitt replaces the far better paid big wheel.
    assert_eq!(
        ex_4_57::engine().answers(
            "(and (can-replace ?p1 ?p2) (salary ?p1 ?s1) (salary ?p2 ?s2) \
                  (lisp-value < ?s1 ?s2))"
        ),
        [
            "(and (can-replace (Fect Cy D) (Hacker Alyssa P)) (salary (Fect Cy D) 35000) (salary (Hacker Alyssa P) 40000) (lisp-value < 35000 40000))",
            "(and (can-replace (Aull DeWitt) (Warbucks Oliver)) (salary (Aull DeWitt) 25000) (salary (Warbucks Oliver) 150000) (lisp-value < 25000 150000))",
        ]
    );
}
