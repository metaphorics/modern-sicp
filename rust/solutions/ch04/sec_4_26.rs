// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.26: `unless` as syntax avoids
//! eager operands, while the procedure version needs delayed ones.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_2::{LazyEngine, LazyExpr, Mode};

fn special_unless(test: bool, then: LazyExpr, otherwise: LazyExpr) -> LazyExpr {
    LazyExpr::If(
        Box::new(LazyExpr::Int(i64::from(test))),
        Box::new(otherwise),
        Box::new(then),
    )
}

fn procedure_unless(test: bool, then: LazyExpr, otherwise: LazyExpr) -> LazyExpr {
    LazyExpr::Apply(
        Box::new(LazyExpr::Lambda(
            vec!["test".to_owned(), "then".to_owned(), "otherwise".to_owned()],
            Box::new(LazyExpr::If(
                Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("test".to_owned())))),
                Box::new(LazyExpr::Force(Box::new(LazyExpr::Var(
                    "otherwise".to_owned(),
                )))),
                Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("then".to_owned())))),
            )),
        )),
        vec![LazyExpr::Int(i64::from(test)), then, otherwise],
    )
}

#[test]
fn ex_4_26() {
    let special = special_unless(
        false,
        LazyExpr::Emit("taken".to_owned()),
        LazyExpr::Emit("skipped".to_owned()),
    );
    let procedure = procedure_unless(
        false,
        LazyExpr::Emit("taken".to_owned()),
        LazyExpr::Emit("skipped".to_owned()),
    );
    assert_eq!(
        LazyEngine::new(Mode::Memo).run(&special).effects,
        vec!["taken"]
    );
    assert_eq!(
        LazyEngine::new(Mode::Memo).run(&procedure).effects,
        vec!["taken"]
    );
}
