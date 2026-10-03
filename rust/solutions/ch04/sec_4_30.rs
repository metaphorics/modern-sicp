// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.30: sequence evaluation forces
//! each delayed expression only when its position is reached.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_2::{LazyEngine, LazyExpr, Mode};

#[test]
fn ex_4_30() {
    let program = LazyExpr::Let(
        "_first".to_owned(),
        Box::new(LazyExpr::Force(Box::new(LazyExpr::Thunk(
            1,
            Box::new(LazyExpr::Emit("first".to_owned())),
        )))),
        Box::new(LazyExpr::Force(Box::new(LazyExpr::Thunk(
            2,
            Box::new(LazyExpr::Emit("second".to_owned())),
        )))),
    );
    let outcome = LazyEngine::new(Mode::Recompute).run(&program);
    assert_eq!(outcome.effects, vec!["first", "second"]);
}
