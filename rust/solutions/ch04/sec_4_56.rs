// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.56: three compound queries. Each
//! `and` runs its conjuncts in series over the frame stream, and the
//! `lisp-value` filter applies the host `<` to instantiated salaries.

use ch04::sec_4_4::microshaft;

mod ex_4_56 {
    //! Exercise 4.56: supervised-with-address, salaries below Ben's, and
    //! supervisees whose boss is outside the computer division.

    use super::*;

    /// Runs `query` over a fresh Microshaft engine.
    pub fn answers(query: &str) -> Vec<String> {
        microshaft().answers(query)
    }
}

#[test]
fn ex_4_56() {
    // a. Ben's supervisees, with their addresses.
    assert_eq!(
        ex_4_56::answers("(and (supervisor ?name (Bitdiddle Ben)) (address ?name ?where))"),
        [
            "(and (supervisor (Hacker Alyssa P) (Bitdiddle Ben)) (address (Hacker Alyssa P) (Cambridge (Mass Ave) 78)))",
            "(and (supervisor (Fect Cy D) (Bitdiddle Ben)) (address (Fect Cy D) (Cambridge (Ames Street) 3)))",
            "(and (supervisor (Tweakit Lem E) (Bitdiddle Ben)) (address (Tweakit Lem E) (Boston (Bay State Road) 22)))",
        ]
    );
    // b. everyone earning less than Ben, with both salaries.
    assert_eq!(
        ex_4_56::answers(
            "(and (salary (Bitdiddle Ben) ?ben-salary) (salary ?name ?amount) \
                  (lisp-value < ?amount ?ben-salary))"
        ),
        [
            "(and (salary (Bitdiddle Ben) 60000) (salary (Hacker Alyssa P) 40000) (lisp-value < 40000 60000))",
            "(and (salary (Bitdiddle Ben) 60000) (salary (Fect Cy D) 35000) (lisp-value < 35000 60000))",
            "(and (salary (Bitdiddle Ben) 60000) (salary (Tweakit Lem E) 25000) (lisp-value < 25000 60000))",
            "(and (salary (Bitdiddle Ben) 60000) (salary (Reasoner Louis) 30000) (lisp-value < 30000 60000))",
            "(and (salary (Bitdiddle Ben) 60000) (salary (Cratchet Robert) 18000) (lisp-value < 18000 60000))",
            "(and (salary (Bitdiddle Ben) 60000) (salary (Aull DeWitt) 25000) (lisp-value < 25000 60000))",
        ]
    );
    // c. everyone supervised by someone outside the computer division,
    // with the supervisor's name and job. The driver instantiates the
    // whole query, `not` clause included.
    assert_eq!(
        ex_4_56::answers(
            "(and (supervisor ?person ?boss) (job ?boss ?job) \
                  (not (job ?boss (computer . ?type))))"
        ),
        [
            "(and (supervisor (Bitdiddle Ben) (Warbucks Oliver)) (job (Warbucks Oliver) (administration big wheel)) (not (job (Warbucks Oliver) (computer . ?type))))",
            "(and (supervisor (Scrooge Eben) (Warbucks Oliver)) (job (Warbucks Oliver) (administration big wheel)) (not (job (Warbucks Oliver) (computer . ?type))))",
            "(and (supervisor (Cratchet Robert) (Scrooge Eben)) (job (Scrooge Eben) (accounting chief accountant)) (not (job (Scrooge Eben) (computer . ?type))))",
            "(and (supervisor (Aull DeWitt) (Warbucks Oliver)) (job (Warbucks Oliver) (administration big wheel)) (not (job (Warbucks Oliver) (computer . ?type))))",
        ]
    );
}
