// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.33: quoted list data becomes a
//! lazy pair value rather than eager source text.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_2::{LazyEngine, LazyExpr, LazyVal, Mode};

#[test]
fn ex_4_33() {
    let quoted = LazyVal::PairVal(
        Box::new(LazyVal::Now(1)),
        Box::new(LazyVal::PairVal(
            Box::new(LazyVal::Now(2)),
            Box::new(LazyVal::EmptyVal),
        )),
    );
    let outcome = LazyEngine::new(Mode::Memo).run(&LazyExpr::Quote(quoted));
    assert!(outcome.rendered.iter().any(|line| line.contains('1')));
    assert!(outcome.rendered.iter().any(|line| line.contains('2')));
}
