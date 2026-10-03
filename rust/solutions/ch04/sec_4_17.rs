// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.17: the scan-out rewrite adds
//! one explicit frame around the procedure body.

/// Shared typed support for this exercise.
pub mod support;

use support::{BindingValue, Env};

fn frames_with_scan_out(definition_count: usize) -> usize {
    let outer = Env::root();
    let scanned = outer.child();
    for index in 0..definition_count {
        let value = i64::try_from(index).expect("small definition index") * 2;
        scanned.define(format!("u{index}"), BindingValue::Int(value));
    }
    scanned.depth()
}

fn frames_without_scan_out(definition_count: usize) -> usize {
    let outer = Env::root();
    for index in 0..definition_count {
        let value = i64::try_from(index).expect("small definition index") * 2;
        outer.define(format!("u{index}"), BindingValue::Int(value));
    }
    outer.depth()
}

#[test]
fn ex_4_17() {
    assert_eq!(frames_with_scan_out(2), frames_without_scan_out(2) + 1);
}
