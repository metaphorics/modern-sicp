// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.29: memoization changes the
//! number of thunk evaluations when the same demand repeats.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_2::{LazyEngine, LazyExpr, Mode};

fn forces(mode: Mode) -> usize {
    let thunk = LazyExpr::Thunk(1, Box::new(LazyExpr::Emit("compute".to_owned())));
    let program = LazyExpr::Let(
        "x".to_owned(),
        Box::new(thunk),
        Box::new(LazyExpr::Add(
            Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("x".to_owned())))),
            Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("x".to_owned())))),
        )),
    );
    LazyEngine::new(mode).run(&program).effects.len()
}

#[test]
fn ex_4_29() {
    assert_eq!(forces(Mode::Recompute), 2);
    assert_eq!(forces(Mode::Memo), 1);
}
