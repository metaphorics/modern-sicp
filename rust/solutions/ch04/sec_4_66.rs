// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.66: Ben's accumulation over
//! frames. The scheme maps `qeval`'s frames to the designated variable's
//! values and folds them; over the duplicated `wheel` answers it
//! quadruples Warbucks's salary, which is what Ben groans about, and
//! distinct-ing the answers salvages the number.

use ch04::sec_4_4::{Engine, Frame, microshaft};
use std::collections::HashSet;

mod ex_4_66 {
    //! Exercise 4.66: sum over a query's frames, and the distinct
    //! salvage.

    use super::*;

    /// One Microshaft engine carrying the `wheel` rule of 4.4.1.
    pub fn engine() -> Engine {
        let engine = microshaft();
        engine.load(&["(rule (wheel ?person) \
             (and (supervisor ?middle-manager ?person) \
             (supervisor ?x ?middle-manager)))"]);
        engine
    }

    /// Ben's scheme: the sum of `variable` over every answer frame of
    /// `query`.
    pub fn sum(engine: &Engine, variable: &str, query: &str) -> i128 {
        let processed = ch04::sec_4_4::read_query(query);
        let var =
            ch04::sec_4_4::query_syntax_process(&sicp_runtime::read(variable).expect("parses"));
        let mut total = 0;
        for frame in &engine.query_frames(&processed) {
            total += instantiated_int(&var, &frame);
        }
        total
    }

    /// The salvage: the same sum over the distinct instantiated values.
    pub fn sum_distinct(engine: &Engine, variable: &str, query: &str) -> i128 {
        let processed = ch04::sec_4_4::read_query(query);
        let var =
            ch04::sec_4_4::query_syntax_process(&sicp_runtime::read(variable).expect("parses"));
        let mut seen = HashSet::new();
        let mut total = 0;
        for frame in &engine.query_frames(&processed) {
            let value = ch04::sec_4_4::instantiate_query(&var, &frame);
            if seen.insert(sicp_runtime::print_value(&value)) {
                total += match value {
                    sicp_runtime::Value::Int(n) => n,
                    other => panic!("not a number: {other}"),
                };
            }
        }
        total
    }

    fn instantiated_int(var: &sicp_runtime::Value, frame: &Frame) -> i128 {
        match ch04::sec_4_4::instantiate_query(var, frame) {
            sicp_runtime::Value::Int(n) => n,
            other => panic!("not a number: {other}"),
        }
    }
}

#[test]
fn ex_4_66() {
    // The programmer query works: two programmers, 40000 + 35000.
    assert_eq!(
        ex_4_66::sum(
            &ex_4_66::engine(),
            "?amount",
            "(and (job ?x (computer programmer)) (salary ?x ?amount))"
        ),
        75_000
    );
    // Over the wheel answers Ben's scheme breaks: Warbucks's salary
    // counts four times (once per derivation route), 60000 + 4x150000.
    assert_eq!(
        ex_4_66::sum(
            &ex_4_66::engine(),
            "?salary",
            "(and (wheel ?who) (salary ?who ?salary))"
        ),
        660_000
    );
    // The salvage: distinct answers first, 60000 + 150000.
    assert_eq!(
        ex_4_66::sum_distinct(
            &ex_4_66::engine(),
            "?salary",
            "(and (wheel ?who) (salary ?who ?salary))"
        ),
        210_000
    );
}
