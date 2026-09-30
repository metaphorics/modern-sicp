// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.27: a memoized thunk computes
//! once, while a recomputing thunk runs its effect every demand.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_2::{LazyEngine, LazyExpr, Mode};

fn count_probe(mode: Mode) -> usize {
    let thunk = LazyExpr::Thunk(1, Box::new(LazyExpr::Emit("compute".to_owned())));
    let program = LazyExpr::Let(
        "value".to_owned(),
        Box::new(thunk),
        Box::new(LazyExpr::Add(
            Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("value".to_owned())))),
            Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("value".to_owned())))),
        )),
    );
    LazyEngine::new(mode).run(&program).effects.len()
}

#[test]
fn ex_4_27() {
    assert_eq!(count_probe(Mode::Recompute), 2);
    assert_eq!(count_probe(Mode::Memo), 1);
}
