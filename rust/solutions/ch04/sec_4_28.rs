// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.28: the operator is demanded
//! before its operands in a lazy application.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_2::{LazyEngine, LazyExpr, Mode};

#[test]
fn ex_4_28() {
    let program = LazyExpr::Apply(
        Box::new(LazyExpr::Force(Box::new(LazyExpr::Thunk(
            1,
            Box::new(LazyExpr::Let(
                "_seen".to_owned(),
                Box::new(LazyExpr::Emit("operator".to_owned())),
                Box::new(LazyExpr::Lambda(
                    vec!["x".to_owned()],
                    Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("x".to_owned())))),
                )),
            )),
        )))),
        vec![LazyExpr::Emit("operand".to_owned())],
    );
    let outcome = LazyEngine::new(Mode::Memo).run(&program);
    assert_eq!(outcome.effects, vec!["operator", "operand"]);
}
