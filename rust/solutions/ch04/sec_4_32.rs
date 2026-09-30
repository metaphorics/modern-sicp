// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.32: lazy pairs can be built
//! and forced selectively.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_2::{LazyEngine, LazyExpr, LazyVal, Mode, PrimOp};

fn pair(first: LazyExpr, second: LazyExpr) -> LazyExpr {
    LazyExpr::Pair(Box::new(first), Box::new(second))
}

#[test]
fn ex_4_32() {
    let list = pair(LazyExpr::Int(1), pair(LazyExpr::Int(2), LazyExpr::Empty));
    let first = LazyExpr::Apply(
        Box::new(LazyExpr::Quote(LazyVal::Prim(PrimOp::First))),
        vec![list],
    );
    assert_eq!(LazyEngine::new(Mode::Memo).run(&first).value, Some(1));
}

#[test]
fn ex_4_32a() {
    let tree = LazyExpr::Let(
        "tree".to_owned(),
        Box::new(pair(
            LazyExpr::Thunk(1, Box::new(LazyExpr::Emit("root-left".to_owned()))),
            pair(
                LazyExpr::Thunk(2, Box::new(LazyExpr::Emit("root-right".to_owned()))),
                LazyExpr::Empty,
            ),
        )),
        Box::new(LazyExpr::Force(Box::new(LazyExpr::Apply(
            Box::new(LazyExpr::Quote(LazyVal::Prim(PrimOp::First))),
            vec![LazyExpr::Var("tree".to_owned())],
        )))),
    );
    let outcome = LazyEngine::new(Mode::Memo).run(&tree);
    assert_eq!(outcome.effects, vec!["root-left"]);
    assert_eq!(outcome.value, Some(0));
}
