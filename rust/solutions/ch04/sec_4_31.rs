// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.31: parameter declarations
//! choose strict, recomputing, or memoized demand.

/// Shared typed support for this exercise.
pub mod support;

use ch04::sec_4_2::{LazyEngine, LazyExpr, Mode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParamMode {
    Strict,
    Lazy,
    LazyMemo,
}

fn argument(mode: ParamMode, id: usize, effect: &'static str) -> LazyExpr {
    let thunk = LazyExpr::Thunk(id, Box::new(LazyExpr::Emit(effect.to_owned())));
    match mode {
        ParamMode::Strict => LazyExpr::Force(Box::new(thunk)),
        ParamMode::Lazy | ParamMode::LazyMemo => thunk,
    }
}

fn demand_twice(argument: LazyExpr) -> LazyExpr {
    LazyExpr::Let(
        "x".to_owned(),
        Box::new(argument),
        Box::new(LazyExpr::Add(
            Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("x".to_owned())))),
            Box::new(LazyExpr::Force(Box::new(LazyExpr::Var("x".to_owned())))),
        )),
    )
}

#[test]
fn ex_4_31() {
    let strict = demand_twice(argument(ParamMode::Strict, 1, "strict"));
    assert_eq!(
        LazyEngine::new(Mode::Memo).run(&strict).effects,
        vec!["strict"]
    );

    let lazy = demand_twice(argument(ParamMode::Lazy, 2, "lazy"));
    assert_eq!(
        LazyEngine::new(Mode::Recompute).run(&lazy).effects,
        vec!["lazy", "lazy"]
    );

    let memo = demand_twice(argument(ParamMode::LazyMemo, 3, "memo"));
    assert_eq!(LazyEngine::new(Mode::Memo).run(&memo).effects, vec!["memo"]);
}
