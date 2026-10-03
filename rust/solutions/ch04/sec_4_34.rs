// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.34: lazy pair printing renders
//! only the demanded part of the structure.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_2::{LazyEngine, LazyExpr, Mode};

#[test]
fn ex_4_34() {
    let lazy_pair = LazyExpr::Pair(
        Box::new(LazyExpr::Emit("first".to_owned())),
        Box::new(LazyExpr::Thunk(
            1,
            Box::new(LazyExpr::Emit("second".to_owned())),
        )),
    );
    let outcome = LazyEngine::new(Mode::Recompute).run(&lazy_pair);
    assert_eq!(outcome.effects, vec!["first"]);
    assert!(outcome.rendered.iter().any(|line| line.contains("first")));
}
